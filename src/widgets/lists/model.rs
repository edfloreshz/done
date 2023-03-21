use crate::application::plugin::Plugin;
use crate::factories::task_list::model::TaskListFactoryModel;
use crate::widgets::list_entry::ListEntryModel;
use relm4::factory::AsyncFactoryVecDeque;
use relm4::Controller;

pub struct TaskListsModel {
	pub plugin: Option<Plugin>,
	pub new_list_controller: Controller<ListEntryModel>,
	pub list_factory: AsyncFactoryVecDeque<TaskListFactoryModel>,
	pub show_pane: bool,
}
