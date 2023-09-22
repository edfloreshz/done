use crate::app::components::task_input::TaskInputOutput;
use crate::app::factories::details::factory::{
	TaskDetailsFactoryInit, TaskDetailsFactoryModel,
};
use crate::app::factories::task::{TaskInit, TaskModel};
use crate::app::models::sidebar_list::SidebarList;
use crate::fl;

use chrono::{DateTime, Utc};
use core_done::models::status::Status;
use core_done::models::task::Task;
use core_done::service::Service;
use futures::StreamExt;
use relm4::component::{
	AsyncComponent, AsyncComponentParts, AsyncComponentSender,
};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::gtk::traits::ButtonExt;
use relm4::prelude::DynamicIndex;
use relm4::{
	adw,
	adw::prelude::NavigationPageExt,
	gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
};
use relm4::{tokio, Component, ComponentController, Controller, RelmWidgetExt};
use relm4_icons::icon_name;

use super::task_input::{TaskInputInput, TaskInputModel};
use super::welcome::WelcomeComponent;

pub struct ContentModel {
	task_factory: AsyncFactoryVecDeque<TaskModel>,
	task_details_factory: AsyncFactoryVecDeque<TaskDetailsFactoryModel>,
	task_entry: Controller<TaskInputModel>,
	welcome: Controller<WelcomeComponent>,
	state: ContentState,
	service: Service,
	parent_list: Option<SidebarList>,
	selected_task: Option<Task>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContentState {
	Empty,
	AllDone,
	Loading,
	TasksLoaded,
	Details,
}

#[derive(Debug)]
pub enum ContentInput {
	AddTask(Task),
	RemoveTask(DynamicIndex),
	UpdateTask(Task),
	LoadTask(Task),
	SelectList(SidebarList, Service),
	ServiceDisabled(Service),
	LoadTasks(SidebarList, Service),
	RevealTaskDetails(Option<DynamicIndex>, Task),
	CleanTaskEntry,
	HideFlap,
	Refresh,
	Clean,
}

#[derive(Debug)]
pub enum ContentOutput {}

#[derive(Debug)]
pub enum TaskInput {
	SetCompleted(bool),
	Favorite(DynamicIndex),
	ModifyTitle(String),
	RevealTaskDetails(Option<DynamicIndex>),
}

#[derive(Debug)]
pub enum TaskOutput {
	Remove(DynamicIndex),
	UpdateTask(Option<DynamicIndex>, Task),
	RevealTaskDetails(Option<DynamicIndex>, Task),
}

#[relm4::component(pub async)]
impl AsyncComponent for ContentModel {
	type CommandOutput = ();
	type Input = ContentInput;
	type Output = ContentOutput;
	type Init = Option<Service>;

