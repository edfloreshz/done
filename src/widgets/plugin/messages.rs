use done_provider::List;
use relm4::prelude::DynamicIndex;

use crate::{
	application::plugin::Plugin, widgets::task_list::model::ListFactoryModel,
};

#[derive(Debug)]
pub enum PluginFactoryInput {
	FillTaskFactory,
	RequestAddList(usize, String),
	AddList(List),
	DeleteTaskList(DynamicIndex, String),
	Forward,
	ListSelected(ListFactoryModel),
	Notify(String),
	Enable,
	Disable,
}

#[derive(Debug)]
pub enum PluginFactoryOutput {
	AddListToProvider(usize, Plugin, String),
	ListSelected(ListFactoryModel),
	Notify(String),
	Forward,
}
