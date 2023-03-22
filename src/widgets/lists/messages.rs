use relm4::prelude::DynamicIndex;

use crate::{
	application::plugin::Plugin, factories::task_list::model::TaskListFactoryInit,
};

#[derive(Debug)]
pub enum TaskListsInput {
	RemoveService(Plugin),
	PluginSelected(Plugin),
	AddTaskList(String),
	Forward,
	Notify(String),
	ListSelected(Box<TaskListFactoryInit>),
	DeleteTaskList(DynamicIndex, String),
}

#[derive(Debug)]
pub enum TaskListsOutput {
	Forward,
	Notify(String),
	ListSelected(Box<TaskListFactoryInit>),
}
