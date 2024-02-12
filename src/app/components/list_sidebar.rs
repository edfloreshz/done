use adw::{gtk::prelude::ActionableExt, prelude::AdwDialogExt};
use core_done::{models::list::List, service::Service};
use futures::StreamExt;
use relm4::{
	actions::{ActionGroupName, RelmAction, RelmActionGroup},
	adw,
	component::{
		AsyncComponent, AsyncComponentController, AsyncComponentParts,
		AsyncController, SimpleAsyncComponent,
	},
	factory::AsyncFactoryVecDeque,
	gtk::{
		self,
		prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt},
	},
	prelude::DynamicIndex,
	tokio, AsyncComponentSender, Component, ComponentController, Controller,
	JoinHandle, RelmWidgetExt,
};
use relm4_icons::icon_name;

use crate::{
	app::{
		components::{list_dialog::ListDialogOutput, services::ServicesOutput},
		factories::task_list::{
			TaskListFactoryInit, TaskListFactoryModel, TaskListFactoryOutput,
		},
		models::sidebar_list::SidebarList,
		AboutAction, NewListAction, PreferencesAction, QuitAction, ShortcutsAction,
		WindowActionGroup,
	},
	fl,
};

use super::{
	list_dialog::ListDialogComponent,
	services::{ServicesInput, ServicesModel},
};

pub struct ListSidebarModel {
	service: Service,
	state: ListSidebarStatus,
	task_list_factory: AsyncFactoryVecDeque<TaskListFactoryModel>,
	list_entry: Controller<ListDialogComponent>,
	services_sidebar_controller: AsyncController<ServicesModel>,
	handle: Option<JoinHandle<()>>,
}

#[derive(Debug)]
pub enum ListSidebarInput {
	LoadTaskLists,
	LoadTaskList(List),
	AddTaskListToSidebar(String),
	ServiceSelected(Service),
	ServiceDisabled(Service),
	SelectList(SidebarList),
	DeleteTaskList(DynamicIndex),
	SetStatus(ListSidebarStatus),
	ReloadSidebar(Service),
	OpenNewTaskListDialog,
}

