use done_provider::ProviderResponse;
use relm4::tokio::sync::mpsc::Receiver;
use relm4::{factory::AsyncFactoryVecDeque, Controller};

use crate::widgets::task_list::model::ListFactoryModel;
use crate::{application::plugin::Plugin, widgets::list_entry::ListEntryModel};

#[derive(Debug)]
pub struct PluginFactoryModel {
	pub plugin: Plugin,
	pub enabled: bool,
	pub last_list_selected: Option<ListFactoryModel>,
	pub list_factory: AsyncFactoryVecDeque<ListFactoryModel>,
	pub new_list_controller: Controller<ListEntryModel>,
	pub rx: Receiver<ProviderResponse>,
}

#[derive(derive_new::new)]
pub struct PluginFactoryInit {
	pub plugin: Plugin,
	pub enabled: bool,
}
