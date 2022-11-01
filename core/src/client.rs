use anyhow::Result;

use anyhow::{Ok, bail};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tonic::transport::Channel;
use uuid::Uuid;
use chrono::Utc;

pub use provider::provider_client::ProviderClient;
pub use provider::provider_client::*;
pub use provider::*;

pub mod provider {
	tonic::include_proto!("provider");
}

impl List {
	pub fn new(
		name: &str,
		icon: &str,
		provider: &str,
	) -> Self {
		let icon = if icon.is_empty() {
			None
		} else {
			Some(icon.to_string())
		};
		Self {
			id: Uuid::new_v4().to_string(),
			name: name.to_string(),
			is_owner: true,
			icon,
			provider: provider.to_string(),
		}
	}
}

impl Task {
	pub fn new(title: String, parent: String) -> Self {
		Self {
			id: Uuid::new_v4().to_string(),
			parent,
			title,
			body: None,
			importance: 0,
			favorite: false,
			is_reminder_on: false,
			due_date: None,
			reminder_date: None,
			completed_on: None,
			status: 0,
			created_date_time: Utc::now().timestamp(),
			last_modified_date_time: Utc::now().timestamp(),
		}
	}
}

impl ProviderRequest {
	pub fn new(list: Option<List>, task: Option<Task>) -> Self {
		Self { list, task }
	}
}

#[derive(Debug, EnumIter)]
pub enum Plugin {
	Local = 7007,
	Google = 6006,
	Microsoft = 3003,
	Nextcloud = 4004,
	None
}

impl Plugin {
	pub fn from_str(provider: &str) -> Plugin {
		match provider {
			"local" => Self::Local,
			"google" => Self::Google,
			"microsoft" => Self::Microsoft,
			"nextcloud" => Self::Nextcloud,
			_ => Self::None,
		}
	}

	pub fn list() -> Vec<Plugin> {
		Plugin::iter().map(|p| p).collect()
	}

	pub async fn connect(&self) -> Result<ProviderClient<Channel>> {
		if let Plugin::None = self {
			bail!("We couldn't find this plugin.");
		}
		let port = *self as i32;
		let url = format!("http://[::1]:{port}");
		let plugin = ProviderClient::connect(url).await?;
		Ok(plugin)
	}
}