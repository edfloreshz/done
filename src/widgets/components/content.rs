use crate::application::plugin::Plugin;
use crate::widgets::components::new_task::{
	NewTask, NewTaskEvent, NewTaskOutput,
};
use crate::widgets::factory::list::ListData;
use crate::widgets::factory::task::{TaskData, TaskInit};
use proto_rust::provider::provider_client::ProviderClient;
use proto_rust::provider::{List, Task};

use relm4::component::{
	AsyncComponent, AsyncComponentParts, AsyncComponentSender,
};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::factory::DynamicIndex;
use relm4::{
	gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
	view, Controller,
};
use relm4::{Component, ComponentController};
use std::str::FromStr;
use tonic::transport::Channel;

use super::smart_lists::{
	AllModel, Next7DaysModel, SmartList, StarredModel, TodayModel,
};

pub struct ContentModel {
	task_factory: AsyncFactoryVecDeque<TaskData>,
	task_entry: Controller<NewTask>,
	all: Controller<AllModel>,
	today: Controller<TodayModel>,
	starred: Controller<StarredModel>,
	next7days: Controller<Next7DaysModel>,
	service: Option<ProviderClient<Channel>>,
	plugin: Option<Plugin>,
	parent_list: Option<List>,
	selected_smart_list: Option<SmartList>,
}

#[derive(Debug)]
pub enum ContentInput {
	AddTask(Task),
	RemoveTask(DynamicIndex),
	UpdateTask(Option<DynamicIndex>, Task),
	TaskListSelected(ListData),
	SelectSmartList(SmartList),
	DisablePlugin,
}

#[derive(Debug)]
pub enum ContentOutput {
	Notify(String),
}

#[relm4::component(pub async)]
impl AsyncComponent for ContentModel {
	type Input = ContentInput;
	type Output = ContentOutput;
	type Init = ();
	type Widgets = ContentWidgets;
	type CommandOutput = ();

	view! {
		#[root]
		gtk::Stack {
			set_vexpand: true,
			set_transition_duration: 250,
			set_transition_type: gtk::StackTransitionType::Crossfade,
			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				#[name(task_container)]
				gtk::Stack {
					set_transition_duration: 250,
					set_transition_type: gtk::StackTransitionType::Crossfade,
					gtk::ScrolledWindow {
						#[watch]
						set_visible: model.parent_list.is_some(),
						set_vexpand: true,
						set_hexpand: true,
						set_child: Some(&list_box),
					},
					gtk::ScrolledWindow {
						#[watch]
						set_visible: model.selected_smart_list.is_some() && model.selected_smart_list.as_ref().unwrap() == &SmartList::All,
						set_vexpand: true,
						set_hexpand: true,
						set_child: Some(model.all.widget())
					},
					gtk::ScrolledWindow {
						#[watch]
						set_visible: model.selected_smart_list.is_some() && model.selected_smart_list.as_ref().unwrap() == &SmartList::Today,
						set_vexpand: true,
						set_hexpand: true,
						set_child: Some(model.today.widget())
					},
					gtk::ScrolledWindow {
						#[watch]
						set_visible: model.selected_smart_list.is_some() && model.selected_smart_list.as_ref().unwrap() == &SmartList::Starred,
						set_vexpand: true,
						set_hexpand: true,
						set_child: Some(model.starred.widget())
					},
					gtk::ScrolledWindow {
						#[watch]
						set_visible: model.selected_smart_list.is_some() && model.selected_smart_list.as_ref().unwrap() == &SmartList::Next7Days,
						set_vexpand: true,
						set_hexpand: true,
						set_child: Some(model.next7days.widget())
					},
				},
				append: model.task_entry.widget()
			},
		}
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		view! {
			list_box = &gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
			}
		}
		let plugin = None;
		let service = None;

		let all = AllModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {});
		let today = TodayModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {});
		let starred = StarredModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {});
		let next7days = Next7DaysModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {});

		let model = ContentModel {
			task_factory: AsyncFactoryVecDeque::new(
				list_box.clone(),
				sender.input_sender(),
			),
			task_entry: NewTask::builder().launch(None).forward(
				sender.input_sender(),
				|message| match message {
					NewTaskOutput::AddTask(task) => ContentInput::AddTask(task),
				},
			),
			all,
			today,
			starred,
			next7days,
			service,
			plugin,
			parent_list: None,
			selected_smart_list: None,
		};
		let widgets = view_output!();
		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			ContentInput::AddTask(task) => {
				match self
					.service
					.as_mut()
					.unwrap()
					.create_task(task.clone())
					.await
				{
					Ok(response) => {
						let response = response.into_inner();
						if response.successful && response.task.is_some() {
							let task = response.task.unwrap();
							self.task_factory.guard().push_back(TaskInit::new(
								task.id,
								self.service.clone().unwrap(),
							));
						}
						sender
							.output(ContentOutput::Notify(response.message))
							.unwrap_or_default();
					},
					Err(err) => {
						sender
							.output(ContentOutput::Notify(err.to_string()))
							.unwrap_or_default();
					},
				}
			},
			ContentInput::RemoveTask(index) => {
				let mut guard = self.task_factory.guard();
				let task = guard.get(index.current_index()).unwrap();
				match self
					.service
					.as_mut()
					.unwrap()
					.delete_task(task.clone().task.id)
					.await
				{
					Ok(response) => {
						let response = response.into_inner();
						if response.successful {
							guard.remove(index.current_index());
						}
						sender
							.output(ContentOutput::Notify(response.message))
							.unwrap_or_default();
					},
					Err(err) => {
						sender
							.output(ContentOutput::Notify(err.to_string()))
							.unwrap_or_default();
					},
				}
			},
			ContentInput::UpdateTask(index, task) => {
				match self.service.as_mut().unwrap().update_task(task).await {
					Ok(response) => {
						let response = response.into_inner();
						if response.successful {
							if let Some(index) = index {
								if self.parent_list.as_ref().unwrap().provider == "starred" {
									self.task_factory.guard().remove(index.current_index());
								}
							}
						} else {
							sender
								.output(ContentOutput::Notify(response.message))
								.unwrap_or_default();
						}
					},
					Err(err) => {
						sender
							.output(ContentOutput::Notify(err.to_string()))
							.unwrap_or_default();
					},
				}
			},
			ContentInput::TaskListSelected(list) => {
				self.parent_list = Some(list.list.clone());
				self.selected_smart_list = None;
				self
					.task_entry
					.sender()
					.send(NewTaskEvent::SetParentList(self.parent_list.clone()))
					.unwrap_or_default();
				self.plugin = Some(Plugin::from_str(&list.list.provider).unwrap());
				self.service = Some(self.plugin.unwrap().connect().await.unwrap());

				self.task_factory.guard().clear();

				for task in list.tasks {
					self
						.task_factory
						.guard()
						.push_back(TaskInit::new(task, self.service.clone().unwrap()));
				}
			},
			ContentInput::DisablePlugin => {
				self.parent_list = None;
			},
			ContentInput::SelectSmartList(list) => {
				self.selected_smart_list = Some(list);
				self.parent_list = None;
			},
		}
	}
}
