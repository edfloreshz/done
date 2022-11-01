use done_core::enums::TaskStatus;
use relm4::factory::{
	DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryView,
};
use relm4::gtk;
use relm4::gtk::prelude::{
	BoxExt, ButtonExt, CheckButtonExt, EditableExt, EntryBufferExtManual,
	EntryExt, ListBoxRowExt, OrientableExt, ToggleButtonExt, WidgetExt,
};
use relm4::WidgetPlus;

use crate::plugins::client::Task;
use crate::widgets::components::content::ContentInput;

#[derive(Debug)]
pub enum TaskInput {
	SetCompleted(bool),
	Favorite(DynamicIndex),
	ModifyTitle(String),
}

#[derive(Debug)]
pub enum TaskOutput {
	Remove(DynamicIndex),
	UpdateTask(Option<DynamicIndex>, Task),
}

#[relm4::factory(pub)]
impl FactoryComponent for Task {
	type ParentMsg = ContentInput;
	type ParentWidget = gtk::Box;
	type CommandOutput = ();
	type Input = TaskInput;
	type Output = TaskOutput;
	type Init = Task;
	type Widgets = TaskWidgets;

	view! {
		root = gtk::ListBoxRow {
			set_selectable: false,
			#[name = "container"]
			gtk::Box {
				append = &gtk::Box {
					set_orientation: gtk::Orientation::Horizontal,
					set_spacing: 5,
					set_margin_top: 10,
					set_margin_bottom: 10,
					set_margin_start: 10,
					set_margin_end: 10,
					#[name = "check_button"]
					gtk::CheckButton {
						set_active: self.status == 1,
						connect_toggled[sender] => move |checkbox| {
							sender.input.send(TaskInput::SetCompleted(checkbox.is_active()));
						}
					},
					gtk::Box {
						set_orientation: gtk::Orientation::Horizontal,
						set_spacing: 15,
						#[name = "entry"]
						gtk::Entry {
							add_css_class: "flat",
							add_css_class: "no-border",
							set_hexpand: true,
							set_text: &self.title,
							connect_activate[sender] => move |entry| {
								let buffer = entry.buffer();
								sender.input.send(TaskInput::ModifyTitle(buffer.text()));
							},
							// connect_insert_text[sender] => move |entry, _, _| {
							// 	let buffer = entry.buffer();
							// 	sender.input.send(TaskInput::ModifyTitle(buffer.text()));
							// },
							// connect_delete_text[sender] => move |entry, _, _| {
							// 	let buffer = entry.buffer();
							// 	sender.input.send(TaskInput::ModifyTitle(buffer.text()));
							// }
						},
						#[name = "favorite"]
						gtk::ToggleButton {
							add_css_class: "opaque",
							add_css_class: "circular",
							#[watch]
							set_class_active: ("favorite", self.favorite),
							set_icon_name: "star-filled-rounded-symbolic",
							connect_toggled[sender, index] => move |_| {
								sender.input.send(TaskInput::Favorite(index.clone()));
							}
						},
						#[name = "delete"]
						gtk::Button {
							add_css_class: "destructive-action",
							add_css_class: "circular",
							set_icon_name: "user-trash-full-symbolic",
							connect_clicked[sender, index] => move |_| {
								sender.output.send(TaskOutput::Remove(index.clone()))
							}
						}
					}
				}
			}
		}
	}

	fn init_model(
		params: Self::Init,
		_index: &DynamicIndex,
		_sender: FactoryComponentSender<Self>,
	) -> Self {
		params
	}

	fn init_widgets(
		&mut self,
		index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: FactoryComponentSender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}

	fn update(
		&mut self,
		message: Self::Input,
		sender: FactoryComponentSender<Self>,
	) {
		match message {
			TaskInput::SetCompleted(completed) => {
				self.status = if completed {
					TaskStatus::Completed as i32
				} else {
					TaskStatus::NotStarted as i32
				};
				sender
					.output
					.send(TaskOutput::UpdateTask(None, self.clone()));
			},
			TaskInput::Favorite(index) => {
				self.favorite = !self.favorite;
				sender
					.output
					.send(TaskOutput::UpdateTask(Some(index), self.clone()));
			},
			TaskInput::ModifyTitle(title) => {
				self.title = title;
				sender
					.output
					.send(TaskOutput::UpdateTask(None, self.clone()));
			},
		}
	}

	fn output_to_parent_msg(output: Self::Output) -> Option<ContentInput> {
		Some(match output {
			TaskOutput::Remove(index) => ContentInput::RemoveTask(index),
			TaskOutput::UpdateTask(index, task) => {
				ContentInput::UpdateTask(index, task)
			},
		})
	}
}
