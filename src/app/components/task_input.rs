use crate::{app::models::sidebar_list::SidebarList, fl};
use adw::prelude::{EntryRowExt, PreferencesRowExt};
use core_done::models::task::Task;
use gtk::prelude::{EditableExt, ListBoxRowExt};
use relm4::{
	adw, gtk, gtk::prelude::WidgetExt, Component, ComponentParts,
	ComponentSender, RelmWidgetExt,
};

#[derive(Debug)]
pub struct TaskInputModel {
	pub task: Task,
	pub parent_list: SidebarList,
	pub buffer: gtk::EntryBuffer,
}

#[derive(Debug)]
pub enum TaskInputInput {
	SetParentList(SidebarList),
	AddTask,
	Rename(String),
	CleanTaskEntry,
}

#[derive(Debug)]
pub enum TaskInputOutput {
	AddTask(Task),
}

#[relm4::component(pub)]
impl Component for TaskInputModel {
	type CommandOutput = ();
	type Input = TaskInputInput;
	type Output = TaskInputOutput;
	type Init = SidebarList;

	view! {
		#[root]
		adw::EntryRow {
			#[watch]
			set_visible: matches!(model.parent_list, SidebarList::Custom(_)),
			set_hexpand: true,
			add_css_class: "card",
			set_title: fl!("new-task"),
			set_margin_all: 5,
			set_height_request: 42,
			set_show_apply_button: true,
			set_enable_emoji_completion: true,
			connect_apply[sender] => move |_| {
				sender.input(TaskInputInput::AddTask);
			},
			connect_activate[sender] => move |_| {
				sender.input(TaskInputInput::AddTask);
			},
			connect_changed[sender] => move |entry| {
				let text = entry.text().to_string();
				sender.input(TaskInputInput::Rename(text));
			},
		}
	}

	fn init(
		init: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = TaskInputModel {
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
			TaskInputInput::CleanTaskEntry => {
				self.task = Task::new(String::new(), String::new());
				root.set_text("");
			},
			TaskInputInput::Rename(title) => {
				self.task.title = title;
			},
			TaskInputInput::AddTask => {
				if !self.task.title.is_empty() {
					if let SidebarList::Custom(list) = &self.parent_list {
						self.task.parent = list.id.clone();
						sender
							.output(TaskInputOutput::AddTask(self.task.clone()))
							.unwrap_or_default();
						self.task = Task::new(String::new(), list.id.clone());
						sender.input(TaskInputInput::CleanTaskEntry);
					}
				}
			},
			TaskInputInput::SetParentList(list) => {
				self.parent_list = list;
			},
		}
	}
}
