use crate::factories::details::model::{
	TaskDetailsFactoryInit, TaskDetailsFactoryModel,
};
use crate::factories::task::model::{TaskInit, TaskModel};
use crate::fl;
use crate::widgets::content::messages::{ContentInput, ContentOutput};
use crate::widgets::sidebar::model::SidebarList;
use crate::widgets::task_input::messages::{TaskInputInput, TaskInputOutput};
use crate::widgets::task_input::model::TaskInputModel;

use done_local_storage::models::task::Task;
use done_local_storage::service::Service;
use relm4::component::{
	AsyncComponent, AsyncComponentParts, AsyncComponentSender,
};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::{
	adw, gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
};
use relm4::{Component, ComponentController, Controller, RelmWidgetExt};

use super::helpers::select_task_list;

pub struct ContentModel {
	pub task_factory: AsyncFactoryVecDeque<TaskModel>,
	pub task_details_factory: AsyncFactoryVecDeque<TaskDetailsFactoryModel>,
	pub task_entry: Controller<TaskInputModel>,
	pub state: ContentState,
	pub service: Service,
	pub parent_list: SidebarList,
	pub selected_task: Option<Task>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContentState {
	Empty,
	AllDone,
	Loading,
	TasksLoaded,
	Details,
}

#[relm4::component(pub async)]
impl AsyncComponent for ContentModel {
	type CommandOutput = ();
	type Input = ContentInput;
	type Output = ContentOutput;
	type Init = Option<Service>;

