use anyhow::Result;
use done_provider::plugin::Plugin;
use libset::format::FileFormat;
use libset::project::Project;
use relm4::adw::prelude::{
	ActionRowExt, AdwWindowExt, PreferencesGroupExt, PreferencesPageExt,
	PreferencesRowExt,
};
use relm4::gtk::prelude::{BoxExt, OrientableExt, WidgetExt};
use relm4::ComponentParts;
use relm4::{adw, gtk};
use relm4::{Component, ComponentSender};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Preferences {
	pub plugins: ProviderPreferences,
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
			if let Ok(preferences) =
				project.get_file_as::<Preferences>("preferences", FileFormat::TOML)
			{
				preferences
			} else {
				Preferences::default()
			}
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
					match update_preferences(&self) {
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
					match update_preferences(&self) {
						Ok(()) => sender
							.output(PreferencesOutput::DisablePluginOnSidebar(plugin))
							.unwrap(),
						Err(e) => error!("{:?}", e),
					}
				},
				Err(err) => info!("Failed to stop {:?} plugin: {:?}", plugin, err),
			},
		}
	}
}

fn update_preferences(preferences: &Preferences) -> Result<()> {
	Project::open("dev", "edfloreshz", "done")?
		.get_file("preferences", FileFormat::TOML)?
		.set_content(preferences)?
		.write()
}
