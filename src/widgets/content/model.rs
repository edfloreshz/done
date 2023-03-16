use done_provider::{List, Task};
use relm4::{
	component::AsyncController, factory::AsyncFactoryVecDeque, Controller,
};

use crate::{
	application::plugin::Plugin,
	widgets::{
		details::model::TaskDetailsFactoryModel,
		smart_lists::{sidebar::model::SmartList, widget::SmartListContainerModel},
		task_entry::model::TaskEntryModel,
	},
};

pub struct ContentModel {
	pub task_factory: AsyncFactoryVecDeque<TaskModel>,
	pub task_details_factory: AsyncFactoryVecDeque<TaskDetailsFactoryModel>,
	pub task_entry: Controller<TaskEntryModel>,
	pub smart_lists: AsyncController<SmartListContainerModel>,
	pub plugin: Option<Plugin>,
	pub parent_list: Option<List>,
	pub selected_smart_list: Option<SmartList>,
	pub compact: bool,
	pub selected_task: Option<Task>,
	pub show_task_details: bool,
}

#[derive(Debug, Clone)]
pub struct TaskModel {
	pub task: Task,
	pub parent_list: List,
	pub compact: bool,
}

#[derive(derive_new::new)]
pub struct TaskInit {
	pub task: Task,
	pub parent_list: List,
	pub compact: bool,
}
