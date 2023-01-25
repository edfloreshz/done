use relm4::prelude::DynamicIndex;

use crate::application::plugin::Plugin;

use super::model::UpdateStatus;

#[derive(Debug)]
pub enum ServiceRowInput {
	InstallPlugin(DynamicIndex),
	EnablePlugin(DynamicIndex),
	DisablePlugin(DynamicIndex),
	RemovePlugin(DynamicIndex),
	UpdatePlugin(DynamicIndex),
	ShowInstallButton(bool),
	SwitchOn(bool),
	ToggleSwitch(DynamicIndex, bool),
	InformStatus(UpdateStatus),
	UpdateChildId(usize),
}

#[derive(Debug)]
pub enum ServiceRowOutput {
	InstallPlugin(DynamicIndex, Plugin),
	EnablePlugin(DynamicIndex, Plugin),
	DisablePlugin(DynamicIndex, Plugin, usize),
	RemovePlugin(DynamicIndex, Plugin, usize),
	UpdatePlugin(DynamicIndex, Plugin, usize),
}
