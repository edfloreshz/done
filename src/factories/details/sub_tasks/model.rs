use done_local_storage::models::Task;
use relm4::prelude::DynamicIndex;

pub struct SubTaskModel {
	pub sub_task: Task,
	pub index: DynamicIndex,
}

#[derive(derive_new::new)]
pub struct SubTaskInit {
	pub sub_task: Task,
}
