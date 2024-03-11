use relm4::prelude::DynamicIndex;

use done_core::models::task::Task;

#[derive(Debug)]
pub struct SubTaskModel {
	pub sub_task: Task,
	pub index: DynamicIndex,
}

#[derive(derive_new::new)]
pub struct SubTaskInit {
	pub sub_task: Task,
}
