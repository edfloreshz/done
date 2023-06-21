use core_done::service::Service;

use super::model::SidebarList;

#[derive(Debug)]
pub enum SidebarComponentInput {
	ToggleExtended(bool),
	SelectList(SidebarList),
	AddTaskListToSidebar(String, Service),
	Notify(String),
	OpenPreferences,
	ReloadSidebar,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SidebarComponentOutput {
	Notify(String, u32),
	DisablePlugin,
	SelectList(SidebarList, Service),
	OpenPreferences,
}
