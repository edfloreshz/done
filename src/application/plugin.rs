use anyhow::Result;
use libset::format::FileFormat;
use libset::project::Project;
use proto_rust::provider::provider_client::ProviderClient;
use proto_rust::provider::{Empty, List};
use serde::{Deserialize, Serialize};
use std::process::Command;
use sysinfo::{ProcessExt, System, SystemExt};
use tonic::transport::Channel;

pub const PLUGINS_URL: &str = "https://raw.githubusercontent.com/done-devs/done/beta/dev.edfloreshz.Done.Plugins.json";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
	pub async fn fetch_plugins() -> Result<Vec<Plugin>> {
		let response = reqwest::get(PLUGINS_URL).await?.text().await?;
		let plugins: Vec<Plugin> = serde_json::from_str(&response)?;
		Ok(plugins)
	}

	pub fn get_plugins() -> Result<Vec<Plugin>> {
		let plugins = Project::open("dev", "edfloreshz", "done")?
			.get_file_as::<Vec<Plugin>>(
				"dev.edfloreshz.Done.Plugins",
				FileFormat::JSON,
			)?;
		Ok(plugins)
	}

	pub fn get_by_id(id: &str) -> Result<Plugin> {
		let plugins = Self::get_plugins()?;
		let plugin = plugins
			.into_iter()
			.find(|plugin| plugin.id == id)
			.ok_or(anyhow::anyhow!("Plugin not found."))?;
		Ok(plugin)
	}

	pub fn start(&self) -> Result<u32, std::io::Error> {
		match Command::new(&self.process_name).spawn() {
			Ok(child) => Ok(child.id()),
			Err(err) => Err(err),
		}
	}

	pub fn stop(&self) -> Result<()> {
		let processes = System::new_all();
		match processes.processes_by_exact_name(&self.process_name).next() {
			Some(process) => {
				if process.kill() {
					info!("Process was killed.")
				} else {
					error!("Failed to kill process.")
				}
			},
			None => info!("Process is not running."),
		}
		Ok(())
	}

	pub fn is_running(&self) -> bool {
		let processes = System::new_all();
		let is_running = processes
			.processes_by_exact_name(&self.process_name)
			.next()
			.is_some();
		is_running
	}

	pub fn is_installed(&self) -> bool {
		Command::new(&self.process_name).spawn().ok().is_some()
	}

	pub async fn connect(&self) -> Result<ProviderClient<Channel>> {
		let url = format!("http://[::1]:{}", self.port);
		let plugin = ProviderClient::connect(url).await?;
		Ok(plugin)
	}

	pub async fn lists(&self) -> Result<Vec<String>> {
		let mut connector = self.connect().await?;
		let response = connector.read_all_list_ids(Empty {}).await?.into_inner();
		Ok(response.lists)
	}
}
