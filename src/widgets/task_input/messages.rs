use done_local_storage::models::task::Task;

use crate::widgets::sidebar::model::SidebarList;

#[derive(Debug)]
pub enum TaskInputInput {
	SetParentList(SidebarList),
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
