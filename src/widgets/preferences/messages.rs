use relm4::prelude::DynamicIndex;

use crate::application::plugin::Plugin;

use super::model::ColorScheme;

#[derive(Debug)]
pub enum PreferencesComponentInput {
	EnablePlugin(DynamicIndex, Plugin),
	DisablePlugin(DynamicIndex, Plugin),
	InstallPlugin(DynamicIndex, Plugin),
	RemovePlugin(DynamicIndex, Plugin),
	UpdatePlugin(DynamicIndex, Plugin),
	SetColorScheme(ColorScheme),
	ToggleCompact(bool),
}

#[derive(Debug)]
pub enum PreferencesComponentOutput {
	EnablePluginOnSidebar(Plugin),
	AddPluginToSidebar(Plugin),
	DisablePluginOnSidebar(Plugin),
	RemovePluginFromSidebar(Plugin),
	ToggleCompact(bool),
}
