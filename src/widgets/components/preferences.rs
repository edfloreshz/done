use std::path::PathBuf;

use crate::app::toast;
use crate::application::plugin::Plugin;
use crate::fl;
use crate::widgets::factory::service_row::{ServiceRowInput, ServiceRowModel};
use adw::prelude::{BoxExt, GtkWindowExt, OrientableExt, WidgetExt};
use anyhow::Result;
use directories::ProjectDirs;
use libset::format::FileFormat;
use libset::project::Project;
use relm4::adw::prelude::{
	ActionRowExt, AdwWindowExt, PreferencesGroupExt, PreferencesPageExt,
	PreferencesRowExt,
};
use relm4::adw::traits::ComboRowExt;
use relm4::component::{AsyncComponent, AsyncComponentParts};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::prelude::DynamicIndex;
use relm4::AsyncComponentSender;
use relm4::{adw, gtk};
use serde::{Deserialize, Serialize};

#[derive(Debug)]

pub struct PreferencesComponentModel {
	pub preferences: Preferences,
	pub service_row_factory: AsyncFactoryVecDeque<ServiceRowModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Preferences {
	pub plugins: Vec<PluginPreferences>,
	pub color_scheme: ColorScheme,
	pub compact: bool,
}

impl Default for Preferences {
	fn default() -> Self {
		let project = ProjectDirs::from("dev", "edfloreshz", "done").unwrap();

		let plugins: Vec<PluginPreferences> = Plugin::get_plugins()
			.unwrap()
			.iter()
			.map(|plugin| PluginPreferences {
				plugin: plugin.clone(),
				enabled: false,
				installed: false,
				executable: project.data_dir().join("bin").join(&plugin.process_name),
			})
			.collect();
		Self {
			plugins,
			color_scheme: ColorScheme::Default,
			compact: false,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ColorScheme {
	Dark,
	Light,
	Default,
}

impl Default for ColorScheme {
	fn default() -> Self {
		Self::Default
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PluginPreferences {
	pub plugin: Plugin,
	pub enabled: bool,
	pub installed: bool,
	pub executable: PathBuf,
}

#[derive(Debug)]
pub enum PreferencesComponentInput {
	EnablePlugin(DynamicIndex, Plugin),
	DisablePlugin(DynamicIndex, Plugin),
	InstallPlugin(DynamicIndex, Plugin),
	RemovePlugin(DynamicIndex, Plugin),
	SetDarkColorScheme,
	SetLightColorScheme,
	SetDefaultColorScheme,
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

#[relm4::component(pub async)]
impl AsyncComponent for PreferencesComponentModel {
	type CommandOutput = ();
	type Input = PreferencesComponentInput;
	type Output = PreferencesComponentOutput;
	type Init = ();

	view! {
		adw::PreferencesWindow {
			set_title: Some(fl!("preferences")),
			set_hide_on_close: true,
			#[wrap(Some)]
			#[name = "overlay"]
			set_content = &adw::ToastOverlay {
				#[wrap(Some)]
				set_child = &gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					append = &adw::HeaderBar {
						set_show_end_title_buttons: true
					},
					append = &adw::Clamp {
						#[wrap(Some)]
						set_child = &adw::PreferencesPage {
							add = &adw::PreferencesGroup {
								set_title: fl!("appearance"),
								adw::ComboRow {
									set_title: fl!("color-scheme"),
									set_subtitle: fl!("color-scheme-description"),
									set_model: Some(&gtk::StringList::new(&[
										fl!("color-scheme-light"),
										fl!("color-scheme-dark"),
										fl!("color-scheme-default")
									])),
									set_selected: match model.preferences.color_scheme {
										ColorScheme::Light => 0,
										ColorScheme::Dark => 1,
										ColorScheme::Default => 2,
									},
									connect_selected_notify[sender] => move |combo_row| {
										match combo_row.selected() {
											0 => sender.input_sender().send(PreferencesComponentInput::SetLightColorScheme).unwrap(),
											1 => sender.input_sender().send(PreferencesComponentInput::SetDarkColorScheme).unwrap(),
											_ => sender.input_sender().send(PreferencesComponentInput::SetDefaultColorScheme).unwrap(),
										}
									},
								},
								adw::ActionRow {
									set_title: fl!("compact"),
									set_subtitle: fl!("compact-description"),
									add_suffix = &gtk::Box {
										set_halign: gtk::Align::Center,
										set_valign: gtk::Align::Center,
										append = &gtk::Switch {
											set_active: model.preferences.compact,
											connect_state_set[sender] => move |_, state| {
												sender.input(PreferencesComponentInput::ToggleCompact(state));
												gtk::Inhibit::default()
											}
										}
									}
								}
							},
							#[local_ref]
							add = services_container -> adw::PreferencesGroup {
								set_title: fl!("services"),
							},
						}
					}
				}
			}
		}
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let preferences =
			if let Ok(project) = Project::open("dev", "edfloreshz", "done") {
				project
					.get_file_as::<Preferences>("preferences", FileFormat::JSON)
					.unwrap_or_default()
			} else {
				Preferences::default()
			};

		let mut model = Self {
			preferences,
			service_row_factory: AsyncFactoryVecDeque::new(
				adw::PreferencesGroup::default(),
				sender.input_sender(),
			),
		};

		let services_container = model.service_row_factory.widget();

		let widgets = view_output!();

		let plugins = relm4::spawn(Plugin::fetch_plugins())
			.await
			.unwrap()
			.unwrap();

		let project = ProjectDirs::from("dev", "edfloreshz", "done").unwrap();

		model.preferences.plugins.clear();
		for plugin in plugins {
			let preferences = PluginPreferences {
				plugin: plugin.clone(),
				enabled: plugin.clone().is_running(),
				installed: plugin.clone().is_installed(),
				executable: project.data_dir().join("bin").join(&plugin.process_name),
			};
			model
				.service_row_factory
				.guard()
				.push_back(preferences.clone());
			model.preferences.plugins.push(preferences);
		}

		AsyncComponentParts { model, widgets }
	}

	async fn update_with_view(
		&mut self,
		widgets: &mut Self::Widgets,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			PreferencesComponentInput::EnablePlugin(index, plugin) => {
				match plugin.start() {
					Ok(_) => {
						tracing::info!("Plugin {:?} started...", plugin);
						widgets.overlay.add_toast(&toast("Service enabled.", 1));

						self.preferences.plugins = self
							.preferences
							.plugins
							.iter_mut()
							.map(|p| {
								if p.plugin == plugin {
									p.enabled = true;
								}
								p.clone()
							})
							.collect();

						match update_preferences(&self.preferences) {
							Ok(()) => {
								sender
									.output(PreferencesComponentOutput::EnablePluginOnSidebar(
										plugin,
									))
									.unwrap();
								self.service_row_factory.send(
									index.current_index(),
									ServiceRowInput::EnableSwitch(true),
								);
							},
							Err(e) => tracing::error!("{:?}", e),
						}
					},
					Err(err) => {
						tracing::info!("Failed to start {:?} plugin: {:?}", plugin, err);
						widgets
							.overlay
							.add_toast(&toast("Failed to start this plug-in.", 2));
					},
				}
			},
			PreferencesComponentInput::DisablePlugin(index, plugin) => {
				plugin.stop();
				tracing::info!("Plugin {:?} stopped.", plugin);
				let previous_model = self.preferences.clone();
				self.preferences.plugins = self
					.preferences
					.plugins
					.iter_mut()
					.map(|p| {
						if p.plugin == plugin {
							p.enabled = false;
						}
						p.clone()
					})
					.collect();
				if previous_model != self.preferences {
					widgets.overlay.add_toast(&toast("Service disabled.", 1));
					match update_preferences(&self.preferences) {
						Ok(()) => {
							sender
								.output(PreferencesComponentOutput::DisablePluginOnSidebar(
									plugin,
								))
								.unwrap();
							self.service_row_factory.send(
								index.current_index(),
								ServiceRowInput::EnableSwitch(false),
							);
						},
						Err(e) => tracing::error!("{:?}", e),
					}
				}
			},
			PreferencesComponentInput::InstallPlugin(index, plugin) => {
				let install_plugin = plugin.clone();
				match relm4::spawn(async move { install_plugin.install().await })
					.await
					.unwrap()
				{
					Ok(_) => {
						if let Some(plugin) = self
							.preferences
							.plugins
							.iter_mut()
							.find(|p| p.plugin == plugin)
						{
							plugin.installed = true;
							plugin.enabled = true;
						} else {
							tracing::error!("This plugin is not registered.")
						}
						update_preferences(&self.preferences).unwrap();
						sender
							.output_sender()
							.send(PreferencesComponentOutput::AddPluginToSidebar(
								plugin.clone(),
							))
							.unwrap();
						self.service_row_factory.send(
							index.current_index(),
							ServiceRowInput::EnableInstallButton(false),
						);
						self
							.service_row_factory
							.send(index.current_index(), ServiceRowInput::EnableSwitch(true));
					},
					Err(err) => {
						tracing::error!("{err:?}");
						widgets.overlay.add_toast(&toast(err, 2))
					},
				}
			},
			PreferencesComponentInput::RemovePlugin(index, plugin) => {
				plugin.stop();
				if let Some(preferences) = self
					.preferences
					.plugins
					.iter_mut()
					.find(|preferences| preferences.plugin == plugin)
				{
					match std::fs::remove_file(&preferences.executable) {
						Ok(_) => {
							preferences.enabled = false;
							preferences.installed = false;
							match update_preferences(&self.preferences) {
								Ok(_) => {
									self.service_row_factory.send(
										index.current_index(),
										ServiceRowInput::EnableSwitch(false),
									);
									self.service_row_factory.send(
										index.current_index(),
										ServiceRowInput::EnableInstallButton(true),
									);
									sender
										.output(
											PreferencesComponentOutput::RemovePluginFromSidebar(
												plugin,
											),
										)
										.unwrap()
								},
								Err(err) => {
									tracing::error!("Failed to update plugin preferences: {err}")
								},
							}
						},
						Err(err) => {
							tracing::error!("Failed to remove plugin executable: {err}")
						},
					}
				}
			},
			PreferencesComponentInput::SetDarkColorScheme => {
				adw::StyleManager::default()
					.set_color_scheme(adw::ColorScheme::ForceDark);
				self.preferences.color_scheme = ColorScheme::Dark;
				update_preferences(&self.preferences).unwrap();
			},
			PreferencesComponentInput::SetLightColorScheme => {
				adw::StyleManager::default()
					.set_color_scheme(adw::ColorScheme::ForceLight);
				self.preferences.color_scheme = ColorScheme::Light;
				update_preferences(&self.preferences).unwrap();
			},
			PreferencesComponentInput::SetDefaultColorScheme => {
				adw::StyleManager::default()
					.set_color_scheme(adw::ColorScheme::Default);
				self.preferences.color_scheme = ColorScheme::Default;
				update_preferences(&self.preferences).unwrap();
			},
			PreferencesComponentInput::ToggleCompact(compact) => {
				self.preferences.compact = compact;
				update_preferences(&self.preferences).unwrap();
				sender
					.output(PreferencesComponentOutput::ToggleCompact(
						self.preferences.compact,
					))
					.unwrap();
			},
		}
		self.update_view(widgets, sender);
	}
}

fn update_preferences(preferences: &Preferences) -> Result<()> {
	Project::open("dev", "edfloreshz", "done")?
		.get_file("preferences", FileFormat::JSON)?
		.set_content(preferences)?
		.write()
}
