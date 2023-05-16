use relm4::prelude::DynamicIndex;

use super::model::TaskListFactoryInit;

#[derive(Debug)]
pub enum TaskListFactoryInput {
	Select,
	Rename,
	OpenRightClickMenu,
	Delete(DynamicIndex),
	ChangeIcon(String),
	ToggleExtended(bool),
}

#[derive(Debug)]
pub enum TaskListFactoryOutput {
	Select(Box<TaskListFactoryInit>),
	DeleteTaskList(DynamicIndex, String),
	Notify(String),
}
