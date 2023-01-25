use crate::fl;
use proto_rust::provider::{List, Task};
use relm4::{
	gtk,
	gtk::prelude::{
		BoxExt, ButtonExt, EntryBufferExtManual, EntryExt, OrientableExt, WidgetExt,
	},
	Component, ComponentParts, ComponentSender, RelmWidgetExt,
};

use crate::widgets::task_entry::model::TaskEntryModel;

use super::messages::{TaskEntryInput, TaskEntryOutput};

#[relm4::component(pub)]
impl Component for TaskEntryModel {
	type CommandOutput = ();
	type Input = TaskEntryInput;
	type Output = TaskEntryOutput;
	type Init = Option<List>;

	view! {
		#[root]
		gtk::Box {
			set_orientation: gtk::Orientation::Horizontal,
			set_margin_all: 12,
			set_spacing: 5,
			#[name(entry)]
			gtk::Entry {
				set_buffer: &model.buffer,
				set_hexpand: true,
				#[watch]
				set_visible: true,
				set_icon_from_icon_name: (gtk::EntryIconPosition::Primary, Some("value-increase-symbolic")),
				set_placeholder_text: Some(fl!("new-task")),
				set_height_request: 42,
			},
			gtk::Button {
				set_tooltip_text: Some(fl!("more-details")),
				set_icon_name: "text-editor-symbolic",
				connect_clicked[sender] => move |_| {
					sender.input(TaskEntryInput::EnterCreationMode);
				}
			},
			gtk::Button {
				set_icon_name: "mail-send-symbolic",
				connect_clicked[sender] => move |_| {
					sender.input(TaskEntryInput::AddTask);
				}
			}
		}
	}

	fn init(
		init: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = TaskEntryModel {
			task: Task::new(String::new(), String::new()),
			parent_list: init,
			buffer: gtk::EntryBuffer::new(None),
		};

		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(
		&mut self,
		message: Self::Input,
		sender: ComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			TaskEntryInput::CleanTaskEntry => {
				self.task = Task::new(String::new(), String::new());
				self.buffer.set_text("");
			},
			TaskEntryInput::EnterCreationMode => {
				self.task.title = self.buffer.text();
				sender
					.output(TaskEntryOutput::EnterCreationMode(self.task.clone()))
					.unwrap()
			},
			TaskEntryInput::AddTask => {
				self.task.title = self.buffer.text();
				if !self.task.title.is_empty() && self.parent_list.is_some() {
					self.task.parent = self.parent_list.as_ref().unwrap().id.clone();
					sender
						.output(TaskEntryOutput::AddTask(self.task.clone()))
						.unwrap_or_default();
					self.task = Task::new(
						String::new(),
						self.parent_list.as_ref().unwrap().id.clone(),
					);
					self.buffer.set_text("");
				}
			},
			TaskEntryInput::SetParentList(list) => self.parent_list = list,
		}
	}
}
