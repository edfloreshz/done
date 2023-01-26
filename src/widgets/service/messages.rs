use relm4::prelude::DynamicIndex;

use crate::application::plugin::Plugin;

use super::model::UpdateStatus;

#[derive(Debug)]
pub enum ServiceInput {
	InstallPlugin(DynamicIndex),
	EnablePlugin(DynamicIndex),
	DisablePlugin(DynamicIndex),
	RemovePlugin(DynamicIndex),
	UpdatePlugin(DynamicIndex),
	ShowInstallButton(bool),
	SwitchOn(bool),
	ToggleSwitch(DynamicIndex, bool),
	InformStatus(UpdateStatus),
}

#[derive(Debug)]
pub enum ServiceOutput {
	Install(DynamicIndex, Plugin),
	Uninstall(DynamicIndex, Plugin),
	Update(DynamicIndex, Plugin),
	Enable(DynamicIndex, Plugin),
	Disable(DynamicIndex, Plugin),
}
