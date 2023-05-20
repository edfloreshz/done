use done_local_storage::models::Task;
use relm4::{factory::AsyncFactoryVecDeque, Controller};

use crate::factories::details::model::TaskDetailsFactoryModel;
use crate::factories::task::model::TaskModel;
use crate::widgets::sidebar::model::SidebarList;
use crate::widgets::task_input::model::TaskInputModel;

pub struct ContentModel {
	pub task_factory: AsyncFactoryVecDeque<TaskModel>,
	pub task_details_factory: AsyncFactoryVecDeque<TaskDetailsFactoryModel>,
	pub task_entry: Controller<TaskInputModel>,
	pub parent_list: Option<SidebarList>,
	pub icon: Option<String>,
	pub title: String,
	pub description: String,
	pub smart: bool,
	pub selected_task: Option<Task>,
	pub show_task_details: bool,
	pub page_icon: String,
	pub page_title: String,
	pub page_subtitle: String,
}
