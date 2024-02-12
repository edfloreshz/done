use adw::prelude::PreferencesDialogExt;
use anyhow::Result;
use core_done::service::Service;
use libset::Config;
use relm4::{
	adw,
	adw::prelude::ComboRowExt,
	adw::prelude::{
		ActionRowExt, PreferencesGroupExt, PreferencesPageExt, PreferencesRowExt,
		WidgetExt,
	},
	component::{AsyncComponent, AsyncComponentParts},
	gtk, AsyncComponentSender,
};

use crate::app::config::preferences::Preferences;
use crate::app::config::{appearance::ColorScheme, info::APP_ID};
use crate::fl;

#[derive(Debug)]
pub struct PreferencesComponentModel {
	pub preferences: Preferences,
}

#[derive(Debug)]
pub enum PreferencesComponentInput {
	SetColorScheme(ColorScheme),
	ExpandSubTasks,
	MicrosoftLogin,
	MicrosoftLogout,
}

#[derive(Debug)]
pub enum PreferencesComponentOutput {
	ServiceDisabled(Service),
	ExpandSubTasks(bool),
}

#[relm4::component(pub async)]
impl AsyncComponent for PreferencesComponentModel {
	type CommandOutput = ();
	type Input = PreferencesComponentInput;
	type Output = PreferencesComponentOutput;
	type Init = ();

	view! {
		adw::PreferencesDialog {
			#[name = "overlay"]
			add = &adw::PreferencesPage {
				set_vexpand: true,
				add = &adw::PreferencesGroup {
					set_title: fl!("appearance"),
					adw::ComboRow {
						set_title: fl!("color-scheme"),
						set_subtitle: fl!("color-scheme-description"),
						add_prefix = &gtk::Image {
							set_icon_name: Some("dark-mode-symbolic")
						},
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
								0 => sender.input_sender().send(PreferencesComponentInput::SetColorScheme(ColorScheme::Light)).unwrap(),
								1 => sender.input_sender().send(PreferencesComponentInput::SetColorScheme(ColorScheme::Dark)).unwrap(),
								_ => sender.input_sender().send(PreferencesComponentInput::SetColorScheme(ColorScheme::Default)).unwrap(),
							}
						},
					},
				},
				add = &adw::PreferencesGroup {
					set_title: fl!("services"),
					adw::SwitchRow {
						set_title: "Microsoft To Do",
						set_subtitle: fl!("msft-todo-description"),
						add_prefix = &gtk::Image {
							set_resource: Some(Service::Microsoft.icon())
						},
						set_active: Service::Microsoft.get_service().available(),
						connect_active_notify[sender] => move |switch| {
							println!("Switch activated");
							if switch.is_active() {
								sender.input_sender().send(PreferencesComponentInput::MicrosoftLogin).unwrap();
							} else {
								sender.input_sender().send(PreferencesComponentInput::MicrosoftLogout).unwrap();
							}
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
		let preferences = if let Ok(config) = Config::new(APP_ID, 1, None) {
			config.get_json("preferences").unwrap_or(Preferences::new())
		} else {
			Preferences::new()
		};

		let model = Self { preferences };

		let widgets = view_output!();

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
			PreferencesComponentInput::SetColorScheme(color_scheme) => {
				match color_scheme {
					ColorScheme::Dark => {
						adw::StyleManager::default()
							.set_color_scheme(adw::ColorScheme::ForceDark);
						self.preferences.color_scheme = ColorScheme::Dark;
					},
					ColorScheme::Light => {
						adw::StyleManager::default()
							.set_color_scheme(adw::ColorScheme::ForceLight);
						self.preferences.color_scheme = ColorScheme::Light;
					},
					ColorScheme::Default => {
						adw::StyleManager::default()
							.set_color_scheme(adw::ColorScheme::Default);
						self.preferences.color_scheme = ColorScheme::Default;
					},
				}

				if let Err(err) = update_preferences(&self.preferences) {
					tracing::error!("{err}")
				}
			},
			PreferencesComponentInput::ExpandSubTasks => {
				self.preferences.expand_subtasks = !self.preferences.expand_subtasks;
				if let Err(err) = update_preferences(&self.preferences) {
					tracing::error!("{err}")
				}
				sender
					.output(PreferencesComponentOutput::ExpandSubTasks(
						self.preferences.expand_subtasks,
					))
					.unwrap();
			},
			PreferencesComponentInput::MicrosoftLogin => {
				let service = Service::Microsoft.get_service();
				match service.login() {
					Ok(_) => println!("Login started"),
					Err(err) => eprintln!("{err}"),
				};
			},
			PreferencesComponentInput::MicrosoftLogout => {
				let service = Service::Microsoft.get_service();
				match service.logout() {
					Ok(_) => {
						println!("Logout completed");
						sender
							.output(PreferencesComponentOutput::ServiceDisabled(
								Service::Microsoft,
							))
							.unwrap();
					},
					Err(err) => eprintln!("{err}"),
				};
			},
		}
		self.update_view(widgets, sender);
	}
}

fn update_preferences(preferences: &Preferences) -> Result<()> {
	Config::new(APP_ID, 1, None)?
		.set_json::<Preferences>("preferences", preferences.to_owned())?;
	Ok(())
}
