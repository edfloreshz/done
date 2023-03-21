use crate::application::plugin::Plugin;

#[derive(Debug)]
pub struct PluginFactoryModel {
	pub plugin: Plugin,
	pub enabled: bool,
}

#[derive(derive_new::new)]
pub struct PluginFactoryInit {
	pub plugin: Plugin,
	pub enabled: bool,
}
