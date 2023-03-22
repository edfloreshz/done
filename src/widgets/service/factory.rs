use crate::fl;
use crate::widgets::preferences::messages::PreferencesComponentInput;
use crate::widgets::preferences::model::PluginPreferences;
use adw::prelude::{ActionRowExt, ButtonExt, PreferencesRowExt};
use relm4::adw;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::gtk;
use relm4::gtk::traits::WidgetExt;
use relm4_icons::icon_name;

use super::messages::{ServiceInput, ServiceOutput};
use super::model::{ServiceModel, UpdateStatus};

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for ServiceModel {
	type ParentInput = PreferencesComponentInput;
	type ParentWidget = adw::PreferencesGroup;
	type CommandOutput = ();
	type Input = ServiceInput;
	type Output = ServiceOutput;
	type Init = PluginPreferences;

	view! {
		#[root]
		#[name(service)]
		adw::ActionRow {
				set_title: &self.plugin.name,
				set_subtitle: &self.plugin.description,
				add_suffix = &gtk::Button {
					set_has_tooltip: true,
					set_tooltip_text: Some("Remove service"),
					#[watch]
					set_visible: self.installed,
					set_icon_name: icon_name::X_CIRCULAR,
					set_css_classes: &["destructive-action"],
					set_tooltip_text: Some(fl!("remove")),
					set_valign: gtk::Align::Center,
					connect_clicked[sender, index] => move |_| {
						sender.input(ServiceInput::RemovePlugin(index.clone()));
					}
				},
				add_suffix = &gtk::Button {
					set_has_tooltip: true,
					set_tooltip_text: Some("Update service"),
					#[watch]
					set_visible: self.update && self.installed,
					set_icon_name: icon_name::UPDATE,
					set_css_classes: &["favorite"],
					set_tooltip_text: Some(fl!("update")),
					set_valign: gtk::Align::Center,
					connect_clicked[sender, index] => move |_| {
							sender.input(ServiceInput::UpdatePlugin(index.clone()));
					}
				},
				add_suffix = &gtk::Button {
					set_has_tooltip: true,
					set_tooltip_text: Some("Install service"),
					#[watch]
					set_visible: !self.installed,
					set_label: fl!("install"),
					set_valign: gtk::Align::Center,
					connect_clicked[sender, index] => move |_| {
							sender.input(ServiceInput::InstallPlugin(index.clone()));
					}
				},
				#[name(switch)]
				add_suffix = &gtk::Switch {
					set_has_tooltip: true,
					set_tooltip_text: Some("Set service state"),
					set_valign: gtk::Align::Center,
					#[watch]
					set_visible: self.installed,
					connect_state_set[sender, index] => move |_, state| {
						sender.input(ServiceInput::ToggleSwitch(index.clone(), state));
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
			ServiceInput::ToggleSwitch(index, state) => {
				if state {
					sender.input(ServiceInput::EnablePlugin(index));
				} else {
					sender.input(ServiceInput::DisablePlugin(index));
				}
			},
			ServiceInput::InstallPlugin(index) => {
				sender.output(ServiceOutput::Install(index, self.plugin.clone()));
			},
			ServiceInput::EnablePlugin(index) => {
				if !self.first_load {
					sender.output(ServiceOutput::Enable(index, self.plugin.clone()))
				}
			},
			ServiceInput::DisablePlugin(index) => {
				if !self.first_load {
					sender.output(ServiceOutput::Disable(index, self.plugin.clone()))
				}
			},
			ServiceInput::RemovePlugin(index) => {
				sender.output(ServiceOutput::Uninstall(index, self.plugin.clone()))
			},
			ServiceInput::UpdatePlugin(index) => {
				sender.output(ServiceOutput::Update(index, self.plugin.clone()))
			},
			ServiceInput::InformStatus(status) => match status {
				UpdateStatus::Completed => self.update = false,
				UpdateStatus::Failed => self.update = true,
			},
			ServiceInput::ShowInstallButton(enable) => self.installed = !enable,
			ServiceInput::SwitchOn(enabled) => {
				widgets.switch.set_state(enabled);
			},
		}
		self.first_load = false;
		self.update_view(widgets, sender);
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			ServiceOutput::Install(index, plugin) => {
				PreferencesComponentInput::InstallPlugin(index, plugin)
			},
			ServiceOutput::Enable(index, plugin) => {
				PreferencesComponentInput::EnablePlugin(index, plugin)
			},
			ServiceOutput::Disable(index, plugin) => {
				PreferencesComponentInput::DisablePlugin(index, plugin)
			},
			ServiceOutput::Uninstall(index, plugin) => {
				PreferencesComponentInput::RemovePlugin(index, plugin)
			},
			ServiceOutput::Update(index, plugin) => {
				PreferencesComponentInput::UpdatePlugin(index, plugin)
			},
		};
		Some(output)
	}
}
