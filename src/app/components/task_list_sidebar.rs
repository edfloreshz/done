use core_done::{models::list::List, service::Service};
use futures::StreamExt;
use relm4::{
	adw,
	component::{
		AsyncComponent, AsyncComponentController, AsyncComponentParts,
		AsyncController, SimpleAsyncComponent,
	},
	factory::AsyncFactoryVecDeque,
	gtk::{
		self,
		prelude::BoxExt,
		traits::{ButtonExt, GtkWindowExt, OrientableExt, WidgetExt},
	},
	prelude::DynamicIndex,
	tokio, AsyncComponentSender, Component, ComponentController, Controller,
	JoinHandle, RelmWidgetExt,
};
use relm4_icons::icon_name;

use crate::{
	app::{
		components::{
			list_dialog::ListDialogOutput, services_sidebar::ServicesSidebarOutput,
		},
		factories::task_list::{
			TaskListFactoryInit, TaskListFactoryModel, TaskListFactoryOutput,
		},
		models::sidebar_list::SidebarList,
		AboutAction, PreferencesAction, QuitAction, ShortcutsAction,
	},
	fl,
};

use super::{
	list_dialog::ListDialogComponent,
	services_sidebar::{ServicesSidebarInput, ServicesSidebarModel},
};

pub struct TaskListSidebarModel {
	service: Service,
	state: TaskListSidebarStatus,
	task_list_factory: AsyncFactoryVecDeque<TaskListFactoryModel>,
	list_entry: Controller<ListDialogComponent>,
	services_sidebar_controller: AsyncController<ServicesSidebarModel>,
	handle: Option<JoinHandle<()>>,
}

#[derive(Debug)]
pub enum TaskListSidebarInput {
	LoadTaskLists,
	OpenNewTaskListDialog,
	LoadTaskList(List),
	AddTaskListToSidebar(String),
	ServiceSelected(Service),
	ServiceDisabled(Service),
	SelectList(SidebarList),
	DeleteTaskList(DynamicIndex),
	SetStatus(TaskListSidebarStatus),
	ReloadSidebar(Service),
}

