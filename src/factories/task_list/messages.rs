use relm4::prelude::DynamicIndex;

use crate::widgets::sidebar::model::SidebarList;

#[derive(Debug)]
pub enum TaskListFactoryInput {
	Select,
	Delete,
	RenameList(String),
	ChangeIcon(String),
	ToggleExtended(bool),
}

#[derive(Debug)]
pub enum TaskListFactoryOutput {
	Select(SidebarList),
	DeleteTaskList(DynamicIndex, String),
	Notify(String),
}
