use done_provider::SubTask;
use relm4::prelude::DynamicIndex;

#[derive(Debug)]
pub enum SubTaskInput {
	SetCompleted(DynamicIndex, bool),
	ModifyTitle(DynamicIndex, String),
	Remove(DynamicIndex),
}

#[derive(Debug)]
pub enum SubTaskOutput {
	Update(DynamicIndex, SubTask),
	Remove(DynamicIndex),
}
