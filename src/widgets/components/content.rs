use crate::application::plugin::Plugin;
use crate::widgets::components::preferences::Preferences;
use crate::widgets::components::task_entry::{
	TaskEntryComponent, TaskEntryComponentInput, TaskEntryComponentOutput,
};
use crate::widgets::factory::list::ListFactoryModel;
use crate::widgets::factory::task::{
	TaskFactoryInit, TaskFactoryInput, TaskFactoryModel,
};
use libset::format::FileFormat;
use libset::project::Project;
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
	Controller,
};
use relm4::{Component, ComponentController, RelmWidgetExt};
use tonic::transport::Channel;

use super::smart_lists::{
	AllComponentModel, Next7DaysComponentModel, SmartList, StarredComponentModel,
	TodayComponentModel,
};

pub struct ContentComponentModel {
	task_factory: AsyncFactoryVecDeque<TaskFactoryModel>,
	task_entry: Controller<TaskEntryComponent>,
	all: Controller<AllComponentModel>,
	today: Controller<TodayComponentModel>,
	starred: Controller<StarredComponentModel>,
	next7days: Controller<Next7DaysComponentModel>,
	service: Option<ProviderClient<Channel>>,
	plugin: Option<Plugin>,
	parent_list: Option<List>,
	selected_smart_list: Option<SmartList>,
	compact: bool,
}

#[derive(Debug)]
pub enum ContentComponentInput {
	AddTask(Task),
	RemoveTask(DynamicIndex),
	UpdateTask(Option<DynamicIndex>, Task),
	TaskListSelected(ListFactoryModel),
	SelectSmartList(SmartList),
	ToggleCompact(bool),
	DisablePlugin,
}

#[derive(Debug)]
pub enum ContentComponentOutput {
	Notify(String, u32),
}

#[relm4::component(pub async)]
impl AsyncComponent for ContentComponentModel {
	type CommandOutput = ();
	type Input = ContentComponentInput;
	type Output = ContentComponentOutput;
	type Init = ();

	view! {
		#[root]
		gtk::Stack {
			set_vexpand: true,
			set_transition_duration: 250,
			set_transition_type: gtk::StackTransitionType::Crossfade,
			set_margin_all: 10,
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
						set_child: Some(list_box),
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
		let plugin = None;
		let service = None;
		let compact = Project::open("dev", "edfloreshz", "done")
			.unwrap()
			.get_file_as::<Preferences>("preferences", FileFormat::JSON)
			.unwrap()
			.compact;
		let all = AllComponentModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {});
		let today = TodayComponentModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {});
		let starred = StarredComponentModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {});
		let next7days = Next7DaysComponentModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {});

		let model = ContentComponentModel {
			task_factory: AsyncFactoryVecDeque::new(
				gtk::ListBox::builder()
					.show_separators(true)
					.css_classes(vec!["boxed-list".to_string()])
					.valign(gtk::Align::Start)
					.build(),
				sender.input_sender(),
			),
			task_entry: TaskEntryComponent::builder().launch(None).forward(
				sender.input_sender(),
				|message| match message {
					TaskEntryComponentOutput::AddTask(task) => {
						ContentComponentInput::AddTask(task)
					},
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
			compact,
		};
		let list_box = model.task_factory.widget();

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
			ContentComponentInput::AddTask(task) => {
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
							self.task_factory.guard().push_back(TaskFactoryInit::new(
								self.plugin.clone().unwrap(),
								task.id,
								self.compact,
							));
						}
						sender
							.output(ContentComponentOutput::Notify(response.message, 1))
							.unwrap_or_default();
					},
					Err(err) => {
						sender
							.output(ContentComponentOutput::Notify(err.to_string(), 2))
							.unwrap_or_default();
					},
				}
			},
			ContentComponentInput::RemoveTask(index) => {
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
							.output(ContentComponentOutput::Notify(response.message, 1))
							.unwrap_or_default();
					},
					Err(err) => {
						sender
							.output(ContentComponentOutput::Notify(err.to_string(), 2))
							.unwrap_or_default();
					},
				}
			},
			ContentComponentInput::UpdateTask(index, task) => {
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
								.output(ContentComponentOutput::Notify(response.message, 1))
								.unwrap_or_default();
						}
					},
					Err(err) => {
						sender
							.output(ContentComponentOutput::Notify(err.to_string(), 2))
							.unwrap_or_default();
					},
				}
			},
			ContentComponentInput::TaskListSelected(model) => {
				self.parent_list = Some(model.list.clone().unwrap());
				self.selected_smart_list = None;
				self
					.task_entry
					.sender()
					.send(TaskEntryComponentInput::SetParentList(
						self.parent_list.clone(),
					))
					.unwrap_or_default();
				self.plugin =
					Some(Plugin::get_by_id(&model.list.unwrap().provider).unwrap());
				self.service =
					Some(self.plugin.as_ref().unwrap().connect().await.unwrap());

				self.task_factory.guard().clear();

				for task in model.tasks {
					self.task_factory.guard().push_back(TaskFactoryInit::new(
						self.plugin.clone().unwrap(),
						task,
						self.compact,
					));
				}
			},
			ContentComponentInput::DisablePlugin => {
				self.parent_list = None;
			},
			ContentComponentInput::SelectSmartList(list) => {
				self.selected_smart_list = Some(list);
				self.parent_list = None;
			},
			ContentComponentInput::ToggleCompact(compact) => {
				let size = self.task_factory.len();
				for index in 0..size {
					self
						.task_factory
						.send(index, TaskFactoryInput::ToggleCompact(compact));
				}
			},
		}
	}
}
