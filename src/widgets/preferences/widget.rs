use crate::fl;
use adw::prelude::{BoxExt, GtkWindowExt, OrientableExt, WidgetExt};
use done_local_storage::service::Service;
use libset::format::FileFormat;
use libset::project::Project;
use relm4::adw::prelude::{
	ActionRowExt, AdwWindowExt, PreferencesGroupExt, PreferencesPageExt,
	PreferencesRowExt,
};
use relm4::adw::traits::ComboRowExt;
use relm4::component::{AsyncComponent, AsyncComponentParts};
use relm4::gtk::traits::ButtonExt;
use relm4::AsyncComponentSender;
use relm4::{adw, gtk};
use relm4_icons::icon_name;

use super::helpers::{set_color_scheme, set_extended};
use super::messages::{PreferencesComponentInput, PreferencesComponentOutput};
use super::model::{ColorScheme, Preferences, PreferencesComponentModel};

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
							set_vexpand: true,
							add = &adw::PreferencesGroup {
								set_title: fl!("appearance"),
								adw::ComboRow {
									set_title: fl!("color-scheme"),
									set_subtitle: fl!("color-scheme-description"),
									set_icon_name: Some("dark-mode-symbolic"),
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
								adw::ActionRow {
									set_title: fl!("extended-sidebar"),
									set_subtitle: fl!("extended-sidebar-description"),
									set_icon_name: Some("dock-left-symbolic"),
									add_suffix = &gtk::Box {
										set_halign: gtk::Align::Center,
										set_valign: gtk::Align::Center,
										append = &gtk::Switch {
											set_active: model.preferences.extended,
											connect_state_set[sender] => move |_, state| {
												sender.input(PreferencesComponentInput::ToggleExtended(state));
												gtk::Inhibit::default()
											}
										}
									}
								},
							},
							add = &adw::PreferencesGroup {
								set_title: fl!("services"),
								adw::ActionRow {
									set_title: "Microsoft To Do",
									set_subtitle: fl!("msft-todo-description"),
									set_icon_name: Some(icon_name::CHECKMARK),
									add_suffix = &gtk::Box {
										set_halign: gtk::Align::Center,
										set_valign: gtk::Align::Center,
										gtk::Button {
											set_label: "Login",
											connect_clicked => PreferencesComponentInput::MicrosoftLogin
										}
									}
								}
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
		let preferences =
			if let Ok(project) = Project::open("dev", "edfloreshz", "done") {
				project
					.get_file_as::<Preferences>("preferences", FileFormat::JSON)
					.unwrap_or(Preferences::new().await)
			} else {
				Preferences::new().await
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
				if let Err(err) = set_color_scheme(self, color_scheme) {
					tracing::error!("{err}")
				}
			},
			PreferencesComponentInput::ToggleExtended(mode) => {
				if let Err(err) = set_extended(self, &sender, mode) {
					tracing::error!("{err}")
				}
			},
			PreferencesComponentInput::MicrosoftLogin => {
				match Service::Microsoft.get_service().login() {
					Ok(_) => println!("Login successfull"),
					Err(err) => eprintln!("{err}"),
				};
			},
		}
		self.update_view(widgets, sender);
	}
}
