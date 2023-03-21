use relm4::prelude::DynamicIndex;

use super::model::TaskListFactoryModel;

#[derive(Debug)]
pub enum TaskListFactoryInput {
	Select,
	Delete(DynamicIndex),
	Rename(String),
	ChangeIcon(String),
}

#[derive(Debug)]
pub enum TaskListFactoryOutput {
	Select(Box<TaskListFactoryModel>),
	DeleteTaskList(DynamicIndex, String),
	Forward,
	Notify(String),
}
