use done_local_storage::LocalStorage;
use libset::format::FileFormat;
use libset::project::Project;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{DynamicIndex, FactoryView};
use relm4::gtk::gio::{Menu, MenuItem};
use relm4::gtk::prelude::{EntryBufferExtManual, ListBoxRowExt, WidgetExt};
use relm4::gtk::traits::{BoxExt, GestureSingleExt, PopoverExt};
use relm4::loading_widgets::LoadingWidgets;
use relm4::{AsyncFactorySender, RelmWidgetExt};

use crate::gtk;
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
			connect_activate => TaskListFactoryInput::Select,
			#[name(container)]
			gtk::Box {
				set_has_tooltip: true,
				set_tooltip_text: Some(self.list.name.as_str()),
				set_valign: gtk::Align::Center,
				gtk::Label {
					#[watch]
					set_label: self.list.icon.as_deref().unwrap_or_default(),
					#[watch]
					set_visible: !self.extended,
					set_valign: gtk::Align::Center,
					set_halign: gtk::Align::Center,
					set_hexpand: true,
				},
				gtk::MenuButton {
					#[watch]
					set_label: self.list.icon.as_deref().unwrap_or_default(),
					#[watch]
					set_visible: self.extended,
					set_css_classes: &["flat", "image-button"],
					set_margin_all: 2,
					set_valign: gtk::Align::Center,
					#[wrap(Some)]
					set_popover = &gtk::EmojiChooser {
						set_has_tooltip: true,
						set_tooltip_text: Some("Set list icon"),
						connect_emoji_picked[sender] => move |_, emoji| {
							sender.input(TaskListFactoryInput::ChangeIcon(emoji.to_string()));
						}
					}
				},
				gtk::Label {
					#[watch]
					set_visible: self.extended,
					set_hexpand: true,
					#[watch]
					set_text: &self.list.name
				},
				gtk::MenuButton {
					#[watch]
					set_visible: self.extended,
					set_icon_name: "view-more-symbolic",
					set_css_classes: &["flat", "image-button"],
					set_valign: gtk::Align::Center,
					set_menu_model: Some(&primary_menu),
				}
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
		_index: &DynamicIndex,
		_sender: AsyncFactorySender<Self>,
	) -> Self {
		let preferences =
			if let Ok(project) = Project::open("dev", "edfloreshz", "done") {
				project
					.get_file_as::<Preferences>("preferences", FileFormat::JSON)
					.unwrap_or(Preferences::new().await)
			} else {
				Preferences::new().await
			};
		let init_text = init.list.name.clone();
		TaskListFactoryModel {
			list: init.list,
			entry: gtk::EntryBuffer::new(Some(init_text)),
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
		widgets
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncFactorySender<Self>,
	) {
		let local = LocalStorage::new();
		match message {
			TaskListFactoryInput::ToggleExtended(extended) => {
				self.extended = extended
			},
			TaskListFactoryInput::Rename => {
				let mut list = self.list.clone();
				list.name = self.entry.text().to_string();
				match local.update_list(list).await {
					Ok(_) => {
						self.list.name = self.entry.text().to_string();
					},
					Err(err) => {
						sender.output(TaskListFactoryOutput::Notify(err.to_string()))
					},
				}
			},
			TaskListFactoryInput::Delete(index) => {
				let list_id = self.list.id.clone();
				match local.delete_list(list_id.clone()).await {
					Ok(_) => {
						sender
							.output(TaskListFactoryOutput::DeleteTaskList(index, list_id));
					},
					Err(err) => {
						sender.output(TaskListFactoryOutput::Notify(err.to_string()))
					},
				}
			},
			TaskListFactoryInput::ChangeIcon(icon) => {
				let mut list = self.list.clone();
				list.icon = Some(icon.clone());
				match local.update_list(list).await {
					Ok(_) => {
						self.list.icon = Some(icon);
					},
					Err(err) => {
						sender.output(TaskListFactoryOutput::Notify(err.to_string()))
					},
				}
			},
			TaskListFactoryInput::Select => {
				sender.output(TaskListFactoryOutput::Select(Box::new(
					TaskListFactoryInit::new(self.list.clone()),
				)));
			},
			TaskListFactoryInput::OpenRightClickMenu => {

				// let menu = Menu::new();
				// menu.append(Some("Rename"), Some("app.rename"));
				// menu.append(Some("Delete"), Some("app.delete"));
				// let popover_menu = PopoverMenu::from_model(Some(&menu));
				// widgets.container.
				// popover_menu.set_wid(Some(&widgets.container));
				// popover_menu.popup();
			},
		}
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		match output {
			TaskListFactoryOutput::Select(data) => Some(
				SidebarComponentInput::SelectList(SidebarList::Custom(data.list)),
			),
			TaskListFactoryOutput::DeleteTaskList(index, list_id) => {
				Some(SidebarComponentInput::DeleteTaskList(index, list_id))
			},
			TaskListFactoryOutput::Notify(msg) => {
				Some(SidebarComponentInput::Notify(msg))
			},
		}
	}
}

// gtk::Box {
// 	#[watch]
// 	set_visible: !self.edit_mode,
// 	gtk::Box {
// 		set_orientation: gtk::Orientation::Vertical,
// 		set_margin_all: 10,

// 		add_controller = gtk::GestureClick {
// 			connect_pressed[sender] => move |_, _, _, _| {
// 				sender.input(TaskListFactoryInput::Select);
// 			}
// 		}
// 	},
// 	gtk::Box {
// 		set_css_classes: &["linked"],
// 		gtk::Button {
// 			set_has_tooltip: true,
// 			set_tooltip_text: Some("Edit task list name"),
// 			set_icon_name: icon_name::PENCIL_AND_PAPER,
// 			set_valign: gtk::Align::Center,
// 			connect_clicked => TaskListFactoryInput::EditMode,
// 		},
// 		gtk::Button {
// 			set_has_tooltip: true,
// 			set_tooltip_text: Some("Remove task list"),
// 			set_icon_name: icon_name::X_CIRCULAR,
// 			set_valign: gtk::Align::Center,
// 			connect_clicked[sender, index] => move |_| {
// 				sender.input(TaskListFactoryInput::Delete(index.clone()));
// 			}
// 		},
// 	}
// },
// gtk::Box {
// 	#[watch]
// 	set_visible: self.edit_mode,
// 	set_margin_all: 10,
// 	set_css_classes: &["linked"],
// 	gtk::Entry {
// 		set_hexpand: true,
// 		set_buffer: &self.entry,
// 	},
// 	gtk::Button {
// 		set_has_tooltip: true,
// 		set_tooltip_text: Some("Rename list"),
// 		set_icon_name: icon_name::CHECK_ROUND_OUTLINE_WHOLE,
// 		set_valign: gtk::Align::Center,
// 		connect_clicked => TaskListFactoryInput::Rename
// 	},
// 	gtk::Button {
// 		set_has_tooltip: true,
// 		set_tooltip_text: Some("Remove task list"),
// 		set_icon_name: icon_name::X_CIRCULAR,
// 		set_valign: gtk::Align::Center,
// 		connect_clicked[sender, index] => move |_| {
// 			sender.input(TaskListFactoryInput::Delete(index.clone()));
// 		}
// 	},
// },