	view! {
		#[root]
		gtk::Stack {
			set_vexpand: true,
			set_transition_duration: 250,
			set_transition_type: gtk::StackTransitionType::Crossfade,
			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				#[transition = "Crossfade"]
				append = match model.state {
					ContentState::Empty => {
						gtk::CenterBox {
							set_vexpand: true,
							set_hexpand: true,
							set_orientation: gtk::Orientation::Vertical,
							set_halign: gtk::Align::Center,
							set_valign: gtk::Align::Center,
							#[wrap(Some)]
							set_center_widget = &gtk::Box {
								set_orientation: gtk::Orientation::Vertical,
								set_margin_all: 24,
								set_spacing: 24,
								gtk::Picture {
									#[watch]
									set_resource: Some("/dev/edfloreshz/Done/icons/scalable/actions/empty.png"),
									set_margin_all: 70
								},
								gtk::Label {
									set_css_classes: &["title-3", "accent"],
									set_wrap: true,
									set_wrap_mode: gtk::pango::WrapMode::Word,
									set_justify: gtk::Justification::Center,
									#[watch]
									set_text: fl!("list-empty"),
								},
								gtk::Label {
									set_css_classes: &["body"],
									#[watch]
									set_text: fl!("instructions"),
									set_wrap: true,
									set_wrap_mode: gtk::pango::WrapMode::Word,
									set_justify: gtk::Justification::Center,
								},
							}
						}
					},
					ContentState::AllDone => {
						gtk::Box {
							set_orientation: gtk::Orientation::Vertical,
							gtk::CenterBox {
								#[watch]
								set_vexpand: true,
								set_hexpand: true,
								set_orientation: gtk::Orientation::Vertical,
								set_halign: gtk::Align::Center,
								set_valign: gtk::Align::Center,
								#[wrap(Some)]
								set_center_widget = &gtk::Box {
									set_orientation: gtk::Orientation::Vertical,
									set_margin_all: 24,
									set_spacing: 24,
									gtk::Picture {
										#[watch]
										set_resource: Some("/dev/edfloreshz/Done/icons/scalable/actions/checked.png"),
										set_margin_all: 70
									},
									gtk::Label {
										set_css_classes: &["title-3", "accent"],
										set_wrap: true,
										set_wrap_mode: gtk::pango::WrapMode::Word,
										set_justify: gtk::Justification::Center,
										#[watch]
										set_text: fl!("all-done"),
									},
									gtk::Label {
										set_css_classes: &["body"],
										#[watch]
										set_text: fl!("all-done-instructions"),
										set_wrap: true,
										set_wrap_mode: gtk::pango::WrapMode::Word,
										set_justify: gtk::Justification::Center,
									},
								}
							},
						}
					},
					ContentState::Loading => {
						gtk::CenterBox {
							set_orientation: gtk::Orientation::Vertical,
							#[name(spinner)]
							#[wrap(Some)]
							set_center_widget = &gtk::Spinner {
								start: ()
							}
						}
					},
					ContentState::TasksLoaded | ContentState::Details => {
						#[name(flap)]
						adw::Flap {
							set_modal: true,
							set_locked: true,
							#[watch]
							set_reveal_flap: model.state == ContentState::Details,
							#[wrap(Some)]
							set_content = &gtk::Box {
								set_width_request: 300,
								set_margin_all: 10,
								set_orientation: gtk::Orientation::Vertical,
								gtk::Box {
									#[watch]
									set_orientation: gtk::Orientation::Horizontal,
									gtk::Image {
										#[watch]
										set_visible: model.parent_list.smart(),
										#[watch]
										set_icon_name: model.parent_list.icon().as_deref(),
										set_margin_start: 10,
									},
									gtk::Label {
										#[watch]
										set_visible: !model.parent_list.smart(),
										#[watch]
										set_text: model.parent_list.icon().as_deref().unwrap_or_default(),
										set_margin_start: 10,
									},
									gtk::Label {
										set_css_classes: &["title-3"],
										set_halign: gtk::Align::Start,
										set_margin_start: 10,
										set_margin_end: 10,
										#[watch]
										set_text: model.parent_list.name().as_str()
									},
								},
								gtk::Label {
									#[watch]
									set_visible: !model.parent_list.description().is_empty(),
									set_css_classes: &["title-5"],
									set_halign: gtk::Align::Start,
									set_margin_bottom: 10,
									set_margin_start: 10,
									set_margin_end: 10,
									#[watch]
									set_text: model.parent_list.description().as_str()
								},
								#[name(task_container)]
								gtk::Stack {
									set_transition_duration: 250,
									set_transition_type: gtk::StackTransitionType::Crossfade,
									gtk::ScrolledWindow {
										#[watch]
										set_visible: model.state == ContentState::TasksLoaded || model.state == ContentState::Details,
										set_vexpand: true,
										set_hexpand: true,

										#[local_ref]
										list_box -> adw::PreferencesGroup {
											set_css_classes: &["boxed-list"],
											set_valign: gtk::Align::Fill,
											set_margin_all: 5,
										},
									},
								},
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
				},
				gtk::Box {
					#[watch]
					set_visible: model.state != ContentState::Details,
					set_margin_all: 5,
					append: model.task_entry.widget()
				}
			}
		}
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let model = ContentModel {
			task_factory: AsyncFactoryVecDeque::new(
				adw::PreferencesGroup::default(),
				sender.input_sender(),
			),
			task_details_factory: AsyncFactoryVecDeque::new(
				gtk::Box::default(),
				sender.input_sender(),
			),
			task_entry: TaskInputModel::builder()
				.launch(SidebarList::default())
				.forward(sender.input_sender(), |message| match message {
					TaskInputOutput::EnterCreationMode(task) => {
						ContentInput::RevealTaskDetails(None, task)
					},
					TaskInputOutput::AddTask(task) => ContentInput::AddTask(task),
				}),
			state: ContentState::Empty,
			service: Service::Smart,
			parent_list: SidebarList::default(),
			selected_task: None,
		};
		let list_box = model.task_factory.widget();
		let flap_container = model.task_details_factory.widget();

		let widgets = view_output!();

		AsyncComponentParts { model, widgets }
	}

	async fn update_with_view(
		&mut self,
		widgets: &mut Self::Widgets,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			ContentInput::Refresh => sender.input(ContentInput::SelectList(
				self.parent_list.clone(),
				self.service,
			)),
			ContentInput::AddTask(mut task) => {
				if let SidebarList::Custom(parent) = &self.parent_list {
					task.parent = parent.id.clone();
					let mut service = self.service.get_service();
					match service.create_task(task.clone()).await {
						Ok(_) => {
							self
								.task_factory
								.guard()
								.push_back(TaskInit::new(task.clone(), parent.clone()));
							self.state = ContentState::Details;
							sender.input(ContentInput::HideFlap);
							sender
								.output(ContentOutput::Notify(
									"Task added successfully".into(),
									1,
								))
								.unwrap();
						},
						Err(err) => {
							tracing::error!("An error ocurred: {}", err);
							sender
								.output(ContentOutput::Notify("Error adding task".into(), 2))
								.unwrap();
						},
					}
				}
			},
			ContentInput::RemoveTask(index) => {
				let mut guard = self.task_factory.guard();
				if let Some(task) = guard.get(index.current_index()) {
					let mut service = self.service.get_service();
					match service
						.delete_task(task.clone().task.parent, task.clone().task.id)
						.await
					{
						Ok(_) => {
							guard.remove(index.current_index());
							sender
								.output(ContentOutput::Notify(
									"Task removed successfully.".into(),
									1,
								))
								.unwrap_or_default();
						},
						Err(_) => {
							sender
								.output(ContentOutput::Notify("Error removing task.".into(), 2))
								.unwrap_or_default();
						},
					}
				}
			},
			ContentInput::UpdateTask(task) => {
				let mut service = self.service.get_service();
				match service.update_task(task).await {
					Ok(_) => {
						if self.state == ContentState::Details {
							sender.input(ContentInput::HideFlap);
						}
						sender.input(ContentInput::Refresh);
						sender
							.output(ContentOutput::Notify(
								"Task updated successfully".into(),
								1,
							))
							.unwrap_or_default()
					},
					Err(err) => {
						tracing::error!(
							"An error ocurred while updating this task: {}",
							err.to_string()
						);
						sender
							.output(ContentOutput::Notify(
								"An error ocurred while updating this task.".into(),
								2,
							))
							.unwrap_or_default();
					},
				}
			},
			ContentInput::SelectList(list, service) => {
				self.state = ContentState::Loading;
				sender.input(ContentInput::LoadTasks(list, service));
			},
			ContentInput::LoadTasks(list, service) => {
				if let Err(err) = select_task_list(self, list, service).await {
					tracing::error!("{err}");
				}
			},
			ContentInput::RevealTaskDetails(index, task) => {
				self.state = ContentState::Details;
				let mut guard = self.task_details_factory.guard();
				if let Some(task_index) = index {
					self.selected_task = Some(task.clone());
					guard.clear();
					guard.push_back(TaskDetailsFactoryInit::new(task, Some(task_index)));
				} else {
					guard.clear();
					guard.push_back(TaskDetailsFactoryInit::new(task, None));
				}
			},
			ContentInput::DisablePlugin => self.state = ContentState::Empty,
			ContentInput::CleanTaskEntry => self
				.task_entry
				.sender()
				.send(TaskInputInput::CleanTaskEntry)
				.unwrap(),
			ContentInput::HideFlap => sender.input(ContentInput::SelectList(
				self.parent_list.clone(),
				self.service,
			)),
		}
		self.update_view(widgets, sender)
	}
}
