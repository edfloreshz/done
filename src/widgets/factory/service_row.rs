use crate::application::plugin::Plugin;
use crate::fl;
use crate::widgets::components::preferences::{
	PluginPreferences, PreferencesComponentInput,
};
use adw::prelude::{ActionRowExt, BoxExt, ButtonExt, PreferencesRowExt};
use relm4::adw;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::gtk;
use relm4::gtk::traits::WidgetExt;

#[derive(Debug, Default)]
pub struct ServiceRowModel {
	pub plugin: Plugin,
	pub enabled: bool,
	pub installed: bool,
	pub first_load: bool,
}

#[derive(Debug)]
pub enum ServiceRowInput {
	InstallPlugin(DynamicIndex),
	EnablePlugin(DynamicIndex),
	DisablePlugin(DynamicIndex),
	RemovePlugin(DynamicIndex),
	EnableInstallButton(bool),
	EnableSwitch(bool),
}

#[derive(Debug)]
pub enum ServiceRowOutput {
	InstallPlugin(DynamicIndex, Plugin),
	EnablePlugin(DynamicIndex, Plugin),
	DisablePlugin(DynamicIndex, Plugin),
	RemovePlugin(DynamicIndex, Plugin),
}

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
				add_suffix = &gtk::Box {
						set_halign: gtk::Align::Center,
						set_valign: gtk::Align::Center,
						append = &gtk::Button {
								set_label: fl!("install"),
								#[watch]
								set_visible: !self.installed,
								connect_clicked[sender, index] => move |_| {
										sender.input(ServiceRowInput::InstallPlugin(index.clone()));
								}
						},
						append = &gtk::Button {
							#[watch]
							set_visible: self.installed,
							set_icon_name: "user-trash-full-symbolic",
							connect_activate[sender, index] => move |_| {
									sender.input(ServiceRowInput::RemovePlugin(index.clone()));
							}
						},
						append = &gtk::Switch {
								#[watch]
								set_visible: self.installed,
								set_active: self.enabled,
								connect_state_set[sender, index] => move |_, state| {
										if state {
												sender.input(ServiceRowInput::EnablePlugin(index.clone()));
										} else {
												sender.input(ServiceRowInput::DisablePlugin(index.clone()));
										}
										gtk::Inhibit::default()
								}
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
		widgets
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncFactorySender<Self>,
	) {
		match message {
			ServiceRowInput::InstallPlugin(index) => sender
				.output(ServiceRowOutput::InstallPlugin(index, self.plugin.clone())),
			ServiceRowInput::EnablePlugin(index) => {
				if !self.first_load {
					sender
						.output(ServiceRowOutput::EnablePlugin(index, self.plugin.clone()))
				}
			},
			ServiceRowInput::DisablePlugin(index) => {
				if !self.first_load {
					sender
						.output(ServiceRowOutput::DisablePlugin(index, self.plugin.clone()))
				}
			},
			ServiceRowInput::RemovePlugin(index) => sender
				.output(ServiceRowOutput::RemovePlugin(index, self.plugin.clone())),
			ServiceRowInput::EnableInstallButton(enable) => self.installed = !enable,
			ServiceRowInput::EnableSwitch(enabled) => self.enabled = enabled,
		}
		self.first_load = false;
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			ServiceRowOutput::InstallPlugin(index, plugin) => {
				PreferencesComponentInput::InstallPlugin(index, plugin)
			},
			ServiceRowOutput::EnablePlugin(index, plugin) => {
				PreferencesComponentInput::EnablePlugin(index, plugin)
			},
			ServiceRowOutput::DisablePlugin(index, plugin) => {
				PreferencesComponentInput::DisablePlugin(index, plugin)
			},
			ServiceRowOutput::RemovePlugin(index, plugin) => {
				PreferencesComponentInput::RemovePlugin(index, plugin)
			},
		};
		Some(output)
	}
}
