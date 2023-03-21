use done_provider::List;

use crate::application::plugin::Plugin;

#[derive(Debug, Clone, PartialEq, derive_new::new)]
pub struct TaskListFactoryModel {
	pub list: List,
	pub plugin: Plugin,
}

#[derive(derive_new::new)]
pub struct TaskListFactoryInit {
	pub plugin: Plugin,
	pub list: List,
}
