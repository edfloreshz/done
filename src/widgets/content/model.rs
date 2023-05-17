use done_local_storage::models::{List, Task};
use relm4::{factory::AsyncFactoryVecDeque, Controller};

use crate::factories::details::model::TaskDetailsFactoryModel;
use crate::factories::task::model::TaskModel;
use crate::widgets::task_entry::model::TaskEntryModel;

pub struct ContentModel {
	pub task_factory: AsyncFactoryVecDeque<TaskModel>,
	pub task_details_factory: AsyncFactoryVecDeque<TaskDetailsFactoryModel>,
	pub task_entry: Controller<TaskEntryModel>,
	pub parent_list: Option<List>,
	pub compact: bool,
	pub selected_task: Option<Task>,
	pub show_task_details: bool,
}
