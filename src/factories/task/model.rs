use done_provider::{List, Task};

#[derive(Debug, Clone)]
pub struct TaskModel {
	pub task: Task,
	pub parent_list: List,
	pub compact: bool,
}

#[derive(derive_new::new)]
pub struct TaskInit {
	pub task: Task,
	pub parent_list: List,
	pub compact: bool,
}
