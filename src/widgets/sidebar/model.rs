use relm4::factory::AsyncFactoryVecDeque;

use crate::factories::plugin::model::PluginFactoryModel;

#[derive(Debug)]
pub struct SidebarComponentModel {
	pub plugin_factory: AsyncFactoryVecDeque<PluginFactoryModel>,
}
