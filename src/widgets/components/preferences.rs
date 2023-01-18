use crate::app::toast;
use crate::application::plugin::Plugin;
use crate::fl;
use adw::prelude::GtkWindowExt;
use anyhow::Result;
use libset::format::FileFormat;
use libset::project::Project;
use relm4::adw::prelude::{
	ActionRowExt, AdwWindowExt, PreferencesGroupExt, PreferencesPageExt,
	PreferencesRowExt,
};
use relm4::adw::traits::ComboRowExt;
use relm4::component::{AsyncComponent, AsyncComponentParts};
use relm4::gtk::prelude::{BoxExt, OrientableExt, WidgetExt};
use relm4::gtk::traits::ButtonExt;
use relm4::AsyncComponentSender;
use relm4::{adw, gtk};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PreferencesComponent {
	pub plugins: Vec<PluginPreferences>,
	pub color_scheme: ColorScheme,
	pub compact: bool,
}

impl Default for PreferencesComponent {
	fn default() -> Self {
		let plugins: Vec<PluginPreferences> = Plugin::get_plugins()
			.unwrap()
			.iter()
			.map(|plugin| PluginPreferences {
				plugin: plugin.clone(),
				enabled: false,
				installed: false,
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
}

#[derive(Debug)]
pub enum PreferencesComponentEvent {
	EnablePlugin(Plugin),
	DisablePlugin(Plugin),
	InstallPlugin(Plugin),
	SetDarkColorScheme,
	SetLightColorScheme,
	SetDefaultColorScheme,
	ToggleCompact(bool),
}

#[derive(Debug)]
pub enum PreferencesComponentOutput {
	EnablePluginOnSidebar(Plugin),
	DisablePluginOnSidebar(Plugin),
	ToggleCompact(bool),
}

#[relm4::component(pub async)]
impl AsyncComponent for PreferencesComponent {
	type CommandOutput = ();
	type Input = PreferencesComponentEvent;
	type Output = PreferencesComponentOutput;
	type Init = ();

	view! {
		adw::PreferencesWindow {
			set_title: Some(fl!("preferences")),
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
									set_selected: match model.color_scheme {
										ColorScheme::Light => 0,
										ColorScheme::Dark => 1,
										ColorScheme::Default => 2,
									},
									connect_selected_notify[sender] => move |combo_row| {
										match combo_row.selected() {
											0 => sender.input_sender().send(PreferencesComponentEvent::SetLightColorScheme).unwrap(),
											1 => sender.input_sender().send(PreferencesComponentEvent::SetDarkColorScheme).unwrap(),
											_ => sender.input_sender().send(PreferencesComponentEvent::SetDefaultColorScheme).unwrap(),
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
											set_active: model.compact,
											connect_state_set[sender] => move |_, state| {
												sender.input(PreferencesComponentEvent::ToggleCompact(state));
												gtk::Inhibit::default()
											}
										}
									}
								}
							},
							#[name(services)]
							add = &adw::PreferencesGroup {
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
		let model = if let Ok(project) = Project::open("dev", "edfloreshz", "done")
		{
			project
				.get_file_as::<PreferencesComponent>("preferences", FileFormat::JSON)
				.unwrap_or_default()
		} else {
			PreferencesComponent::default()
		};

		let widgets = view_output!();

		let plugins = Plugin::fetch_plugins().await.unwrap_or_default();

		for plugin in plugins {
			relm4::view! {
				#[name(service)]
				adw::ActionRow {
					set_title: &plugin.name,
					set_subtitle: &plugin.description,
					add_suffix = &gtk::Box {
						set_halign: gtk::Align::Center,
						set_valign: gtk::Align::Center,
						append = &gtk::Button {
							set_label: fl!("install"),
							set_visible: !plugin.is_installed(),
							connect_clicked[sender, plugin] => move |_| {
								sender.input(PreferencesComponentEvent::InstallPlugin(plugin.clone()));
							}
						},
						append = &gtk::Switch {
							set_visible: plugin.is_installed(),
							#[watch]
							set_active: plugin.is_running(),
							connect_state_set[sender, plugin] => move |_, state| {
								if state {
									sender.input(PreferencesComponentEvent::EnablePlugin(plugin.clone()));
								} else {
									sender.input(PreferencesComponentEvent::DisablePlugin(plugin.clone()));
								}
								gtk::Inhibit::default()
							}
						}
					}
				}
			}
			widgets.services.add(&service);
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
			PreferencesComponentEvent::EnablePlugin(plugin) => match plugin.start() {
				Ok(_) => {
					info!("Plugin {:?} started...", plugin);
					widgets.overlay.add_toast(&toast("Service enabled."));

					self.plugins = self
						.plugins
						.iter_mut()
						.filter(|p| p.plugin == plugin)
						.map(|p| {
							p.enabled = true;
							p.clone()
						})
						.collect();

					match update_preferences(self) {
						Ok(()) => sender
							.output(PreferencesComponentOutput::EnablePluginOnSidebar(plugin))
							.unwrap(),
						Err(e) => error!("{:?}", e),
					}
				},
				Err(err) => {
					info!("Failed to start {:?} plugin: {:?}", plugin, err);
					widgets
						.overlay
						.add_toast(&toast("Failed to start this plug-in."));
				},
			},
			PreferencesComponentEvent::DisablePlugin(plugin) => {
				plugin.stop();
				info!("Plugin {:?} stopped.", plugin);
				let previous_model = self.clone();
				self.plugins = self
					.plugins
					.iter_mut()
					.filter(|p| p.plugin == plugin)
					.map(|p| {
						p.enabled = false;
						p.clone()
					})
					.collect();
				if previous_model != *self {
					widgets.overlay.add_toast(&toast("Service disabled."));
					match update_preferences(self) {
						Ok(()) => sender
							.output(PreferencesComponentOutput::DisablePluginOnSidebar(
								plugin,
							))
							.unwrap(),
						Err(e) => error!("{:?}", e),
					}
				}
			},
			PreferencesComponentEvent::InstallPlugin(_plugin) => todo!(),
			PreferencesComponentEvent::SetDarkColorScheme => {
				adw::StyleManager::default()
					.set_color_scheme(adw::ColorScheme::ForceDark);
				self.color_scheme = ColorScheme::Dark;
				update_preferences(self).unwrap();
			},
			PreferencesComponentEvent::SetLightColorScheme => {
				adw::StyleManager::default()
					.set_color_scheme(adw::ColorScheme::ForceLight);
				self.color_scheme = ColorScheme::Light;
				update_preferences(self).unwrap();
			},
			PreferencesComponentEvent::SetDefaultColorScheme => {
				adw::StyleManager::default()
					.set_color_scheme(adw::ColorScheme::Default);
				self.color_scheme = ColorScheme::Default;
				update_preferences(self).unwrap();
			},
			PreferencesComponentEvent::ToggleCompact(compact) => {
				self.compact = compact;
				update_preferences(self).unwrap();
				sender
					.output(PreferencesComponentOutput::ToggleCompact(self.compact))
					.unwrap();
			},
		}
		self.update_view(widgets, sender);
	}
}

fn update_preferences(preferences: &PreferencesComponent) -> Result<()> {
	Project::open("dev", "edfloreshz", "done")?
		.get_file("preferences", FileFormat::JSON)?
		.set_content(preferences)?
		.write()
}
