use anyhow::{Context, Result};
use directories::ProjectDirs;
use done_provider::provider::provider_client::ProviderClient;
use done_provider::provider::List;
use libset::format::FileFormat;
use libset::project::Project;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;
use std::{io::Write, time::Duration};
use sysinfo::{ProcessExt, System, SystemExt};
use tonic::transport::Channel;

use crate::widgets::preferences::model::Preferences;

pub const PLUGINS_URL: &str = "https://raw.githubusercontent.com/done-devs/done/main/dev.edfloreshz.Done.Plugins.json";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Plugin {
	#[serde(rename = "pluginId")]
	pub id: String,
	#[serde(rename = "pluginName")]
	pub name: String,
	#[serde(rename = "pluginDescription")]
	pub description: String,
	#[serde(rename = "pluginIcon")]
	pub icon: String,
	#[serde(rename = "pluginPort")]
	pub port: u32,
	#[serde(rename = "pluginVersion")]
	pub version: String,
	#[serde(rename = "pluginDownloadUrl")]
	pub download_url: String,
	#[serde(rename = "pluginProcessName")]
	pub process_name: String,
	#[serde(skip)]
	pub lists: Vec<List>,
}

impl Plugin {
	pub async fn fetch_remote() -> Result<Vec<Plugin>> {
		let response = relm4::spawn(async move {
			reqwest::get(PLUGINS_URL)
				.await
				.unwrap()
				.text()
				.await
				.unwrap()
		})
		.await?;
		let plugins: Vec<Plugin> = serde_json::from_str(&response)?;
		let project = Project::open("dev", "edfloreshz", "done")?;
		project
			.get_file("dev.edfloreshz.Done.Plugins.json", FileFormat::JSON)?
			.set_content(plugins.clone())?
			.write()?;
		Ok(plugins)
	}

	pub fn get_local() -> Result<Vec<Plugin>> {
		let preferences = Project::open("dev", "edfloreshz", "done")?
			.get_file_as::<Preferences>("preferences", FileFormat::JSON)?;
		let plugins = preferences
			.plugins
			.iter()
			.map(|pref| pref.plugin.clone())
			.collect();
		Ok(plugins)
	}

	pub async fn start(&self) -> Result<(), std::io::Error> {
		let project = ProjectDirs::from("dev", "edfloreshz", "done").unwrap();
		let process_name = self.process_name.clone();
		let system = System::new_all();

		for process in system.processes_by_exact_name(&process_name) {
			process.kill();
		}

		relm4::spawn(async move {
			let mut command =
				tokio::process::Command::new(format!("./{}", process_name));
			command.current_dir(project.data_dir().join("bin"));
			match command.spawn() {
				Ok(_) => Ok(()),
				Err(err) => Err(err),
			}
		})
		.await?
	}

	pub fn stop(&self, name: &str) {
		for process in System::new_all().processes_by_exact_name(name) {
			if process.kill() {
				if process.kill() {
					tracing::info!("Process was killed.");
				} else {
					tracing::error!("Failed to kill process.");
				}
			}
		}
	}

	pub fn is_running(&self) -> bool {
		let processes = System::new_all();
		let is_running = processes
			.processes_by_exact_name(&self.process_name)
			.next()
			.is_some();
		is_running
	}

	pub async fn install(&mut self) -> Result<()> {
		let project = Project::open("dev", "edfloreshz", "done")?;
		let remote_plugins = project.get_file_as::<Vec<Plugin>>(
			"dev.edfloreshz.Done.Plugins.json",
			FileFormat::JSON,
		)?;
		let mut local_plugins = project
			.get_file_as::<Preferences>("preferences.json", FileFormat::JSON)?;

		let remote_plugin = remote_plugins
			.iter()
			.find(|plugin| plugin.id == self.id)
			.context("Plugin does not exist")?;

		download_file(
			&remote_plugin.download_url,
			project.path().unwrap().join("bin").join(&self.process_name),
		)
		.await?;

		self.download_url = remote_plugin.download_url.clone();
		self.version = remote_plugin.version.clone();

		for plugin in local_plugins.plugins.iter_mut() {
			if plugin.plugin.id == *self.id {
				plugin.plugin = self.clone()
			}
		}

		project
			.get_file("preferences.json", FileFormat::JSON)?
			.set_content(local_plugins)?
			.write()?;

		Ok(())
	}

	pub fn is_installed(&self) -> bool {
		let project = ProjectDirs::from("dev", "edfloreshz", "done").unwrap();
		project
			.data_dir()
			.join("bin")
			.join(&self.process_name)
			.exists()
	}

	pub async fn connect(&self) -> Result<ProviderClient<Channel>> {
		let port = self.port;
		let client = relm4::spawn(async move {
			match ProviderClient::connect(format!("http://[::1]:{}", port)).await {
				Ok(client) => Ok(client),
				Err(err) => {
					std::thread::sleep(Duration::from_secs(1));
					match ProviderClient::connect(format!("http://[::1]:{}", port)).await
					{
						Ok(client) => Ok(client),
						Err(_) => Err(err.into()),
					}
				},
			}
		})
		.await?;
		client
	}

	pub async fn try_update(&mut self) -> Result<()> {
		self.stop(&self.process_name);
		self.install().await?;
		self.start().await?;
		Ok(())
	}
}

// Download a file from a URL and save it to a file
async fn download_file(url: &str, path: PathBuf) -> Result<()> {
	let client = Client::new();
	let url = url.to_string();
	let response =
		relm4::spawn(async move { client.get(url).send().await.unwrap() }).await?;
	let status = response.status();
	if status == 200 {
		let bytes =
			relm4::spawn(async move { response.bytes().await.unwrap() }).await?;
		std::fs::create_dir_all(path.parent().unwrap())?;
		let mut file = std::fs::File::create(path.clone())?;
		file
			.set_permissions(std::fs::Permissions::from_mode(0o755))
			.unwrap();
		file.write_all(&bytes)?;
		Ok(())
	} else {
		Err(anyhow::anyhow!("This service is unavailable."))
	}
}
