use done_provider::List;
use relm4::gtk;

use crate::application::plugin::Plugin;

#[derive(Debug, Clone, PartialEq, derive_new::new)]
pub struct TaskListFactoryModel {
	pub list: List,
	pub plugin: Plugin,
	pub entry: gtk::EntryBuffer,
	pub edit_mode: bool,
}

#[derive(Debug, derive_new::new)]
pub struct TaskListFactoryInit {
	pub plugin: Plugin,
	pub list: List,
}
