use relm4::{factory::AsyncFactoryVecDeque, Controller};

use crate::widgets::{
	plugin::model::PluginFactoryModel,
	smart_lists::sidebar::model::SmartSidebarListModel,
};

#[derive(Debug)]
pub struct SidebarComponentModel {
	pub plugin_factory: AsyncFactoryVecDeque<PluginFactoryModel>,
	pub smart_list_controller: Controller<SmartSidebarListModel>,
	pub is_sidebar_empty: bool,
}
