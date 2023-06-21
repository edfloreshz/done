use core_done::service::Service;
use relm4::prelude::DynamicIndex;

use crate::widgets::sidebar::model::SidebarList;

#[derive(Debug)]
pub enum TaskListFactoryInput {
	Select,
	Delete,
	RenameList(String, Service),
	ChangeIcon(String),
	ToggleExtended(bool),
}

#[derive(Debug)]
pub enum TaskListFactoryOutput {
	Select(SidebarList),
	DeleteTaskList(DynamicIndex, String, Service),
	Notify(String),
}