#[derive(Debug)]
pub enum ListSidebarOutput {
	SelectList(SidebarList, Service),
	ServiceDisabled(Service),
	CleanContent,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListSidebarStatus {
	Empty,
	Loading,
	Loaded,
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for ListSidebarModel {
	type Input = ListSidebarInput;
	type Output = ListSidebarOutput;
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
				#[name(new_list_button)]
				pack_start = &gtk::ToggleButton {
					set_tooltip: fl!("add-new-task-list"),
					set_icon_name: icon_name::PLUS,
					set_css_classes: &["flat", "image-button"],
					set_valign: gtk::Align::Center,
					connect_clicked => ListSidebarInput::OpenNewTaskListDialog,
					set_action_name: Some("win.new-list"),
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
				append = model.services_sidebar_controller.widget(),
				append = &gtk::Box {
					set_margin_start: 15,
					set_margin_end: 15,
					set_margin_bottom: 5,
					gtk::Label {
						set_css_classes: &["heading"],
						#[watch]
						set_text: &model.service.to_string()
					}
				},
				append = match model.state {
					ListSidebarStatus::Empty => {
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
					ListSidebarStatus::Loading => {
						gtk::CenterBox {
							set_orientation: gtk::Orientation::Vertical,
							#[name(spinner)]
							#[wrap(Some)]
							set_center_widget = &gtk::Spinner {
								start: ()
							}
						}
					},
					ListSidebarStatus::Loaded => {
						gtk::ScrolledWindow {
							set_vexpand: true,
							gtk::Box {
								set_orientation: gtk::Orientation::Vertical,
								#[local_ref]
								task_list_widget -> gtk::ListBox {
									set_margin_all: 10,
									set_css_classes: &["boxed-list"],
								},
							}
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

		let model = ListSidebarModel {
			service: init,
			state: ListSidebarStatus::Empty,
			task_list_factory: AsyncFactoryVecDeque::builder()
				.launch(gtk::ListBox::default())
				.forward(sender.input_sender(), |output| match output {
					TaskListFactoryOutput::Select(list) => {
						ListSidebarInput::SelectList(list)
					},
					TaskListFactoryOutput::DeleteTaskList(index) => {
						ListSidebarInput::DeleteTaskList(index)
					},
				}),
			list_entry: ListDialogComponent::builder().launch(None).forward(
				sender.input_sender(),
				|message| match message {
					ListDialogOutput::AddTaskListToSidebar(name) => {
						ListSidebarInput::AddTaskListToSidebar(name)
					},
					ListDialogOutput::RenameList(_) => todo!(),
				},
			),
			services_sidebar_controller: ServicesModel::builder().launch(()).forward(
				sender.input_sender(),
				|message| match message {
					ServicesOutput::ServiceSelected(service) => {
						ListSidebarInput::ServiceSelected(service)
					},
					ServicesOutput::ServiceDisabled(service) => {
						ListSidebarInput::ServiceDisabled(service)
					},
				},
			),
			handle: None,
		};

		let mut actions = RelmActionGroup::<WindowActionGroup>::new();

		let new_list_action = {
			let list_entry = model.list_entry.widget().clone();
			let root_widget = root.clone();
			RelmAction::<NewListAction>::new_stateless(move |_| {
				list_entry.present(&root_widget)
			})
		};

		actions.add_action(new_list_action);

		sender.input(ListSidebarInput::LoadTaskLists);

		let task_list_widget = model.task_list_factory.widget();
		let widgets = view_output!();

		widgets.new_list_button.insert_action_group(
			WindowActionGroup::NAME,
			Some(&actions.into_action_group()),
		);
		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
	) {
		match message {
			ListSidebarInput::AddTaskListToSidebar(name) => {
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
						self.state = ListSidebarStatus::Loaded;
					},
					Err(e) => {
						tracing::error!("Error while creating task list: {}", e);
					},
				}
			},
			ListSidebarInput::ReloadSidebar(service) => self
				.services_sidebar_controller
				.sender()
				.send(ServicesInput::ReloadServices(service))
				.unwrap_or_default(),
			ListSidebarInput::OpenNewTaskListDialog => {
				let list_entry = self.list_entry.widget();
				list_entry.present(&list_entry.parent().unwrap());
			},
			ListSidebarInput::ServiceSelected(service) => {
				self.service = service;
				if let Some(handle) = &self.handle {
					handle.abort()
				}
				self.state = ListSidebarStatus::Loading;
				sender.input(ListSidebarInput::LoadTaskLists);
			},
			ListSidebarInput::ServiceDisabled(service) => {
				if self.service == service {
					self.service = Service::Smart;
					self.state = ListSidebarStatus::Loading;
					sender.input(ListSidebarInput::LoadTaskLists);
				}
				sender
					.output(ListSidebarOutput::ServiceDisabled(service))
					.unwrap_or_default()
			},
			ListSidebarInput::LoadTaskList(list) => {
				let mut guard = self.task_list_factory.guard();
				guard.push_back(TaskListFactoryInit::new(
					self.service,
					SidebarList::Custom(list),
				));
				self.state = ListSidebarStatus::Loaded;
			},
			ListSidebarInput::SetStatus(status) => {
				self.state = status;
			},
			ListSidebarInput::LoadTaskLists => {
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
									sender_clone.input(ListSidebarInput::LoadTaskList(list));
									while let Some(list) = stream.next().await {
										sender_clone.input(ListSidebarInput::LoadTaskList(list));
									}
								} else {
									sender_clone.input(ListSidebarInput::SetStatus(
										ListSidebarStatus::Empty,
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
						self.state = ListSidebarStatus::Empty;
					} else {
						self.state = ListSidebarStatus::Loaded;
					}
				}
			},
			ListSidebarInput::SelectList(list) => sender
				.output(ListSidebarOutput::SelectList(list, self.service))
				.unwrap(),
			ListSidebarInput::DeleteTaskList(index) => {
				self.task_list_factory.guard().remove(index.current_index());
				sender
					.output(ListSidebarOutput::CleanContent)
					.unwrap_or_default();
				if self.task_list_factory.is_empty() {
					self.state = ListSidebarStatus::Empty;
				}
			},
		}
	}
}
