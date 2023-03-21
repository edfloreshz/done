use crate::application::plugin::Plugin;

#[derive(Debug)]
pub enum PluginFactoryInput {
	PluginSelected,
	Enable,
	Disable,
}

#[derive(Debug)]
pub enum PluginFactoryOutput {
	PluginSelected(Plugin),
}
