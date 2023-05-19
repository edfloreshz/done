use done_local_storage::models::Task;

use crate::widgets::sidebar::model::SidebarList;

#[derive(Debug)]
pub enum TaskInputInput {
	SetParentList(Option<SidebarList>),
	AddTask,
	Rename(String),
	EnterCreationMode,
	CleanTaskEntry,
}

#[derive(Debug)]
pub enum TaskInputOutput {
	AddTask(Task),
	EnterCreationMode(Task),
}
