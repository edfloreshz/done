use relm4::adw::{prelude::EntryRowExt, traits::PreferencesRowExt};
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{DynamicIndex, FactoryView};
use relm4::gtk::prelude::{ButtonExt, EditableExt, WidgetExt};
use relm4::gtk::traits::{BoxExt, ListBoxRowExt};
use relm4::loading_widgets::LoadingWidgets;
use relm4::AsyncFactorySender;

use crate::widgets::lists::messages::TaskListsInput;
use crate::{adw, gtk};

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
		adw::EntryRow {
			set_title: self.plugin.name.as_str(),
			set_show_apply_button: true,
			set_enable_emoji_completion: true,
			set_text: self.list.name.as_str(),
			connect_activate[sender] => move |entry| {
				let buffer = entry.text().to_string();
				sender.input(TaskListFactoryInput::Rename(buffer));
			},
			connect_apply[sender] => move |entry| {
				let buffer = entry.text().to_string();
				sender.input(TaskListFactoryInput::Rename(buffer));
			},
			add_prefix = &gtk::MenuButton {
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
			add_suffix = &gtk::Label {
				set_halign: gtk::Align::End,
				set_css_classes: &["dim-label", "caption"],
				// #[watch]
				// set_text: self.count.to_string().as_str(),
			},
			add_suffix = &gtk::Button {
				set_icon_name: "user-trash-full-symbolic",
				set_css_classes: &["circular", "image-button", "destructive-action"],
				set_valign: gtk::Align::Center,
				connect_clicked[sender, index] => move |_| {
					sender.input(TaskListFactoryInput::Delete(index.clone()));
				}
			},
			add_controller = gtk::GestureClick {
				connect_pressed[sender] => move |_, _, _, _| {
					sender.input(TaskListFactoryInput::Select);
					sender.output(TaskListFactoryOutput::Forward);
				}
			}
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
		TaskListFactoryModel {
			list: init.list,
			plugin: init.plugin,
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
			TaskListFactoryInput::Rename(name) => {
				let mut list = self.list.clone();
				list.name = name.clone();
				if let Ok(client) = &mut self.plugin.connect().await {
					match client.update_list(list).await {
						Ok(response) => {
							let response = response.into_inner();
							if response.successful {
								self.list.name = name;
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
				sender.output(TaskListFactoryOutput::Select(Box::new(self.clone())));
			},
		}
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		match output {
			TaskListFactoryOutput::Select(data) => {
				Some(TaskListsInput::ListSelected(data))
			},
			TaskListFactoryOutput::DeleteTaskList(index, list_id) => {
				Some(TaskListsInput::DeleteTaskList(index, list_id))
			},
			TaskListFactoryOutput::Forward => Some(TaskListsInput::Forward),
			TaskListFactoryOutput::Notify(msg) => Some(TaskListsInput::Notify(msg)),
		}
	}
}
