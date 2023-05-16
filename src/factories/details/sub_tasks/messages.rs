use done_local_storage::models::Task;
use relm4::prelude::DynamicIndex;

#[derive(Debug)]
pub enum SubTaskInput {
	SetStatus(DynamicIndex, bool),
	ModifyTitle(DynamicIndex, String),
	Remove(DynamicIndex),
}

#[derive(Debug)]
pub enum SubTaskOutput {
	Update(DynamicIndex, Task),
	Remove(DynamicIndex),
}