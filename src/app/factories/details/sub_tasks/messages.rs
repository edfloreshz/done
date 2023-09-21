use core_done::models::task::Task;
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
