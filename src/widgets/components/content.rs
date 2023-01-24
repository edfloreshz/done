use crate::application::plugin::Plugin;
use crate::widgets::components::preferences::Preferences;
use crate::widgets::components::task_entry::{
	TaskEntryComponent, TaskEntryComponentInput, TaskEntryComponentOutput,
};
use crate::widgets::factory::list::ListFactoryModel;
use crate::widgets::factory::task::{
	TaskFactoryInit, TaskFactoryInput, TaskFactoryModel,
};
use crate::widgets::factory::task_details::{
	TaskDetailsFactoryInit, TaskDetailsFactoryInput, TaskDetailsFactoryModel,
};
use libset::format::FileFormat;
use libset::project::Project;
use proto_rust::provider::{List, Task};

use relm4::component::{
	AsyncComponent, AsyncComponentParts, AsyncComponentSender,
};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::factory::DynamicIndex;
use relm4::{
	adw, gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
	Controller,
};
use relm4::{Component, ComponentController, RelmWidgetExt};

use super::smart_lists::{
	AllComponentModel, Next7DaysComponentModel, SmartList, StarredComponentModel,
	TodayComponentModel,
};

pub struct ContentComponentModel {
	task_factory: AsyncFactoryVecDeque<TaskFactoryModel>,
	task_details_factory: AsyncFactoryVecDeque<TaskDetailsFactoryModel>,
	task_entry: Controller<TaskEntryComponent>,
	all: Controller<AllComponentModel>,
	today: Controller<TodayComponentModel>,
	starred: Controller<StarredComponentModel>,
	next7days: Controller<Next7DaysComponentModel>,
	plugin: Option<Plugin>,
	parent_list: Option<List>,
	selected_smart_list: Option<SmartList>,
	compact: bool,
	selected_task: Option<Task>,
	show_task_details: bool,
}

#[derive(Debug)]
pub enum ContentComponentInput {
	AddTask(Task),
	RemoveTask(DynamicIndex),
	UpdateTask(Option<DynamicIndex>, Task),
	TaskListSelected(ListFactoryModel),
	SelectSmartList(SmartList),
	RevealTaskDetails(DynamicIndex, Task),
	ToggleCompact(bool),
	DisablePlugin,
	HideFlap,
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
			set_width_request: 400,
			#[name(flap)]
			adw::Flap {
				set_modal: true,
				set_locked: true,
				#[watch]
				set_reveal_flap: model.show_task_details,
				#[wrap(Some)]
				set_content = &gtk::Box {
					set_width_request: 300,
					set_margin_all: 10,
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
				#[wrap(Some)]
				#[local_ref]
				set_flap = flap_container -> gtk::Box {
					set_width_request: 300,
					set_css_classes: &["background"],

				},
				#[wrap(Some)]
				set_separator = &gtk::Separator {
					set_orientation: gtk::Orientation::Vertical,
				},
				set_flap_position: gtk::PackType::End,
			}
		}
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let plugin = None;
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
					.margin_top(5)
					.margin_bottom(5)
					.margin_start(5)
					.margin_end(5)
					.build(),
				sender.input_sender(),
			),
			task_details_factory: AsyncFactoryVecDeque::new(
				gtk::Box::default(),
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
			plugin,
			parent_list: None,
			selected_smart_list: None,
			compact,
			selected_task: None,
			show_task_details: false,
		};
		let list_box = model.task_factory.widget();
		let flap_container = model.task_details_factory.widget();

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
			ContentComponentInput::RevealTaskDetails(index, task) => {
				self.show_task_details = true;
				if self.selected_task.is_none()
					|| self.selected_task.as_ref().unwrap().id != task.id
				{
					self.selected_task = Some(task.clone());
					let mut guard = self.task_details_factory.guard();
					guard.clear();
					guard.push_back(TaskDetailsFactoryInit::new(task, index));
				}
			},
			ContentComponentInput::HideFlap => {
				self.show_task_details = false;
				if let Some(list) = self.parent_list.clone() {
					if let Some(plugin) = self.plugin.clone() {
						sender.input(ContentComponentInput::TaskListSelected(
							ListFactoryModel::new(list, plugin),
						))
					}
				}
			},
			ContentComponentInput::AddTask(task) => {
				if let Ok(mut client) = self.plugin.as_mut().unwrap().connect().await {
					match client.create_task(task.clone()).await {
						Ok(response) => {
							let response = response.into_inner();
							if response.successful && response.task.is_some() {
								let task = response.task.unwrap();
								self.task_factory.guard().push_back(TaskFactoryInit::new(
									task,
									self.parent_list.as_ref().unwrap().clone(),
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
				} //TODO: Error handle
			},
			ContentComponentInput::RemoveTask(index) => {
				let mut guard = self.task_factory.guard();
				let task = guard.get(index.current_index()).unwrap().clone();
				if let Ok(mut client) = self.plugin.as_mut().unwrap().connect().await {
					match client.delete_task(task.clone().task.id).await {
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
				}
			},
			ContentComponentInput::UpdateTask(index, task) => {
				if let Ok(mut client) = self.plugin.as_mut().unwrap().connect().await {
					match client.update_task(task).await {
						Ok(response) => {
							let response = response.into_inner();
							if response.successful {
								if let Some(index) = index {
									self.task_details_factory.send(
										index.current_index(),
										TaskDetailsFactoryInput::Notify(response.message),
									);
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
				}
			},
			ContentComponentInput::TaskListSelected(model) => {
				self.parent_list = Some(model.list.clone());
				self.selected_smart_list = None;
				self
					.task_entry
					.sender()
					.send(TaskEntryComponentInput::SetParentList(
						self.parent_list.clone(),
					))
					.unwrap_or_default();

				self.plugin = Some(model.plugin.clone());
				if let Ok(mut client) = model.plugin.connect().await {
					let mut guard = self.task_factory.guard();
					guard.clear();

					let (tx, mut rx) = relm4::tokio::sync::mpsc::channel(100);
					relm4::spawn(async move {
						let mut stream = client
							.read_tasks_from_list(model.list.id)
							.await
							.unwrap()
							.into_inner();
						while let Some(response) = stream.message().await.unwrap() {
							tx.send(response).await.unwrap()
						}
					});

					while let Some(response) = rx.recv().await {
						if response.successful {
							guard.push_back(TaskFactoryInit::new(
								response.task.unwrap(),
								self.parent_list.as_ref().unwrap().clone(),
								self.compact,
							));
						}
					}
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
