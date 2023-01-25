use proto_rust::Task;
use relm4::prelude::DynamicIndex;

pub struct TaskDetailsFactoryModel {
	pub original_task: Task,
	pub task: Task,
	pub task_details_index: DynamicIndex,
	pub update: bool,
	pub selected_due_date: Option<String>,
	pub selected_reminder_date: Option<String>,
	pub dirty: bool,
}

#[derive(derive_new::new)]
pub struct TaskDetailsFactoryInit {
	pub task: Task,
	pub index: Option<DynamicIndex>,
}

#[derive(Debug)]
pub enum DateTpe {
	Reminder,
	DueDate,
}

#[derive(Debug)]
pub enum DateDay {
	Today,
	Tomorrow,
	None,
}
