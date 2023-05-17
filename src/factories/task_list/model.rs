use relm4::{prelude::DynamicIndex, Controller};

use crate::widgets::{
	delete::DeleteComponent, sidebar::model::SidebarList,
	task_list_entry::model::TaskListEntryComponent,
};

#[derive(Debug, derive_new::new)]
pub struct TaskListFactoryModel {
	pub index: DynamicIndex,
	pub list: SidebarList,
	pub extended: bool,
	pub smart: bool,
	pub rename: Controller<TaskListEntryComponent>,
	pub delete: Controller<DeleteComponent>,
}

#[derive(Debug, derive_new::new)]
pub struct TaskListFactoryInit {
	pub list: SidebarList,
	pub smart: bool,
}
