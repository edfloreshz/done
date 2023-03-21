use relm4::prelude::DynamicIndex;

use super::model::TaskListFactoryInit;

#[derive(Debug)]
pub enum TaskListFactoryInput {
	Select,
	EditMode,
	Delete(DynamicIndex),
	Rename,
	ChangeIcon(String),
}

#[derive(Debug)]
pub enum TaskListFactoryOutput {
	Select(Box<TaskListFactoryInit>),
	DeleteTaskList(DynamicIndex, String),
	Forward,
	Notify(String),
}
