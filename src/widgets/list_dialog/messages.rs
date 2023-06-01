use done_local_storage::services::Service;

#[derive(Debug)]
pub enum ListDialogInput {
	HandleEntry,
	UpdateService(u32),
}

#[derive(Debug)]
pub enum ListDialogOutput {
	AddTaskListToSidebar(String, Service),
	RenameList(String, Service),
}
