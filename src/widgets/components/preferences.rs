use crate::application::plugin::Plugin;
use anyhow::Result;
use libset::format::FileFormat;
use libset::project::Project;
use relm4::adw::prelude::{
	ActionRowExt, AdwWindowExt, PreferencesGroupExt, PreferencesPageExt,
	PreferencesRowExt,
};
use relm4::adw::traits::ComboRowExt;
use relm4::gtk::prelude::{BoxExt, ListModelExt, OrientableExt, WidgetExt};
use relm4::gtk::traits::ListBoxRowExt;
use relm4::ComponentParts;
use relm4::{adw, gtk};
use relm4::{Component, ComponentSender};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Preferences {
	pub plugins: ProviderPreferences,
	pub color_scheme: ColorScheme,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ProviderPreferences {
	pub local_enabled: bool,
	pub google_enabled: bool,
	pub microsoft_enabled: bool,
	pub nextcloud_enabled: bool,
}

impl Default for ProviderPreferences {
	fn default() -> Self {
		Self {
			local_enabled: true,
			google_enabled: false,
			microsoft_enabled: false,
			nextcloud_enabled: false,
		}
	}
}

#[derive(Debug)]
pub enum PreferencesEvent {
	EnablePlugin(Plugin),
	DisablePlugin(Plugin),
	SetDarkColorScheme,
	SetLightColorScheme,
	SetDefaultColorScheme,
}

#[derive(Debug)]
pub enum PreferencesOutput {
	EnablePluginOnSidebar(Plugin),
	DisablePluginOnSidebar(Plugin),
}

#[relm4::component(pub)]
impl Component for Preferences {
	type CommandOutput = ();
	type Input = PreferencesEvent;
	type Output = PreferencesOutput;
	type Init = ();

	view! {
		adw::PreferencesWindow {
			#[wrap(Some)]
			set_content = &gtk::Box {
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
						add = &adw::PreferencesGroup {
							set_title: "Providers",
							add = &adw::ActionRow {
								set_title: "Local",
								set_subtitle: "Local task provider",
								add_suffix = &gtk::Box {
									set_halign: gtk::Align::Center,
									set_valign: gtk::Align::Center,
									append = &gtk::Switch {
										#[watch]
										set_active: model.plugins.local_enabled,
										connect_state_set[sender] => move |_, state| {
											if state {
												sender.input(PreferencesEvent::EnablePlugin(Plugin::Local))
											} else {
												sender.input(PreferencesEvent::DisablePlugin(Plugin::Local))
											}
											Default::default()
										}
									}
								}
							},
							add = &adw::ActionRow {
								set_title: "Google",
								set_subtitle: "Google Task provider",
								add_suffix = &gtk::Box {
									set_halign: gtk::Align::Center,
									set_valign: gtk::Align::Center,
									append = &gtk::Switch {
										#[watch]
										set_active: model.plugins.google_enabled,
										connect_state_set[sender] => move |_, state| {
											if state {
												sender.input(PreferencesEvent::EnablePlugin(Plugin::Google))
											} else {
												sender.input(PreferencesEvent::DisablePlugin(Plugin::Google))
											}
											Default::default()
										}
									}
								}
							},
							add = &adw::ActionRow {
								set_title: "Microsoft",
								set_subtitle: "Microsoft To Do provider",
								add_suffix = &gtk::Box {
									set_halign: gtk::Align::Center,
									set_valign: gtk::Align::Center,
									append = &gtk::Switch {
										#[watch]
										set_active: model.plugins.microsoft_enabled,
										connect_state_set[sender] => move |_, state| {
												if state {
												sender.input(PreferencesEvent::EnablePlugin(Plugin::Microsoft))
											} else {
												sender.input(PreferencesEvent::DisablePlugin(Plugin::Microsoft))
											}
											Default::default()
										}
									}
								}
							},
							add = &adw::ActionRow {
								set_title: "Nextcloud",
								set_subtitle: "Nextcloud Tasks provider",
								add_suffix = &gtk::Box {
									set_halign: gtk::Align::Center,
									set_valign: gtk::Align::Center,
									append = &gtk::Switch {
										#[watch]
										set_active: model.plugins.nextcloud_enabled,
										connect_state_set[sender] => move |_, state| {
											if state {
												sender.input(PreferencesEvent::EnablePlugin(Plugin::Nextcloud))
											} else {
												sender.input(PreferencesEvent::DisablePlugin(Plugin::Nextcloud))
											}
											Default::default()
										}
									}
								}
							},
						},
					}
				}
			}
		}
	}

	fn init(
		_init: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = if let Ok(project) = Project::open("dev", "edfloreshz", "done")
		{
			project
				.get_file_as::<Preferences>("preferences", FileFormat::JSON)
				.unwrap_or(Preferences::default())
		} else {
			Preferences::default()
		};

		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(
		&mut self,
		message: Self::Input,
		sender: ComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			PreferencesEvent::EnablePlugin(plugin) => match plugin.start() {
				Ok(_) => {
					info!("Plugin {:?} started...", plugin);
					match plugin {
						Plugin::Local => self.plugins.local_enabled = true,
						Plugin::Google => self.plugins.google_enabled = true,
						Plugin::Microsoft => self.plugins.microsoft_enabled = true,
						Plugin::Nextcloud => self.plugins.nextcloud_enabled = true,
					}
					match update_preferences(self) {
						Ok(()) => sender
							.output(PreferencesOutput::EnablePluginOnSidebar(plugin))
							.unwrap(),
						Err(e) => error!("{:?}", e),
					}
				},
				Err(err) => info!("Failed to start {:?} plugin: {:?}", plugin, err),
			},
			PreferencesEvent::DisablePlugin(plugin) => match plugin.stop() {
				Ok(_) => {
					info!("Plugin {:?} stopped.", plugin);
					match plugin {
						Plugin::Local => self.plugins.local_enabled = false,
						Plugin::Google => self.plugins.google_enabled = false,
						Plugin::Microsoft => self.plugins.microsoft_enabled = false,
						Plugin::Nextcloud => self.plugins.nextcloud_enabled = false,
					}
					match update_preferences(self) {
						Ok(()) => sender
							.output(PreferencesOutput::DisablePluginOnSidebar(plugin))
							.unwrap(),
						Err(e) => error!("{:?}", e),
					}
				},
				Err(err) => info!("Failed to stop {:?} plugin: {:?}", plugin, err),
			},
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
	}
}

fn update_preferences(preferences: &Preferences) -> Result<()> {
	Project::open("dev", "edfloreshz", "done")?
		.get_file("preferences", FileFormat::JSON)?
		.set_content(preferences)?
		.write()
}
