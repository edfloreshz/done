use anyhow::Result;
use libset::{format::FileFormat, project::Project};
use relm4::{adw, prelude::DynamicIndex, AsyncComponentSender};

use crate::{
	app::toast,
	application::plugin::Plugin,
	widgets::{
		preferences::messages::PreferencesComponentOutput,
		service_row::{messages::ServiceRowInput, model::UpdateStatus},
	},
};

use super::model::{ColorScheme, Preferences, PreferencesComponentModel};

pub async fn enable_plugin(
	model: &mut PreferencesComponentModel,
	index: DynamicIndex,
	sender: &AsyncComponentSender<PreferencesComponentModel>,
	plugin: Plugin,
	overlay: &mut adw::ToastOverlay,
) -> Result<()> {
	let id = plugin.start().await?;
	if let Some(id) = id {
		model.service_row_factory.send(
			index.current_index(),
			ServiceRowInput::UpdateChildId(id.try_into()?),
		);
	}
	tracing::info!("Plugin {:?} started...", plugin);
	overlay.add_toast(&toast("Service enabled.", 1));

	model.preferences.plugins = model
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

	match update_preferences(&model.preferences) {
		Ok(()) => {
			if id.is_some() {
				sender
					.output(PreferencesComponentOutput::EnablePluginOnSidebar(plugin))
					.unwrap();
			}
			model
				.service_row_factory
				.send(index.current_index(), ServiceRowInput::SwitchOn(true));
		},
		Err(e) => tracing::error!("{:?}", e),
	}
	Ok(())
}

pub fn disable_plugin(
	model: &mut PreferencesComponentModel,
	index: DynamicIndex,
	sender: &AsyncComponentSender<PreferencesComponentModel>,
	plugin: Plugin,
	process_id: usize,
	overlay: &mut adw::ToastOverlay,
) {
	plugin.stop(process_id);
	tracing::info!("Plugin {:?} stopped.", plugin);
	let previous_model = model.preferences.clone();
	model.preferences.plugins = model
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
	if previous_model != model.preferences {
		overlay.add_toast(&toast("Service disabled.", 1));
		match update_preferences(&model.preferences) {
			Ok(()) => {
				sender
					.output(PreferencesComponentOutput::DisablePluginOnSidebar(plugin))
					.unwrap();
				model
					.service_row_factory
					.send(index.current_index(), ServiceRowInput::SwitchOn(false));
			},
			Err(e) => tracing::error!("{:?}", e),
		}
	}
}

pub async fn install_plugin(
	model: &mut PreferencesComponentModel,
	index: DynamicIndex,
	sender: &AsyncComponentSender<PreferencesComponentModel>,
	plugin: Plugin,
	overlay: &mut adw::ToastOverlay,
) -> Result<()> {
	let install_plugin = plugin.clone();
	match install_plugin.install().await {
		Ok(_) => {
			if let Some(plugin) = model
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
			update_preferences(&model.preferences)?;
			sender
				.output_sender()
				.send(PreferencesComponentOutput::AddPluginToSidebar(
					plugin.clone(),
				))
				.unwrap();
			model.service_row_factory.send(
				index.current_index(),
				ServiceRowInput::ShowInstallButton(false),
			);
			model
				.service_row_factory
				.send(index.current_index(), ServiceRowInput::SwitchOn(true));
		},
		Err(err) => {
			tracing::error!("Failed to install plugin: {}", err.to_string());
			overlay.add_toast(&toast(err, 2))
		},
	}
	Ok(())
}

pub fn remove_plugin(
	model: &mut PreferencesComponentModel,
	index: DynamicIndex,
	sender: &AsyncComponentSender<PreferencesComponentModel>,
	plugin: Plugin,
	process_id: usize,
) {
	plugin.stop(process_id);
	if let Some(preferences) = model
		.preferences
		.plugins
		.iter_mut()
		.find(|preferences| preferences.plugin == plugin)
	{
		match std::fs::remove_file(&preferences.executable) {
			Ok(_) => {
				preferences.enabled = false;
				preferences.installed = false;
				match update_preferences(&model.preferences) {
					Ok(_) => {
						model
							.service_row_factory
							.send(index.current_index(), ServiceRowInput::SwitchOn(false));
						model.service_row_factory.send(
							index.current_index(),
							ServiceRowInput::ShowInstallButton(true),
						);
						sender
							.output(PreferencesComponentOutput::RemovePluginFromSidebar(
								plugin,
							))
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
}

pub async fn update_plugin(
	model: &mut PreferencesComponentModel,
	index: DynamicIndex,
	process_id: usize,
	plugin: Plugin,
) {
	match plugin.try_update(process_id).await {
		Ok(_) => model.service_row_factory.send(
			index.current_index(),
			ServiceRowInput::InformStatus(UpdateStatus::Completed),
		),
		Err(err) => {
			tracing::error!("Failed to update plugin: {}", err.to_string());
			model.service_row_factory.send(
				index.current_index(),
				ServiceRowInput::InformStatus(UpdateStatus::Failed),
			)
		},
	}
}

pub fn set_color_scheme(
	model: &mut PreferencesComponentModel,
	color_scheme: ColorScheme,
) -> Result<()> {
	match color_scheme {
		ColorScheme::Dark => {
			adw::StyleManager::default()
				.set_color_scheme(adw::ColorScheme::ForceDark);
			model.preferences.color_scheme = ColorScheme::Dark;
		},
		ColorScheme::Light => {
			adw::StyleManager::default()
				.set_color_scheme(adw::ColorScheme::ForceLight);
			model.preferences.color_scheme = ColorScheme::Light;
		},
		ColorScheme::Default => {
			adw::StyleManager::default().set_color_scheme(adw::ColorScheme::Default);
			model.preferences.color_scheme = ColorScheme::Default;
		},
	}
	update_preferences(&model.preferences)
}

pub fn set_compact(
	model: &mut PreferencesComponentModel,
	sender: &AsyncComponentSender<PreferencesComponentModel>,
	compact: bool,
) -> Result<()> {
	model.preferences.compact = compact;
	update_preferences(&model.preferences)?;
	sender
		.output(PreferencesComponentOutput::ToggleCompact(
			model.preferences.compact,
		))
		.unwrap();
	Ok(())
}

fn update_preferences(preferences: &Preferences) -> Result<()> {
	Project::open("dev", "edfloreshz", "done")?
		.get_file("preferences", FileFormat::JSON)?
		.set_content(preferences)?
		.write()
}

pub async fn has_update(local_plugin: &Plugin) -> Result<bool> {
	let remote_plugins = Plugin::fetch_remote().await?;
	if let Some(remote_plugin) =
		remote_plugins.iter().find(|r| r.name == local_plugin.name)
	{
		if local_plugin.version != remote_plugin.version {
			return Ok(true);
		}
	}
	Ok(false)
}
