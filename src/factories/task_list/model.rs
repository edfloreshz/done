use relm4::{prelude::DynamicIndex, Controller};

use crate::widgets::{
	delete::DeleteComponent, list_dialog::model::ListDialogComponent,
	sidebar::model::SidebarList,
};

#[derive(Debug, derive_new::new)]
pub struct TaskListFactoryModel {
	pub index: DynamicIndex,
	pub list: SidebarList,
	pub extended: bool,
	pub smart: bool,
	pub rename: Controller<ListDialogComponent>,
	pub delete: Controller<DeleteComponent>,
}

#[derive(Debug, derive_new::new)]
pub struct TaskListFactoryInit {
	pub list: SidebarList,
	pub smart: bool,
}
