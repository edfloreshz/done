use done_local_storage::services::Service;
use relm4::prelude::DynamicIndex;

use super::model::SidebarList;

#[derive(Debug)]
pub enum SidebarComponentInput {
	ToggleExtended(bool),
	SelectList(SidebarList),
	AddTaskListToSidebar(String, Service),
	DeleteTaskList(DynamicIndex, String, Service),
	Notify(String),
	OpenPreferences,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SidebarComponentOutput {
	Notify(String, u32),
	DisablePlugin,
	SelectList(SidebarList, Option<Service>),
	OpenPreferences,
}
