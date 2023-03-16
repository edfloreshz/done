use done_provider::Task;
use relm4::prelude::DynamicIndex;

use crate::widgets::{
	smart_lists::sidebar::model::SmartList, task_list::model::ListFactoryModel,
};

#[derive(Debug)]
pub enum ContentInput {
	AddTask(Task),
	RemoveTask(DynamicIndex),
	UpdateTask(Task),
	TaskListSelected(ListFactoryModel),
	SelectSmartList(SmartList),
	RevealTaskDetails(Option<DynamicIndex>, Task),
	ToggleCompact(bool),
	DisablePlugin,
	CleanTaskEntry,
	HideFlap,
}

#[derive(Debug)]
pub enum ContentOutput {
	Notify(String, u32),
}

#[derive(Debug)]
pub enum TaskInput {
	SetCompleted(bool),
	Favorite(DynamicIndex),
	ModifyTitle(String),
	ToggleCompact(bool),
	RevealTaskDetails(Option<DynamicIndex>),
}

#[derive(Debug)]
pub enum TaskOutput {
	Remove(DynamicIndex),
	UpdateTask(Option<DynamicIndex>, Task),
	RevealTaskDetails(Option<DynamicIndex>, Task),
}
