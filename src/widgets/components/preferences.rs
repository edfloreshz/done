use crate::app::toast;
use crate::application::plugin::Plugin;
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
pub struct Preferences {
	pub plugins: Vec<PluginPreferences>,
	pub color_scheme: ColorScheme,
}

impl Default for Preferences {
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
pub enum PreferencesEvent {
	EnablePlugin(Plugin),
	DisablePlugin(Plugin),
	InstallPlugin(Plugin),
	SetDarkColorScheme,
	SetLightColorScheme,
	SetDefaultColorScheme,
}

#[derive(Debug)]
pub enum PreferencesOutput {
	EnablePluginOnSidebar(Plugin),
	DisablePluginOnSidebar(Plugin),
}

#[relm4::component(pub async)]
impl AsyncComponent for Preferences {
	type CommandOutput = ();
	type Input = PreferencesEvent;
	type Output = PreferencesOutput;
	type Init = ();

	view! {
		adw::PreferencesWindow {
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
								set_title: "Appearance",
								add = &adw::ComboRow {
									set_title: "Color scheme",
									set_subtitle: "Set the color scheme of the app",
									set_model: Some(&gtk::StringList::new(&["Light", "Dark", "Default"])),
									set_selected: match model.color_scheme {
										ColorScheme::Light => 0,
										ColorScheme::Dark => 1,
										ColorScheme::Default => 2,
									},
									connect_selected_notify[sender] => move |combo_row| {
										match combo_row.selected() {
											0 => sender.input_sender().send(PreferencesEvent::SetLightColorScheme).unwrap(),
											1 => sender.input_sender().send(PreferencesEvent::SetDarkColorScheme).unwrap(),
											_ => sender.input_sender().send(PreferencesEvent::SetDefaultColorScheme).unwrap(),
										}
									},
								}
							},
							#[name(services)]
							add = &adw::PreferencesGroup {
								set_title: "Services",
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
				.get_file_as::<Preferences>("preferences", FileFormat::JSON)
				.unwrap_or(Preferences::default())
		} else {
			Preferences::default()
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
							set_label: "Install",
							set_visible: !plugin.is_installed(),
							connect_clicked[sender, plugin] => move |_| {
								sender.input(PreferencesEvent::InstallPlugin(plugin.clone()))
							}
						},
						append = &gtk::Switch {
							set_visible: plugin.is_installed(),
							#[watch]
							set_active: plugin.is_running(),
							connect_state_set[sender, plugin] => move |_, state| {
								if state {
									sender.input(PreferencesEvent::EnablePlugin(plugin.clone()))
								} else {
									sender.input(PreferencesEvent::DisablePlugin(plugin.clone()))
								}
								Default::default()
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
			PreferencesEvent::EnablePlugin(plugin) => match plugin.start() {
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
							.output(PreferencesOutput::EnablePluginOnSidebar(plugin))
							.unwrap(),
						Err(e) => error!("{:?}", e),
					}
				},
				Err(err) => {
					info!("Failed to start {:?} plugin: {:?}", plugin, err);
					widgets
						.overlay
						.add_toast(&toast("Failed to start this plug-in."))
				},
			},
			PreferencesEvent::DisablePlugin(plugin) => match plugin.stop() {
				Ok(_) => {
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
								.output(PreferencesOutput::DisablePluginOnSidebar(plugin))
								.unwrap(),
							Err(e) => error!("{:?}", e),
						}
					}
				},
				Err(err) => info!("Failed to stop {:?} plugin: {:?}", plugin, err),
			},
			PreferencesEvent::InstallPlugin(_plugin) => todo!(),
			PreferencesEvent::SetDarkColorScheme => {
				adw::StyleManager::default()
					.set_color_scheme(adw::ColorScheme::ForceDark);
				self.color_scheme = ColorScheme::Dark;
				update_preferences(self).unwrap()
			},
			PreferencesEvent::SetLightColorScheme => {
				adw::StyleManager::default()
					.set_color_scheme(adw::ColorScheme::ForceLight);
				self.color_scheme = ColorScheme::Light;
				update_preferences(self).unwrap()
			},
			PreferencesEvent::SetDefaultColorScheme => {
				adw::StyleManager::default()
					.set_color_scheme(adw::ColorScheme::Default);
				self.color_scheme = ColorScheme::Default;
				update_preferences(self).unwrap()
			},
		}
		self.update_view(widgets, sender)
	}
}

fn update_preferences(preferences: &Preferences) -> Result<()> {
	Project::open("dev", "edfloreshz", "done")?
		.get_file("preferences", FileFormat::JSON)?
		.set_content(preferences)?
		.write()
}
