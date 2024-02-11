use core_done::models::task::Task;
use relm4::prelude::DynamicIndex;

#[derive(Debug)]
pub struct SubTaskModel {
	pub sub_task: Task,
	pub index: DynamicIndex,
}

#[derive(derive_new::new)]
pub struct SubTaskInit {
	pub sub_task: Task,
}
