#[derive(Debug)]
pub enum ListDialogInput {
	HandleEntry,
}

#[derive(Debug)]
pub enum ListDialogOutput {
	AddTaskListToSidebar(String),
	RenameList(String),
}
