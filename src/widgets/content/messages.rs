use done_local_storage::{models::task::Task, service::Service};
use relm4::prelude::DynamicIndex;

use crate::widgets::sidebar::model::SidebarList;

#[derive(Debug)]
pub enum ContentInput {
	AddTask(Task),
	RemoveTask(DynamicIndex),
	UpdateTask(Task),
	SelectList(SidebarList, Option<Service>),
	RevealTaskDetails(Option<DynamicIndex>, Task),
	DisablePlugin,
	CleanTaskEntry,
	HideFlap,
	Refresh,
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
	RevealTaskDetails(Option<DynamicIndex>),
}

#[derive(Debug)]
pub enum TaskOutput {
	Remove(DynamicIndex),
	UpdateTask(Option<DynamicIndex>, Task),
	RevealTaskDetails(Option<DynamicIndex>, Task),
}