	view! {
		#[root]
		adw::ToolbarView {
			#[name = "content_header"]
			add_top_bar = &adw::HeaderBar {
				set_hexpand: true,
				set_css_classes: &["flat"],
				set_show_start_title_buttons: false,
				set_show_end_title_buttons: true,
				#[watch]
				set_title_widget: Some(&gtk::Label::new(
					Some("Tasks")
				)),
				pack_start: go_back_button = &gtk::Button {
					set_tooltip: fl!("back"),
					set_icon_name: icon_name::LEFT,
					set_visible: false,
				},
				pack_start = &gtk::Button {
					set_visible: false,
					set_tooltip: fl!("search"),
					set_icon_name: icon_name::LOUPE,
				},
			},
			#[name(overlay)]
			#[wrap(Some)]
			set_content = &adw::ToastOverlay {
				#[wrap(Some)]
				set_child = &gtk::Box {
					gtk::Box {
						#[watch]
						set_visible: model.parent_list.is_none(),
						append: model.welcome.widget()
					},
					adw::Clamp {
						gtk::Box {
							#[watch]
							set_visible: model.parent_list.is_some(),
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
											set_spacing: 10,
											gtk::Image {
												set_icon_name: Some(icon_name::SONAR),
												set_pixel_size: 64,
												set_margin_all: 10,
											},
											gtk::Label {
												set_css_classes: &["title-2"],
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
												set_spacing: 10,
												gtk::Image {
													set_icon_name: Some(icon_name::CHECK_ROUND_OUTLINE2),
													set_pixel_size: 64,
													set_margin_all: 10,
												},
												gtk::Label {
													set_css_classes: &["title-2"],
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
									#[name(split_view)]
									adw::NavigationView {
										add = &adw::NavigationPage {
											#[wrap(Some)]
											set_child = &gtk::Box {
												set_orientation: gtk::Orientation::Vertical,
												set_margin_all: 10,
												gtk::Box {
													#[watch]
													set_orientation: gtk::Orientation::Horizontal,
													gtk::Image {
														#[watch]
														set_visible: model.parent_list.as_ref().unwrap().smart(),
														#[watch]
														set_icon_name: model.parent_list.as_ref().unwrap().icon(),
														set_margin_start: 10,
													},
													gtk::Label {
														#[watch]
														set_visible: !model.parent_list.as_ref().unwrap().smart(),
														#[watch]
														set_text: model.parent_list.as_ref().unwrap().icon().unwrap_or_default(),
														set_margin_start: 10,
													},
													gtk::Label {
														set_css_classes: &["title-3"],
														set_halign: gtk::Align::Start,
														set_margin_start: 10,
														set_margin_end: 10,
														#[watch]
														set_text: model.parent_list.as_ref().unwrap().name().as_str()
													},
												},
												gtk::Label {
													#[watch]
													set_visible: !model.parent_list.as_ref().unwrap().description().is_empty(),
													set_css_classes: &["title-5"],
													set_halign: gtk::Align::Start,
													set_margin_bottom: 10,
													set_margin_start: 10,
													set_margin_end: 10,
													#[watch]
													set_text: model.parent_list.as_ref().unwrap().description().as_str()
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
										},
										add = &adw::NavigationPage {
											set_tag: Some("task-details-page"),
											set_child: Some(model.task_details_factory.widget()),
										},
									}
								}
							},
							gtk::Box {
								#[watch]
								set_visible: model.state == ContentState::TasksLoaded || model.state == ContentState::AllDone,
								set_margin_all: 5,
								append: model.task_entry.widget()
							}
						}
					},
				}
			},
		},
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
			welcome: WelcomeComponent::builder().launch(()).detach(),
			state: ContentState::Empty,
			service: Service::Smart,
			parent_list: None,
			selected_task: None,
		};

		let list_box = model.task_factory.widget();

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
			ContentInput::Clean => self.state = ContentState::Empty,
			ContentInput::Refresh => sender.input(ContentInput::SelectList(
				self.parent_list.as_ref().unwrap().clone(),
				self.service,
			)),
			ContentInput::LoadTask(task) => {
				if let SidebarList::Custom(parent) = &self.parent_list.as_ref().unwrap()
				{
					let mut guard = self.task_factory.guard();
					guard.push_back(TaskInit::new(task, parent.clone()));
					self.state = ContentState::TasksLoaded;
				}
			},
			ContentInput::AddTask(mut task) => {
				if let SidebarList::Custom(parent) = &self.parent_list.as_ref().unwrap()
				{
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
						},
						Err(err) => {
							tracing::error!("An error ocurred: {err}");
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
						},
						Err(err) => {
							tracing::error!("An error ocurred: {err}");
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
					},
					Err(err) => {
						tracing::error!("An error ocurred: {err}");
					},
				}
			},
			ContentInput::SelectList(list, service) => {
				self.state = ContentState::Loading;
				sender.input(ContentInput::LoadTasks(list, service));
			},
			ContentInput::LoadTasks(list, service) => {
				let mut guard = self.task_factory.guard();
				guard.clear();
				self.service = service;

				let mut service = service.get_service();
				if let Ok(tasks) = service.read_tasks().await {
					match &list {
						SidebarList::All => {
							self.parent_list = Some(SidebarList::All);
							for task in tasks {
								guard.push_back(TaskInit::new(
									task.clone(),
									service.read_list(task.parent).await.unwrap(),
								));
							}
							self.state = ContentState::TasksLoaded;
						},
						SidebarList::Today => {
							self.parent_list = Some(SidebarList::Today);
							for task in tasks.iter().filter(|task| {
								task.today
									|| task.due_date.is_some()
										&& task.due_date.unwrap().date_naive()
											== Utc::now().date_naive()
							}) {
								guard.push_back(TaskInit::new(
									task.clone(),
									service.read_list(task.parent.clone()).await.unwrap(),
								));
							}
							self.state = ContentState::TasksLoaded;
						},
						SidebarList::Starred => {
							self.parent_list = Some(SidebarList::Starred);
							for task in tasks.iter().filter(|task| task.favorite) {
								guard.push_back(TaskInit::new(
									task.clone(),
									service.read_list(task.parent.clone()).await.unwrap(),
								));
							}
							self.state = ContentState::TasksLoaded;
						},
						SidebarList::Next7Days => {
							self.parent_list = Some(SidebarList::Next7Days);
							for task in tasks.iter().filter(|task: &&Task| {
								task.due_date.is_some()
									&& is_within_next_7_days(task.due_date.unwrap())
							}) {
								guard.push_back(TaskInit::new(
									task.clone(),
									service.read_list(task.parent.clone()).await.unwrap(),
								));
							}
							self.state = ContentState::TasksLoaded;
						},
						SidebarList::Done => {
							self.parent_list = Some(SidebarList::Done);
							for task in tasks
								.iter()
								.filter(|task: &&Task| task.status == Status::Completed)
							{
								guard.push_back(TaskInit::new(
									task.clone(),
									service.read_list(task.parent.clone()).await.unwrap(),
								));
							}
							self.state = ContentState::TasksLoaded;
						},
						SidebarList::Custom(list) => {
							self.parent_list = Some(SidebarList::Custom(list.clone()));
							let sender_clone = sender.clone();
							let list_clone = list.clone();
							let mut service = self.service.get_service();
							if service.stream_support() {
								tokio::spawn(async move {
									match service.get_tasks(list_clone.id.clone()).await {
										Ok(mut stream) => {
											while let Some(task) = stream.next().await {
												sender_clone.input(ContentInput::LoadTask(task));
											}
										},
										Err(err) => tracing::error!("{err}"),
									}
								});
								self.state = ContentState::Loading;
							} else if let Ok(tasks) =
								service.read_tasks_from_list(list_clone.id.clone()).await
							{
								for task in tasks {
									guard.push_back(TaskInit::new(
										task.clone(),
										service.read_list(task.parent.clone()).await.unwrap(),
									));
								}
								self.state = ContentState::TasksLoaded;
							}
						},
					}
				}

				if guard.is_empty() && self.state != ContentState::Loading {
					self.state = ContentState::AllDone;
				}

				if list.smart() {
					self.state = ContentState::Empty;
				}

				self
					.task_entry
					.sender()
					.send(TaskInputInput::SetParentList(
						self.parent_list.as_ref().unwrap().clone(),
					))
					.unwrap();
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
			ContentInput::ServiceDisabled(service) => {
				if self.service == service {
					self.state = ContentState::Empty;
				}
			},
			ContentInput::CleanTaskEntry => self
				.task_entry
				.sender()
				.send(TaskInputInput::CleanTaskEntry)
				.unwrap(),
			ContentInput::HideFlap => sender.input(ContentInput::SelectList(
				self.parent_list.as_ref().unwrap().clone(),
				self.service,
			)),
		}
		self.update_view(widgets, sender)
	}
}

fn is_within_next_7_days(date: DateTime<Utc>) -> bool {
	let now = Utc::now();
	let next_7_days = now + chrono::Duration::days(7);
	date >= now && date <= next_7_days
}
