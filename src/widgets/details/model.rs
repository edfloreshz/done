use done_provider::Task;
use relm4::{factory::FactoryVecDeque, prelude::DynamicIndex};

use super::{sub_tasks::model::SubTaskModel, tags::factory::TagModel};

pub struct TaskDetailsFactoryModel {
	pub original_task: Task,
	pub task: Task,
	pub task_details_index: DynamicIndex,
	pub update: bool,
	pub selected_due_date: Option<String>,
	pub selected_reminder_date: Option<String>,
	pub sub_tasks: FactoryVecDeque<SubTaskModel>,
	pub tags: FactoryVecDeque<TagModel>,
	pub dirty: bool,
}

#[derive(derive_new::new, Debug)]
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
