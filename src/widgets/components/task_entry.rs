use crate::fl;
use chrono::NaiveDateTime;
use proto_rust::provider::{List, Task};
use relm4::{
	gtk,
	gtk::prelude::{
		BoxExt, ButtonExt, EditableExt, EntryBufferExtManual, EntryExt,
		OrientableExt, WidgetExt,
	},
	Component, ComponentParts, ComponentSender, RelmWidgetExt,
};

#[derive(Debug)]
pub struct TaskEntryComponent {
	task: Task,
	parent_list: Option<List>,
}

#[derive(Debug)]
pub enum TaskEntryComponentInput {
	AddToMyDay,
	SetTitle(String),
	SetReminder(NaiveDateTime),
	SetDueDate(NaiveDateTime),
	AddNote(String),
	AddTask,
	SetParentList(Option<List>),
}

#[derive(Debug)]
pub enum TaskEntryComponentOutput {
	AddTask(Task),
}

#[relm4::component(pub)]
impl Component for TaskEntryComponent {
	type CommandOutput = ();
	type Input = TaskEntryComponentInput;
	type Output = TaskEntryComponentOutput;
	type Init = Option<List>;

	view! {
		#[root]
		gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			gtk::Box {
				set_orientation: gtk::Orientation::Horizontal,
				set_margin_end: 12,
				set_margin_start: 12,
				set_margin_top: 12,
				set_spacing: 5,
				set_halign: gtk::Align::Center,
				gtk::Button {
					set_tooltip_text: Some(fl!("add-to-today")),
					set_icon_name: "daytime-sunrise-symbolic",
					connect_clicked[sender] => move |_| {
						sender.input(TaskEntryComponentInput::AddToMyDay);
					}
				},
				gtk::Button {
					set_tooltip_text: Some(fl!("set-time")),
					set_icon_name: "appointment-soon-symbolic",
					connect_clicked[sender] => move |_| {
						sender.input(TaskEntryComponentInput::SetReminder(chrono::Utc::now().naive_utc()));
					}
				},
				gtk::Button {
					set_tooltip_text: Some(fl!("set-due-date")),
					set_icon_name: "office-calendar-symbolic",
					connect_clicked[sender] => move |_| {
						sender.input(TaskEntryComponentInput::SetDueDate(chrono::Utc::now().naive_utc()));
					}
				},
				gtk::Button {
					set_tooltip_text: Some(fl!("more-details")),
					set_icon_name: "text-editor-symbolic",
					connect_clicked[sender] => move |_| {
						sender.input(TaskEntryComponentInput::AddNote(String::new()));
					}
				},
			},
			gtk::Box {
				set_orientation: gtk::Orientation::Horizontal,
				set_margin_all: 12,
				set_spacing: 5,
				#[name(entry)]
				gtk::Entry {
					set_hexpand: true,
					#[watch]
					set_visible: true,
					set_icon_from_icon_name: (gtk::EntryIconPosition::Primary, Some("value-increase-symbolic")),
					set_placeholder_text: Some(fl!("new-task")),
					set_height_request: 42,
					connect_changed[sender] => move |entry| {
						let buffer = entry.buffer();
						sender.input(TaskEntryComponentInput::SetTitle(buffer.text()));
					},
					connect_activate[sender] => move |entry| {
						let buffer = entry.buffer();
						sender.input(TaskEntryComponentInput::SetTitle(buffer.text()));
						sender.input(TaskEntryComponentInput::AddTask);
					}
				},
				gtk::Button {
					set_icon_name: "mail-send-symbolic",
					connect_clicked[sender] => move |_| {
						sender.input(TaskEntryComponentInput::AddTask);
					}
				}
			}
		}
	}

	fn init(
		init: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = TaskEntryComponent {
			task: Task::new(String::new(), String::new()),
			parent_list: init,
		};

		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update_with_view(
		&mut self,
		widgets: &mut Self::Widgets,
		message: Self::Input,
		sender: ComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			TaskEntryComponentInput::AddToMyDay => (), // TODO: Add to my day.
			TaskEntryComponentInput::SetTitle(title) => self.task.title = title,
			TaskEntryComponentInput::SetReminder(reminder) => {
				self.task.reminder_date = Some(reminder.timestamp());
				self.task.is_reminder_on = true;
			},
			TaskEntryComponentInput::SetDueDate(due) => {
				self.task.due_date = Some(due.timestamp());
			},
			TaskEntryComponentInput::AddNote(note) => self.task.body = Some(note),
			TaskEntryComponentInput::AddTask => {
				if !self.task.title.is_empty() && self.parent_list.is_some() {
					self.task.parent = self.parent_list.as_ref().unwrap().id.clone();
					sender
						.output(TaskEntryComponentOutput::AddTask(self.task.clone()))
						.unwrap_or_default();
					self.task = Task::new(
						String::new(),
						self.parent_list.as_ref().unwrap().id.clone(),
					);
					widgets.entry.set_text("");
				}
			},
			TaskEntryComponentInput::SetParentList(list) => self.parent_list = list,
		}
	}
}
