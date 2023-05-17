use done_local_storage::models::Task;
use relm4::prelude::DynamicIndex;

use crate::widgets::sidebar::model::SidebarList;

#[derive(Debug)]
pub enum ContentInput {
	AddTask(Task),
	RemoveTask(DynamicIndex),
	UpdateTask(Task),
	SelectList(SidebarList),
	RevealTaskDetails(Option<DynamicIndex>, Task),
	ToggleCompact(bool),
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
	ToggleCompact(bool),
	RevealTaskDetails(Option<DynamicIndex>),
}

#[derive(Debug)]
pub enum TaskOutput {
	Remove(DynamicIndex),
	UpdateTask(Option<DynamicIndex>, Task),
	RevealTaskDetails(Option<DynamicIndex>, Task),
}
