use core_done::service::Service;
use relm4::actions::{ActionGroupName, RelmAction, RelmActionGroup};
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{DynamicIndex, FactoryView};
use relm4::gtk::prelude::{ListBoxRowExt, WidgetExt};
use relm4::gtk::traits::{BoxExt, GtkWindowExt};
use relm4::{
	adw::prelude::{ActionableExt, ActionableExtManual},
	gtk, AsyncFactorySender, Component, ComponentController, Controller,
	RelmWidgetExt,
};

use crate::app::components::delete::{
	DeleteComponent, DeleteInit, DeleteOutput,
};
use crate::app::components::list_dialog::{
	ListDialogComponent, ListDialogOutput,
};
use crate::app::components::task_list_sidebar::TaskListSidebarInput;
use crate::app::models::sidebar_list::SidebarList;
use crate::fl;

#[derive(Debug, derive_new::new)]
pub struct TaskListFactoryModel {
	pub service: Service,
	pub index: DynamicIndex,
	pub list: SidebarList,
	pub rename: Controller<ListDialogComponent>,
	pub delete: Controller<DeleteComponent>,
}

#[derive(Debug, derive_new::new)]
pub struct TaskListFactoryInit {
	pub service: Service,
	pub list: SidebarList,
}

#[derive(Debug)]
pub enum TaskListFactoryInput {
	Select,
	Delete,
	RenameList(String),
	ChangeIcon(String),
}

#[derive(Debug)]
pub enum TaskListFactoryOutput {
	Select(SidebarList),
	DeleteTaskList(DynamicIndex, String),
}

