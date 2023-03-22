use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{DynamicIndex, FactoryView};
use relm4::gtk::prelude::{ButtonExt, EntryBufferExtManual, WidgetExt};
use relm4::gtk::traits::{BoxExt, EntryExt, OrientableExt};
use relm4::loading_widgets::LoadingWidgets;
use relm4::{AsyncFactorySender, RelmWidgetExt};

use crate::gtk;
use crate::widgets::lists::messages::TaskListsInput;

use super::{
	messages::{TaskListFactoryInput, TaskListFactoryOutput},
	model::{TaskListFactoryInit, TaskListFactoryModel},
};

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for TaskListFactoryModel {
	type ParentInput = TaskListsInput;
	type ParentWidget = gtk::ListBox;
	type CommandOutput = ();
	type Input = TaskListFactoryInput;
	type Output = TaskListFactoryOutput;
	type Init = TaskListFactoryInit;
	type Widgets = ListWidgets;

	view! {
		#[root]
		gtk::ListBoxRow {
			#[name(container)]
			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				gtk::Box {
					set_css_classes: &["linked"],
					#[watch]
					set_visible: !self.edit_mode,
					gtk::MenuButton {
						#[watch]
						set_label: if self.list.icon.is_some() {
							self.list.icon.as_ref().unwrap().as_str()
						} else {
							""
						},
						set_css_classes: &["flat", "image-button"],
						set_valign: gtk::Align::Center,
						#[wrap(Some)]
						set_popover = &gtk::EmojiChooser{
							connect_emoji_picked[sender] => move |_, emoji| {
								sender.input(TaskListFactoryInput::ChangeIcon(emoji.to_string()));
							}
						}
					},
					gtk::Box {
						set_orientation: gtk::Orientation::Vertical,
						set_margin_all: 10,
						gtk::Label {
							set_hexpand: true,
							set_css_classes: &["dim-label", "caption"],
							set_halign: gtk::Align::Start,
							#[watch]
							set_text: &self.plugin.name
						},
						gtk::Label {
							set_hexpand: true,
							set_halign: gtk::Align::Start,
							#[watch]
							set_text: &self.list.name
						},
						add_controller = gtk::GestureClick {
							connect_pressed[sender] => move |_, _, _, _| {
								sender.input(TaskListFactoryInput::Select);
								sender.output(TaskListFactoryOutput::Forward);
							}
						}
					},
					gtk::Button {
						set_icon_name: "editor",
						set_valign: gtk::Align::Center,
						connect_clicked => TaskListFactoryInput::EditMode,
					},
					gtk::Button {
						set_icon_name: "user-trash-full",
						set_valign: gtk::Align::Center,
						connect_clicked[sender, index] => move |_| {
							sender.input(TaskListFactoryInput::Delete(index.clone()));
						}
					},
				},
				gtk::Box {
					set_css_classes: &["linked"],
					#[watch]
					set_visible: self.edit_mode,
					set_margin_all: 10,
					gtk::MenuButton {
						#[watch]
						set_label: if self.list.icon.is_some() {
							self.list.icon.as_ref().unwrap().as_str()
						} else {
							""
						},
						set_css_classes: &["flat", "image-button"],
						set_valign: gtk::Align::Center,
						#[wrap(Some)]
						set_popover = &gtk::EmojiChooser{
							connect_emoji_picked[sender] => move |_, emoji| {
								sender.input(TaskListFactoryInput::ChangeIcon(emoji.to_string()));
							}
						}
					},
					gtk::Entry {
						set_hexpand: true,
						set_buffer: &self.entry,
					},
					gtk::Button {
						set_icon_name: "emblem-default",
						set_valign: gtk::Align::Center,
						connect_clicked => TaskListFactoryInput::Rename
					},
					gtk::Button {
						set_icon_name: "user-trash-full",
						set_valign: gtk::Align::Center,
						connect_clicked[sender, index] => move |_| {
							sender.input(TaskListFactoryInput::Delete(index.clone()));
						}
					},
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
		_index: &DynamicIndex,
		_sender: AsyncFactorySender<Self>,
	) -> Self {
		let init_text = init.list.name.clone();
		TaskListFactoryModel {
			list: init.list,
			plugin: init.plugin,
			entry: gtk::EntryBuffer::new(Some(init_text)),
			edit_mode: false,
		}
	}

	fn init_widgets(
		&mut self,
		index: &DynamicIndex,
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
		match message {
			TaskListFactoryInput::EditMode => self.edit_mode = true,
			TaskListFactoryInput::Rename => {
				self.edit_mode = false;
				let mut list = self.list.clone();
				list.name = self.entry.text().to_string();
				if let Ok(client) = &mut self.plugin.connect().await {
					match client.update_list(list).await {
						Ok(response) => {
							let response = response.into_inner();
							if response.successful {
								self.list.name = self.entry.text().to_string();
							}
							sender.output(TaskListFactoryOutput::Notify(response.message));
						},
						Err(err) => {
							sender.output(TaskListFactoryOutput::Notify(err.to_string()))
						},
					}
				}
			},
			TaskListFactoryInput::Delete(index) => {
				let list_id = self.list.id.clone();
				if let Ok(client) = &mut self.plugin.connect().await {
					match client.delete_list(list_id.clone()).await {
						Ok(response) => {
							let response = response.into_inner();
							if response.successful {
								sender.output(TaskListFactoryOutput::DeleteTaskList(
									index, list_id,
								));
							}
							sender.output(TaskListFactoryOutput::Notify(response.message));
						},
						Err(err) => {
							sender.output(TaskListFactoryOutput::Notify(err.to_string()))
						},
					}
				}
			},
			TaskListFactoryInput::ChangeIcon(icon) => {
				if let Ok(client) = &mut self.plugin.connect().await {
					let mut list = self.list.clone();
					list.icon = Some(icon.clone());
					match client.update_list(list).await {
						Ok(response) => {
							let response = response.into_inner();
							if response.successful {
								self.list.icon = Some(icon);
							}
							sender.output(TaskListFactoryOutput::Notify(response.message));
						},
						Err(err) => {
							sender.output(TaskListFactoryOutput::Notify(err.to_string()))
						},
					}
				}
			},
			TaskListFactoryInput::Select => {
				sender.output(TaskListFactoryOutput::Select(Box::new(
					TaskListFactoryInit::new(self.plugin.clone(), self.list.clone()),
				)));
			},
		}
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		match output {
			TaskListFactoryOutput::Select(data) => {
				Some(TaskListsInput::ListSelected(Box::new(
					TaskListFactoryInit::new(data.plugin, data.list),
				)))
			},
			TaskListFactoryOutput::DeleteTaskList(index, list_id) => {
				Some(TaskListsInput::DeleteTaskList(index, list_id))
			},
			TaskListFactoryOutput::Forward => Some(TaskListsInput::Forward),
			TaskListFactoryOutput::Notify(msg) => Some(TaskListsInput::Notify(msg)),
		}
	}
}
