use relm4::prelude::DynamicIndex;

use done_core::models::task::Task;

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
