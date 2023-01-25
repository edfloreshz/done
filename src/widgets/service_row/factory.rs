use crate::fl;
use crate::widgets::preferences::messages::PreferencesComponentInput;
use crate::widgets::preferences::model::PluginPreferences;
use adw::prelude::{ActionRowExt, ButtonExt, PreferencesRowExt};
use relm4::adw;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::gtk;
use relm4::gtk::traits::WidgetExt;

use super::messages::{ServiceRowInput, ServiceRowOutput};
use super::model::{ServiceRowModel, UpdateStatus};

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for ServiceRowModel {
	type ParentInput = PreferencesComponentInput;
	type ParentWidget = adw::PreferencesGroup;
	type CommandOutput = ();
	type Input = ServiceRowInput;
	type Output = ServiceRowOutput;
	type Init = PluginPreferences;

	view! {
		#[root]
		#[name(service)]
		adw::ActionRow {
				set_title: &self.plugin.name,
				set_subtitle: &self.plugin.description,
				add_suffix = &gtk::Button {
					#[watch]
					set_visible: self.installed,
					set_icon_name: "user-trash-full-symbolic",
					set_css_classes: &["destructive-action"],
					set_tooltip_text: Some(fl!("remove")),
					set_valign: gtk::Align::Center,
					connect_clicked[sender, index] => move |_| {
						sender.input(ServiceRowInput::RemovePlugin(index.clone()));
					}
				},
				add_suffix = &gtk::Button {
					#[watch]
					set_visible: self.update && self.installed,
					set_icon_name: "software-update-available-symbolic",
					set_css_classes: &["favorite"],
					set_tooltip_text: Some(fl!("update")),
					set_valign: gtk::Align::Center,
					connect_clicked[sender, index] => move |_| {
							sender.input(ServiceRowInput::UpdatePlugin(index.clone()));
					}
				},
				add_suffix = &gtk::Button {
					#[watch]
					set_visible: !self.installed,
					set_label: fl!("install"),
					set_valign: gtk::Align::Center,
					connect_clicked[sender, index] => move |_| {
							sender.input(ServiceRowInput::InstallPlugin(index.clone()));
					}
				},
				#[name(switch)]
				add_suffix = &gtk::Switch {
					set_valign: gtk::Align::Center,
					#[watch]
					set_visible: self.installed,
					connect_state_set[sender, index] => move |_, state| {
						sender.input(ServiceRowInput::ToggleSwitch(index.clone(), state));
						gtk::Inhibit(false)
					}
				}
		}
	}

	async fn init_model(
		plugin: Self::Init,
		_index: &DynamicIndex,
		_sender: AsyncFactorySender<Self>,
	) -> Self {
		Self {
			plugin: plugin.plugin,
			enabled: plugin.enabled,
			installed: plugin.installed,
			update: plugin.update,
			first_load: true,
			process_id: 0,
		}
	}

	fn init_widgets(
		&mut self,
		index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: AsyncFactorySender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		if self.enabled {
			widgets.switch.set_state(true);
		}
		widgets
	}

	async fn update_with_view(
		&mut self,
		widgets: &mut Self::Widgets,
		message: Self::Input,
		sender: AsyncFactorySender<Self>,
	) {
		match message {
			ServiceRowInput::UpdateChildId(id) => self.process_id = id,
			ServiceRowInput::ToggleSwitch(index, state) => {
				if state {
					sender.input(ServiceRowInput::EnablePlugin(index));
				} else {
					sender.input(ServiceRowInput::DisablePlugin(index));
				}
			},
			ServiceRowInput::InstallPlugin(index) => {
				sender
					.output(ServiceRowOutput::InstallPlugin(index, self.plugin.clone()));
			},
			ServiceRowInput::EnablePlugin(index) => {
				if !self.first_load {
					sender
						.output(ServiceRowOutput::EnablePlugin(index, self.plugin.clone()))
				}
			},
			ServiceRowInput::DisablePlugin(index) => {
				if !self.first_load {
					sender.output(ServiceRowOutput::DisablePlugin(
						index,
						self.plugin.clone(),
						self.process_id,
					))
				}
			},
			ServiceRowInput::RemovePlugin(index) => {
				sender.output(ServiceRowOutput::RemovePlugin(
					index,
					self.plugin.clone(),
					self.process_id,
				))
			},
			ServiceRowInput::UpdatePlugin(index) => {
				sender.output(ServiceRowOutput::UpdatePlugin(
					index,
					self.plugin.clone(),
					self.process_id,
				))
			},
			ServiceRowInput::InformStatus(status) => match status {
				UpdateStatus::Completed => self.update = false,
				UpdateStatus::Failed => self.update = true,
			},
			ServiceRowInput::ShowInstallButton(enable) => self.installed = !enable,
			ServiceRowInput::SwitchOn(enabled) => {
				widgets.switch.set_state(enabled);
			},
		}
		self.first_load = false;
		self.update_view(widgets, sender);
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			ServiceRowOutput::InstallPlugin(index, plugin) => {
				PreferencesComponentInput::InstallPlugin(index, plugin)
			},
			ServiceRowOutput::EnablePlugin(index, plugin) => {
				PreferencesComponentInput::EnablePlugin(index, plugin)
			},
			ServiceRowOutput::DisablePlugin(index, plugin, process_id) => {
				PreferencesComponentInput::DisablePlugin(index, plugin, process_id)
			},
			ServiceRowOutput::RemovePlugin(index, plugin, process_id) => {
				PreferencesComponentInput::RemovePlugin(index, plugin, process_id)
			},
			ServiceRowOutput::UpdatePlugin(index, plugin, process_id) => {
				PreferencesComponentInput::UpdatePlugin(index, plugin, process_id)
			},
		};
		Some(output)
	}
}
