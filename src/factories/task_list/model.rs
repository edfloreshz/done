use done_local_storage::models::List;
use relm4::gtk;

#[derive(Debug, Clone, PartialEq, derive_new::new)]
pub struct TaskListFactoryModel {
	pub list: List,
	pub entry: gtk::EntryBuffer,
	pub extended: bool,
}

#[derive(Debug, derive_new::new)]
pub struct TaskListFactoryInit {
	pub list: List,
}
