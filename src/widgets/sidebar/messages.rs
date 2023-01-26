use crate::{
	application::plugin::Plugin,
	widgets::{
		smart_lists::sidebar::model::SmartList, task_list::model::ListFactoryModel,
	},
};

#[derive(Debug)]
pub enum SidebarComponentInput {
	AddListToProvider(usize, Plugin, String),
	ListSelected(ListFactoryModel),
	EnableService(Plugin),
	DisableService(Plugin),
	RemoveService(Plugin),
	AddPluginToSidebar(Plugin),
	Forward,
	Notify(String),
	SelectSmartList(SmartList),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SidebarComponentOutput {
	ListSelected(Box<ListFactoryModel>),
	Forward,
	Notify(String, u32),
	DisablePlugin,
	SelectSmartList(SmartList),
}
