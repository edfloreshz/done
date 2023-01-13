use done_provider::plugin::Plugin;
use relm4::adw::prelude::{
	ActionRowExt, AdwWindowExt, PreferencesGroupExt, PreferencesPageExt,
	PreferencesRowExt,
};
use relm4::gtk::prelude::{BoxExt, OrientableExt, WidgetExt};
use relm4::ComponentParts;
use relm4::{adw, gtk};
use relm4::{Component, ComponentSender};

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
										connect_activate[sender] => move |switch| {
											if switch.is_active() {
												sender.input(PreferencesEvent::EnablePlugin(Plugin::Local))
											} else {
												sender.input(PreferencesEvent::DisablePlugin(Plugin::Local))
											}
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
										connect_activate[sender] => move |switch| {
											if switch.is_active() {
												sender.input(PreferencesEvent::EnablePlugin(Plugin::Google))
											} else {
												sender.input(PreferencesEvent::DisablePlugin(Plugin::Google))
											}
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
										connect_activate[sender] => move |switch| {
											if switch.is_active() {
												sender.input(PreferencesEvent::EnablePlugin(Plugin::Microsoft))
											} else {
												sender.input(PreferencesEvent::DisablePlugin(Plugin::Microsoft))
											}
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
										connect_activate[sender] => move |switch| {
											if switch.is_active() {
												sender.input(PreferencesEvent::EnablePlugin(Plugin::Nextcloud))
											} else {
												sender.input(PreferencesEvent::DisablePlugin(Plugin::Nextcloud))
											}
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

	//TODO: Handle plugin events.
	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
		match message {
			PreferencesEvent::EnablePlugin(_plugin) => info!("Plugin enabled"),
			PreferencesEvent::DisablePlugin(_plugin) => info!("Plugin disabled"),
		}
	}
}