#[derive(Debug)]
pub enum TaskListSidebarOutput {
	SelectList(SidebarList, Service),
	ServiceDisabled(Service),
	CleanContent,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TaskListSidebarStatus {
	Empty,
	Loading,
	Loaded,
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for TaskListSidebarModel {
	type Input = TaskListSidebarInput;
	type Output = TaskListSidebarOutput;
	type Init = Service;

	menu! {
		primary_menu: {
			section! {
				keyboard_shortcuts => ShortcutsAction,
				about_done => AboutAction,
				preferences => PreferencesAction,
				quit => QuitAction,
			}
		}
	}

	view! {
		#[root]
		adw::ToolbarView {
			add_top_bar = &adw::HeaderBar {
				set_css_classes: &["flat"],
				set_show_start_title_buttons: false,
				set_show_back_button: true,
				set_title_widget: Some(&gtk::Label::new(Some("Lists"))),
				pack_start = &gtk::Button {
					set_tooltip: fl!("add-new-task-list"),
					set_icon_name: icon_name::PLUS,
					set_css_classes: &["flat", "image-button"],
					set_valign: gtk::Align::Center,
					connect_clicked => TaskListSidebarInput::OpenNewTaskListDialog
				},
				pack_end = &gtk::MenuButton {
					set_tooltip: fl!("menu"),
					set_valign: gtk::Align::Center,
					set_css_classes: &["flat"],
					set_icon_name: icon_name::MENU,
					set_menu_model: Some(&primary_menu),
				},
			},
			#[wrap(Some)]
			set_content = &gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				append = match model.state {
					TaskListSidebarStatus::Empty => {
						gtk::Box {
							set_margin_all: 20,
							set_vexpand: true,
							set_valign: gtk::Align::Center,
							set_css_classes: &["empty-state"],
							set_orientation: gtk::Orientation::Vertical,
							set_spacing: 10,
							gtk::Image {
								set_icon_name: Some(icon_name::DOCK_LEFT),
								set_pixel_size: 64,
								set_margin_all: 10,
							},
							gtk::Label {
								add_css_class: "title-2",
								set_wrap: true,
								set_wrap_mode: gtk::pango::WrapMode::Word,
								set_justify: gtk::Justification::Center,
								set_label: fl!("empty-middle-tittle"),
							},
							gtk::Label {
								add_css_class: "body",
								set_wrap: true,
								set_wrap_mode: gtk::pango::WrapMode::Word,
								set_justify: gtk::Justification::Center,
								set_label: fl!("middle-empty-instructions"),
							}
						}
					},
					TaskListSidebarStatus::Loading => {
						gtk::CenterBox {
							set_orientation: gtk::Orientation::Vertical,
							#[name(spinner)]
							#[wrap(Some)]
							set_center_widget = &gtk::Spinner {
								start: ()
							}
						}
					},
					TaskListSidebarStatus::Loaded => {
						gtk::ScrolledWindow {
							set_vexpand: true,
							#[local_ref]
							task_list_widget -> gtk::ListBox {
								set_css_classes: &["navigation-sidebar"],
							},
						}
					}
				}
			}
		}
	}

	async fn init(
		init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let keyboard_shortcuts: &str = fl!("keyboard-shortcuts");
		let about_done: &str = fl!("about-done");
		let preferences: &str = fl!("preferences");
		let quit: &str = fl!("quit");

		let model = TaskListSidebarModel {
			service: init,
			state: TaskListSidebarStatus::Empty,
			task_list_factory: AsyncFactoryVecDeque::builder()
				.launch(gtk::ListBox::default())
				.forward(sender.input_sender(), |output| match output {
					TaskListFactoryOutput::Select(list) => {
						TaskListSidebarInput::SelectList(list)
					},
					TaskListFactoryOutput::DeleteTaskList(index) => {
						TaskListSidebarInput::DeleteTaskList(index)
					},
				}),
			list_entry: ListDialogComponent::builder().launch(None).forward(
				sender.input_sender(),
				|message| match message {
					ListDialogOutput::AddTaskListToSidebar(name) => {
						TaskListSidebarInput::AddTaskListToSidebar(name)
					},
					ListDialogOutput::RenameList(_) => todo!(),
				},
			),
			services_sidebar_controller: ServicesSidebarModel::builder()
				.launch(())
				.forward(sender.input_sender(), |message| match message {
					ServicesSidebarOutput::ServiceSelected(service) => {
						TaskListSidebarInput::ServiceSelected(service)
					},
					ServicesSidebarOutput::ServiceDisabled(service) => {
						TaskListSidebarInput::ServiceDisabled(service)
					},
				}),
			handle: None,
		};
		sender.input(TaskListSidebarInput::LoadTaskLists);
		let task_list_widget = model.task_list_factory.widget();
		let widgets = view_output!();
		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
	) {
		match message {
			TaskListSidebarInput::AddTaskListToSidebar(name) => {
				match self
					.service
					.get_service()
					.create_list(List::new(&name, self.service))
					.await
				{
					Ok(list) => {
						let mut guard = self.task_list_factory.guard();
						guard.push_back(TaskListFactoryInit::new(
							self.service,
							SidebarList::Custom(list),
						));
						self.state = TaskListSidebarStatus::Loaded;
					},
					Err(e) => {
						tracing::error!("Error while creating task list: {}", e);
					},
				}
			},
			TaskListSidebarInput::ReloadSidebar(service) => self
				.services_sidebar_controller
				.sender()
				.send(ServicesSidebarInput::ReloadSidebar(service))
				.unwrap_or_default(),
			TaskListSidebarInput::OpenNewTaskListDialog => {
				let list_entry = self.list_entry.widget();
				list_entry.present();
			},
			TaskListSidebarInput::ServiceSelected(service) => {
				self.service = service;
				if let Some(handle) = &self.handle {
					handle.abort()
				}
				self.state = TaskListSidebarStatus::Loading;
				sender.input(TaskListSidebarInput::LoadTaskLists);
			},
			TaskListSidebarInput::ServiceDisabled(service) => {
				if self.service == service {
					self.service = Service::Smart;
					self.state = TaskListSidebarStatus::Loading;
					sender.input(TaskListSidebarInput::LoadTaskLists);
				}
				sender
					.output(TaskListSidebarOutput::ServiceDisabled(service))
					.unwrap_or_default()
			},
			TaskListSidebarInput::LoadTaskList(list) => {
				let mut guard = self.task_list_factory.guard();
				guard.push_back(TaskListFactoryInit::new(
					self.service,
					SidebarList::Custom(list),
				));
				self.state = TaskListSidebarStatus::Loaded;
			},
			TaskListSidebarInput::SetStatus(status) => {
				self.state = status;
			},
			TaskListSidebarInput::LoadTaskLists => {
				let mut guard = self.task_list_factory.guard();
				guard.clear();

				let mut service = self.service.get_service();
				if service.stream_support() {
					let sender_clone = sender.clone();
					self.handle = Some(tokio::spawn(async move {
						match service.get_lists().await {
							Ok(mut stream) => {
								let first = stream.next().await;
								if let Some(list) = first {
									sender_clone.input(TaskListSidebarInput::LoadTaskList(list));
									while let Some(list) = stream.next().await {
										sender_clone
											.input(TaskListSidebarInput::LoadTaskList(list));
									}
								} else {
									sender_clone.input(TaskListSidebarInput::SetStatus(
										TaskListSidebarStatus::Empty,
									));
								}
							},
							Err(err) => tracing::error!("{err}"),
						}
					}));
				} else {
					if matches!(self.service, Service::Smart) {
						for smart_list in SidebarList::list() {
							guard.push_back(TaskListFactoryInit::new(
								Service::Smart,
								smart_list,
							));
						}
					} else {
						for list in service.read_lists().await.unwrap() {
							guard.push_back(TaskListFactoryInit::new(
								self.service,
								SidebarList::Custom(list),
							));
						}
					}
					if guard.is_empty() {
						self.state = TaskListSidebarStatus::Empty;
					} else {
						self.state = TaskListSidebarStatus::Loaded;
					}
				}
			},
			TaskListSidebarInput::SelectList(list) => sender
				.output(TaskListSidebarOutput::SelectList(list, self.service))
				.unwrap(),
			TaskListSidebarInput::DeleteTaskList(index) => {
				self.task_list_factory.guard().remove(index.current_index());
				sender
					.output(TaskListSidebarOutput::CleanContent)
					.unwrap_or_default();
				if self.task_list_factory.is_empty() {
					self.state = TaskListSidebarStatus::Empty;
				}
			},
		}
	}
}
