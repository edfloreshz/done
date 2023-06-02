use done_local_storage::models::task::Task;
use relm4::gtk;

use crate::widgets::sidebar::model::SidebarList;

#[derive(Debug)]
pub struct TaskInputModel {
	pub task: Task,
	pub parent_list: Option<SidebarList>,
	pub buffer: gtk::EntryBuffer,
}
