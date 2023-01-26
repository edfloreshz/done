use std::path::PathBuf;

use directories::ProjectDirs;
use relm4::factory::AsyncFactoryVecDeque;
use serde::{Deserialize, Serialize};

use crate::{
	application::plugin::Plugin, widgets::service::model::ServiceModel,
};

#[derive(Debug)]

pub struct PreferencesComponentModel {
	pub preferences: Preferences,
	pub service_row_factory: AsyncFactoryVecDeque<ServiceModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Preferences {
	pub plugins: Vec<PluginPreferences>,
	pub color_scheme: ColorScheme,
	pub compact: bool,
}

impl Preferences {
	pub async fn new() -> Self {
		let project = ProjectDirs::from("dev", "edfloreshz", "done").unwrap();
		let plugins: Vec<Plugin> = if let Ok(plugins) = Plugin::get_local() {
			plugins
		} else {
			match Plugin::fetch_remote().await {
				Ok(plugins) => plugins,
				Err(err) => {
					tracing::error!("{err:?}");
					vec![]
				},
			}
		};

		let plugins = plugins
			.iter()
			.map(|plugin| PluginPreferences {
				plugin: plugin.clone(),
				enabled: false,
				installed: false,
				update: false,
				executable: project
					.data_dir()
					.join("bin")
					.join(plugin.process_name.as_str()),
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
	#[serde(default)]
	pub update: bool,
	pub executable: PathBuf,
}
