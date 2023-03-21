use done_provider::{List, Task};
use relm4::{
	component::AsyncController, factory::AsyncFactoryVecDeque, Controller,
};

use crate::factories::task::model::TaskModel;
use crate::{
	application::plugin::Plugin,
	factories::{
		details::model::TaskDetailsFactoryModel, task_entry::model::TaskEntryModel,
	},
	widgets::smart_lists::{
		sidebar::model::SmartList, widget::SmartListContainerModel,
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
