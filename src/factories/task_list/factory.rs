use libset::format::FileFormat;
use libset::project::Project;
use relm4::actions::{ActionGroupName, RelmAction, RelmActionGroup};
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{DynamicIndex, FactoryView};
use relm4::gtk::prelude::{ListBoxRowExt, WidgetExt};
use relm4::gtk::traits::{BoxExt, GtkWindowExt};
use relm4::loading_widgets::LoadingWidgets;
use relm4::{
	gtk, AsyncFactorySender, Component, ComponentController, RelmWidgetExt,
};

use crate::fl;
use crate::widgets::delete::{DeleteComponent, DeleteInit, DeleteOutput};
use crate::widgets::list_dialog::messages::ListDialogOutput;
use crate::widgets::list_dialog::model::ListDialogComponent;
use crate::widgets::preferences::model::Preferences;
use crate::widgets::sidebar::messages::SidebarComponentInput;
use crate::widgets::sidebar::model::SidebarList;

use super::{
	messages::{TaskListFactoryInput, TaskListFactoryOutput},
	model::{TaskListFactoryInit, TaskListFactoryModel},
};

relm4::new_action_group!(pub(super) TaskListActionGroup, "win");
relm4::new_stateless_action!(RenameAction, TaskListActionGroup, "rename");
relm4::new_stateless_action!(DeleteAction, TaskListActionGroup, "delete");

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for TaskListFactoryModel {
	type ParentInput = SidebarComponentInput;
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
			#[name(container)]
			gtk::Box {
				set_css_classes: &["toolbar"],
				gtk::Box {
					set_css_classes: &["plugin"],
					gtk::Image {
						#[watch]
						set_visible: self.smart,
						#[watch]
						set_icon_name: self.list.icon(),
						set_margin_all: if self.extended { 5 } else { 0 },
					},
					gtk::Label {
						#[watch]
						set_visible: !self.smart && !self.extended,
						#[watch]
						set_text: self.list.icon().unwrap_or_default(),
					},
					gtk::MenuButton {
						#[watch]
						set_label: self.list.icon().unwrap_or_default(),
						#[watch]
						set_visible: !self.smart && self.extended,
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
						set_visible: self.extended,
						#[watch]
						set_hexpand: !self.smart,
						#[watch]
						set_halign: gtk::Align::Start,
						set_wrap: true,
						set_natural_wrap_mode: gtk::NaturalWrapMode::Word,
						set_text: self.list.name().as_str(),
						set_margin_all: 5,
					},
					#[name(list_actions)]
					gtk::MenuButton {
						#[watch]
						set_visible: !self.smart && self.extended,
						set_icon_name: "view-more-symbolic",
						set_css_classes: &["flat", "image-button"],
						set_valign: gtk::Align::Center,
						set_menu_model: Some(&primary_menu),
					}
				},
			},
		}
	}

	fn init_loading_widgets(root: &mut Self::Root) -> Option<LoadingWidgets> {
		let container = gtk::Box::default();
		container.append(root);
		Some(LoadingWidgets::new(&container, root))
	}

	async fn init_model(
		init: Self::Init,
		index: &DynamicIndex,
		sender: AsyncFactorySender<Self>,
	) -> Self {
		let preferences =
			if let Ok(project) = Project::open("dev", "edfloreshz", "done") {
				project
					.get_file_as::<Preferences>("preferences", FileFormat::JSON)
					.unwrap_or(Preferences::new().await)
			} else {
				Preferences::new().await
			};
		let rename = ListDialogComponent::builder()
			.launch(Some(init.list.name()))
			.forward(sender.input_sender(), |message| match message {
				ListDialogOutput::AddTaskListToSidebar(_, _) => {
					TaskListFactoryInput::Select
				},
				ListDialogOutput::RenameList(name, service) => {
					TaskListFactoryInput::RenameList(name, service)
				},
			});
		let delete = DeleteComponent::builder()
			.launch(DeleteInit {
				warning: format!("You're about to delete this list"),
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
			smart: init.smart,
			extended: preferences.extended,
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
			TaskListFactoryInput::ToggleExtended(extended) => {
				self.extended = extended
			},
			TaskListFactoryInput::RenameList(name, service) => {
				if let SidebarList::Custom(list) = &self.list {
					let mut renamed_list = list.clone();
					renamed_list.name = name.clone();
					let service = service.get_service();
					match service.update_list(renamed_list.clone()).await {
						Ok(_) => self.list = SidebarList::Custom(renamed_list),
						Err(err) => {
							sender.output(TaskListFactoryOutput::Notify(err.to_string()))
						},
					}
				}
			},
			TaskListFactoryInput::Delete => {
				if let SidebarList::Custom(list) = &self.list {
					let list_id = list.id.clone();
					let service = self.service.unwrap().get_service();
					match service.delete_list(list_id.clone()).await {
						Ok(_) => {
							sender.output(TaskListFactoryOutput::DeleteTaskList(
								self.index.clone(),
								list_id,
								self.service.unwrap(),
							));
						},
						Err(err) => {
							sender.output(TaskListFactoryOutput::Notify(err.to_string()))
						},
					}
				}
			},
			TaskListFactoryInput::ChangeIcon(icon) => {
				if let SidebarList::Custom(list) = &self.list {
					let mut list = list.clone();
					list.icon = Some(icon.clone());
					let service = self.service.unwrap().get_service();
					match service.update_list(list.clone()).await {
						Ok(_) => self.list = SidebarList::Custom(list),
						Err(err) => {
							sender.output(TaskListFactoryOutput::Notify(err.to_string()))
						},
					}
				}
			},
			TaskListFactoryInput::Select => {
				sender.output(TaskListFactoryOutput::Select(self.list.clone()));
			},
		}
	}

	fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
		match output {
			TaskListFactoryOutput::Select(list) => {
				Some(SidebarComponentInput::SelectList(list))
			},
			TaskListFactoryOutput::DeleteTaskList(index, list_id, service) => Some(
				SidebarComponentInput::DeleteTaskList(index, list_id, service),
			),
			TaskListFactoryOutput::Notify(msg) => {
				Some(SidebarComponentInput::Notify(msg))
			},
		}
	}
}
