use crate::fl;
use adw::traits::{EntryRowExt, PreferencesRowExt};
use done_provider::provider::{List, Task};
use gtk::traits::{EditableExt, ListBoxRowExt};
use relm4::{
	adw, gtk,
	gtk::prelude::{ButtonExt, WidgetExt},
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
		adw::EntryRow {
			#[watch]
			set_visible: true,
			set_hexpand: true,
			add_css_class: "card",
			set_title: fl!("new-task"),
			set_margin_all: 12,
			set_height_request: 42,
			set_show_apply_button: true,
			set_enable_emoji_completion: true,
			add_suffix = &gtk::Button {
				set_tooltip_text: Some(fl!("more-details")),
				add_css_class: "suggested-action",
				add_css_class: "circular",
				set_icon_name: "text-editor-symbolic",
				set_valign: gtk::Align::Center,
				connect_clicked[sender] => move |_| {
					sender.input(TaskEntryInput::EnterCreationMode);
				}
			},
			connect_apply[sender] => move |_| {
				sender.input(TaskEntryInput::AddTask);
			},
			connect_activate[sender] => move |_| {
				sender.input(TaskEntryInput::AddTask);
			},
			connect_changed[sender] => move |entry| {
				let text = entry.text().to_string();
				sender.input(TaskEntryInput::Rename(text));
			},
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
			buffer: gtk::EntryBuffer::new(None::<String>),
		};

		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(
		&mut self,
		message: Self::Input,
		sender: ComponentSender<Self>,
		root: &Self::Root,
	) {
		match message {
			TaskEntryInput::CleanTaskEntry => {
				self.task = Task::new(String::new(), String::new());
				root.set_text("");
			},
			TaskEntryInput::EnterCreationMode => sender
				.output(TaskEntryOutput::EnterCreationMode(self.task.clone()))
				.unwrap(),
			TaskEntryInput::Rename(title) => {
				self.task.title = title;
			},
			TaskEntryInput::AddTask => {
				if !self.task.title.is_empty() && self.parent_list.is_some() {
					self.task.parent = self.parent_list.as_ref().unwrap().id.clone();
					sender
						.output(TaskEntryOutput::AddTask(self.task.clone()))
						.unwrap_or_default();
					self.task = Task::new(
						String::new(),
						self.parent_list.as_ref().unwrap().id.clone(),
					);
					sender.input(TaskEntryInput::CleanTaskEntry);
				}
			},
			TaskEntryInput::SetParentList(list) => self.parent_list = list,
		}
	}
}
