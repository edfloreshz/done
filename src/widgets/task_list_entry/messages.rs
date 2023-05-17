#[derive(Debug)]
pub enum TaskListEntryInput {
	HandleEntry,
}

#[derive(Debug)]
pub enum TaskListEntryOutput {
	AddTaskListToSidebar(String),
	RenameList(String),
}
