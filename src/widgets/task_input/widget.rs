use crate::{fl, widgets::sidebar::model::SidebarList};
use adw::traits::{EntryRowExt, PreferencesRowExt};
use done_local_storage::models::Task;
use gtk::traits::{EditableExt, ListBoxRowExt};
use relm4::{
	adw, gtk,
	gtk::prelude::{ButtonExt, WidgetExt},
	Component, ComponentParts, ComponentSender, RelmWidgetExt,
};
use relm4_icons::icon_name;

use crate::widgets::task_input::model::TaskInputModel;

use super::messages::{TaskInputInput, TaskInputOutput};

#[relm4::component(pub)]
impl Component for TaskInputModel {
	type CommandOutput = ();
	type Input = TaskInputInput;
	type Output = TaskInputOutput;
	type Init = Option<SidebarList>;

	view! {
		#[root]
		adw::EntryRow {
			#[watch]
			set_visible: matches!(model.parent_list.as_ref(), Some(SidebarList::Custom(_))),
			set_hexpand: true,
			add_css_class: "card",
			set_title: fl!("new-task"),
			set_margin_all: 5,
			set_height_request: 42,
			set_show_apply_button: true,
			set_enable_emoji_completion: true,
			add_suffix = &gtk::Button {
				set_tooltip: fl!("more-details"),
				add_css_class: "suggested-action",
				add_css_class: "circular",
				set_icon_name: icon_name::PENCIL_AND_PAPER,
				set_valign: gtk::Align::Center,
				connect_clicked[sender] => move |_| {
					sender.input(TaskInputInput::EnterCreationMode);
				}
			},
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
		root: &Self::Root,
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
			TaskInputInput::EnterCreationMode => sender
				.output(TaskInputOutput::EnterCreationMode(self.task.clone()))
				.unwrap(),
			TaskInputInput::Rename(title) => {
				self.task.title = title;
			},
			TaskInputInput::AddTask => {
				if !self.task.title.is_empty() && self.parent_list.is_some() {
					if let SidebarList::Custom(list) = self.parent_list.as_ref().unwrap()
					{
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