relm4::new_action_group!(pub(super) TaskListActionGroup, "win");
relm4::new_stateless_action!(RenameAction, TaskListActionGroup, "rename");
relm4::new_stateless_action!(DeleteAction, TaskListActionGroup, "delete");

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for TaskListFactoryModel {
	type ParentInput = TaskListSidebarInput;
	type ParentWidget = gtk::ListBox;
	type CommandOutput = ();
	type Input = TaskListFactoryInput;
	type Output = TaskListFactoryOutput;
	type Init = TaskListFactoryInit;
	type Widgets = ListWidgets;

	menu! {
		primary_menu: {
			section! {
				"Rename" => RenameAction,
				"Delete" => DeleteAction,
			}
		}
	}

	view! {
		#[root]
		gtk::ListBoxRow {
			#[watch]
			set_tooltip: self.list.name().as_str(),
			connect_activate => TaskListFactoryInput::Select,
			set_action_name: Some("navigation.push"),
			set_action_target: Some("content-page"),
			#[name(container)]
			gtk::Box {
				add_controller = gtk::GestureClick {
					connect_pressed[sender] => move |_, _, _, _| {
						sender.input(TaskListFactoryInput::Select)
					}
				},
				set_css_classes: &["toolbar"],
				gtk::Box {
					set_css_classes: &["plugin"],
					gtk::Image {
						#[watch]
						set_visible: self.list.smart(),
						#[watch]
						set_icon_name: self.list.icon(),
					},
					gtk::MenuButton {
						#[watch]
						set_label: self.list.icon().unwrap_or_default(),
						#[watch]
						set_visible: !self.list.smart(),
						set_css_classes: &["flat", "image-button"],
						set_valign: gtk::Align::Center,
						#[wrap(Some)]
						set_popover = &gtk::EmojiChooser {
							set_tooltip: fl!("set-list-icon"),
							connect_emoji_picked[sender] => move |_, emoji| {
								sender.input(TaskListFactoryInput::ChangeIcon(emoji.to_string()));
							}
						}
					},
					append = &gtk::Label {
						#[watch]
						set_hexpand: !self.list.smart(),
						#[watch]
						set_halign: gtk::Align::Start,
						set_wrap: true,
						set_natural_wrap_mode: gtk::NaturalWrapMode::Word,
						#[watch]
						set_text: self.list.name().as_str(),
						set_margin_all: 5,
					},
					#[name(list_actions)]
					gtk::MenuButton {
						#[watch]
						set_visible: !self.list.smart(),
						set_icon_name: "view-more-symbolic",
						set_css_classes: &["flat", "image-button"],
						set_valign: gtk::Align::Center,
						set_menu_model: Some(&primary_menu),
					}
				},
			},
		}
	}

	async fn init_model(
		init: Self::Init,
		index: &DynamicIndex,
		sender: AsyncFactorySender<Self>,
	) -> Self {
		let rename = ListDialogComponent::builder()
			.launch(Some(init.list.name()))
			.forward(sender.input_sender(), |message| match message {
				ListDialogOutput::AddTaskListToSidebar(_) => {
					TaskListFactoryInput::Select
				},
				ListDialogOutput::RenameList(name) => {
					TaskListFactoryInput::RenameList(name)
				},
			});
		let delete = DeleteComponent::builder()
			.launch(DeleteInit {
				warning: "You're about to delete this list".into(),
				delete_warning: "If you do this, all of its tasks will be lost.".into(),
			})
			.forward(sender.input_sender(), |message| match message {
				DeleteOutput::Delete => TaskListFactoryInput::Delete,
			});
		TaskListFactoryModel {
			service: init.service,
			index: index.clone(),
			rename,
			delete,
			list: init.list,
		}
	}

	fn init_widgets(
		&mut self,
		_index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: AsyncFactorySender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();

		let mut actions = RelmActionGroup::<TaskListActionGroup>::new();

		let rename_action = {
			let rename_widget = self.rename.widget().clone();
			RelmAction::<RenameAction>::new_stateless(move |_| {
				rename_widget.present()
			})
		};

		let delete_action = {
			let delete_widget = self.delete.widget().clone();
			RelmAction::<DeleteAction>::new_stateless(move |_| {
				delete_widget.present()
			})
		};

		actions.add_action(rename_action);
		actions.add_action(delete_action);

		widgets.list_actions.insert_action_group(
			TaskListActionGroup::NAME,
			Some(&actions.into_action_group()),
		);

		widgets
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncFactorySender<Self>,
	) {
		match message {
			TaskListFactoryInput::Select => {
				sender.output(TaskListFactoryOutput::Select(self.list.clone()));
			},
			TaskListFactoryInput::RenameList(name) => {
				if let SidebarList::Custom(list) = &self.list {
					let mut renamed_list = list.clone();
					renamed_list.name = name.clone();
					let mut service = self.service.get_service();
					match service.update_list(renamed_list.clone()).await {
						Ok(_) => self.list = SidebarList::Custom(renamed_list),
						Err(err) => {
							tracing::error!("{err}");
						},
					}
				}
			},
			TaskListFactoryInput::Delete => {
				if let SidebarList::Custom(list) = &self.list {
					let list_id = list.id.clone();
					let mut service = self.service.get_service();
					match service.delete_list(list_id.clone()).await {
						Ok(_) => {
							sender.output(TaskListFactoryOutput::DeleteTaskList(
								self.index.clone(),
								list_id,
							));
						},
						Err(err) => {
							tracing::error!("{err}");
						},
					}
				}
			},
			TaskListFactoryInput::ChangeIcon(icon) => {
				if let SidebarList::Custom(list) = &self.list {
					let mut list = list.clone();
					list.icon = Some(icon.clone());
					let mut service = self.service.get_service();
					match service.update_list(list.clone()).await {
						Ok(_) => self.list = SidebarList::Custom(list),
						Err(err) => {
							tracing::error!("{err}");
						},
					}
				}
			},
		}
	}

	fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
		match output {
			TaskListFactoryOutput::Select(list) => {
				Some(TaskListSidebarInput::SelectList(list))
			},
			TaskListFactoryOutput::DeleteTaskList(index, list_id) => {
				Some(TaskListSidebarInput::DeleteTaskList(index, list_id))
			},
		}
	}
}
