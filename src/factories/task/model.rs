use done_local_storage::models::{list::List, task::Task};

#[derive(Debug, Clone)]
pub struct TaskModel {
	pub task: Task,
	pub parent_list: List,
}

#[derive(derive_new::new)]
pub struct TaskInit {
	pub task: Task,
	pub parent_list: List,
}
