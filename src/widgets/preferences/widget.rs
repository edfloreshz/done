use crate::application::plugin::Plugin;
use crate::fl;
use crate::widgets::preferences::helpers::has_update;
use adw::prelude::{BoxExt, GtkWindowExt, OrientableExt, WidgetExt};
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
use relm4::AsyncComponentSender;
use relm4::{adw, gtk};

use super::helpers::{
	disable_plugin, enable_plugin, install_plugin, set_color_scheme, set_compact,
	uninstall_plugin, update_plugin,
};
use super::messages::{PreferencesComponentInput, PreferencesComponentOutput};
use super::model::{
	ColorScheme, PluginPreferences, Preferences, PreferencesComponentModel,
};

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
											0 => sender.input_sender().send(PreferencesComponentInput::SetColorScheme(ColorScheme::Light)).unwrap(),
											1 => sender.input_sender().send(PreferencesComponentInput::SetColorScheme(ColorScheme::Dark)).unwrap(),
											_ => sender.input_sender().send(PreferencesComponentInput::SetColorScheme(ColorScheme::Default)).unwrap(),
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
					.unwrap_or(Preferences::new().await)
			} else {
				Preferences::new().await
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

		let project = ProjectDirs::from("dev", "edfloreshz", "done").unwrap();

		let old_plugins = model.preferences.plugins.clone();

		model.preferences.plugins.clear();

		for plugin in Plugin::get_local().unwrap() {
			let has_update = match has_update(&plugin).await {
				Ok(has_update) => has_update,
				Err(err) => {
					tracing::error!("Failed to fetch updates: {err}");
					false
				},
			};
			let plugin_search = old_plugins.iter().find(|p| p.plugin == plugin);
			let plugin_enabled =
				plugin_search.is_some() && plugin_search.unwrap().enabled;
			if plugin_enabled {
				sender
					.output(PreferencesComponentOutput::EnablePluginOnSidebar(
						plugin.clone(),
					))
					.unwrap()
			}
			let preferences = PluginPreferences {
				plugin: plugin.clone(),
				enabled: plugin_enabled,
				installed: plugin.clone().is_installed(),
				update: has_update,
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
				if let Err(err) =
					enable_plugin(self, index, &sender, plugin, &mut widgets.overlay)
						.await
				{
					tracing::error!("{err}")
				}
			},
			PreferencesComponentInput::DisablePlugin(index, plugin) => {
				disable_plugin(self, index, &sender, plugin, &mut widgets.overlay)
			},
			PreferencesComponentInput::InstallPlugin(index, plugin) => {
				if let Err(err) =
					install_plugin(self, index, &sender, plugin, &mut widgets.overlay)
						.await
				{
					tracing::error!("{err}")
				}
			},
			PreferencesComponentInput::RemovePlugin(index, plugin) => {
				uninstall_plugin(self, index, &sender, plugin, &mut widgets.overlay)
			},
			PreferencesComponentInput::UpdatePlugin(index, plugin) => {
				update_plugin(self, index, plugin).await
			},
			PreferencesComponentInput::SetColorScheme(color_scheme) => {
				if let Err(err) = set_color_scheme(self, color_scheme) {
					tracing::error!("{err}")
				}
			},
			PreferencesComponentInput::ToggleCompact(compact) => {
				if let Err(err) = set_compact(self, &sender, compact) {
					tracing::error!("{err}")
				}
			},
		}
		self.update_view(widgets, sender);
	}
}
