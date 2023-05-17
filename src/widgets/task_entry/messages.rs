use done_local_storage::models::Task;

use crate::widgets::sidebar::model::SidebarList;

#[derive(Debug)]
pub enum TaskEntryInput {
	SetParentList(Option<SidebarList>),
	AddTask,
	Rename(String),
	EnterCreationMode,
	CleanTaskEntry,
}

#[derive(Debug)]
pub enum TaskEntryOutput {
	AddTask(Task),
	EnterCreationMode(Task),
}
