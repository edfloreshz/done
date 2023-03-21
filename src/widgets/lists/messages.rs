use relm4::prelude::DynamicIndex;

use crate::{
	application::plugin::Plugin,
	factories::task_list::model::TaskListFactoryModel,
};

#[derive(Debug)]
pub enum TaskListsInput {
	PluginSelected(Plugin),
	SmartListSelected,
	AddTaskList(String),
	Forward,
	Notify(String),
	ListSelected(Box<TaskListFactoryModel>),
	DeleteTaskList(DynamicIndex, String),
}

#[derive(Debug)]
pub enum TaskListsOutput {
	Forward,
	Notify(String),
	ListSelected(Box<TaskListFactoryModel>),
}
