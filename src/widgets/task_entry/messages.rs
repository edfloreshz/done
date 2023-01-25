use proto_rust::{List, Task};

#[derive(Debug)]
pub enum TaskEntryInput {
	SetParentList(Option<List>),
	AddTask,
	EnterCreationMode,
	CleanTaskEntry,
}

#[derive(Debug)]
pub enum TaskEntryOutput {
	AddTask(Task),
	EnterCreationMode(Task),
}
