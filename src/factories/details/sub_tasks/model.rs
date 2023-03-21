use done_provider::SubTask;
use relm4::prelude::DynamicIndex;

pub struct SubTaskModel {
	pub sub_task: SubTask,
	pub index: DynamicIndex,
}

#[derive(derive_new::new)]
pub struct SubTaskInit {
	pub sub_task: SubTask,
}
