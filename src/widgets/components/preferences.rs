use done_provider::plugin::Plugin;
use relm4::adw::prelude::{
	ActionRowExt, AdwWindowExt, PreferencesGroupExt, PreferencesPageExt,
	PreferencesRowExt,
};
use relm4::gtk::prelude::{BoxExt, OrientableExt, WidgetExt};
use relm4::ComponentParts;
use relm4::{adw, gtk};
use relm4::{Component, ComponentSender};
use directories::{ProjectDirs, ProjectDirsExt};
pub struct Preferences {}

#[derive(Debug)]
pub enum PreferencesEvent {
	EnablePlugin(Plugin),
	DisablePlugin(Plugin)
}

#[relm4::component(pub)]
impl Component for Preferences {
	type CommandOutput = ();
	type Input = PreferencesEvent;
	type Output = ();
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
		let model = Preferences {};
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
		match message {
            PreferencesEvent::EnablePlugin(plugin) => match plugin.start() {
                Ok(_) => {
                    info!("Plugin {:?} started...", plugin);
                    if let Some(project) = ProjectDirs::from("dev", "edfloreshz", "Done") {
                        project.place_config_file("plugins.toml").unwrap();
                    }
                },
                Err(err) => info!("Failed to start {:?} plugin: {:?}", plugin, err)
            },
            PreferencesEvent::DisablePlugin(plugin) => match plugin.stop() {
                Ok(_) => info!("Plugin {:?} stopped.", plugin),
                Err(err) => info!("Failed to stop {:?} plugin: {:?}", plugin, err)
            },
		}
	}
}
