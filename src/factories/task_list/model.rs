use done_local_storage::services::Service;
use relm4::{prelude::DynamicIndex, Controller};

use crate::widgets::{
	delete::DeleteComponent, list_dialog::model::ListDialogComponent,
	sidebar::model::SidebarList,
};

#[derive(Debug, derive_new::new)]
pub struct TaskListFactoryModel {
	pub service: Option<Service>,
	pub index: DynamicIndex,
	pub list: SidebarList,
	pub extended: bool,
	pub smart: bool,
	pub rename: Controller<ListDialogComponent>,
	pub delete: Controller<DeleteComponent>,
}

#[derive(Debug, derive_new::new)]
pub struct TaskListFactoryInit {
	pub service: Option<Service>,
	pub list: SidebarList,
	pub smart: bool,
}
