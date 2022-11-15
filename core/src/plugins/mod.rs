use crate::services::provider::provider_client::ProviderClient;
use anyhow::Result;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use tonic::transport::Channel;
use crate::services::provider::{Empty, List};

#[derive(Debug, Clone)]
pub struct PluginData {
	pub plugin: Plugin,
	pub id: String,
	pub name: String,
	pub description: String,
	pub icon: String,
	pub lists: Vec<List>
}

#[derive(Debug, EnumIter, EnumString, Display, Copy, Clone)]
pub enum Plugin {
	Local = 7007,
	Google = 6006,
	Microsoft = 3003,
	Nextcloud = 4004,
}

impl Plugin {
	pub fn list() -> Vec<Plugin> {
		Plugin::iter().collect()
	}

	pub async fn connect(&self) -> Result<ProviderClient<Channel>> {
		let port = *self as i32;
		let url = format!("http://[::1]:{port}");
		let plugin = ProviderClient::connect(url).await?;
		Ok(plugin)
	}

	pub async fn connected_count() -> i64 {
		let mut count = 0;
		for plugin in Plugin::list() {
			if plugin.connect().await.is_ok() {
				count += 1;
			}
		}
		count
	}

	pub async fn data(&self) -> Result<PluginData> {
		let mut connector = self.connect().await?;
		let mut stream = connector.read_all_lists(Empty {}).await?.into_inner();
		let mut lists = vec![];
		while let Some(msg) = stream.message().await.unwrap() {
			lists.push(msg.list.unwrap());
		}
		let data = PluginData {
			plugin: self.clone(),
			id: connector.get_id(Empty {}).await?.into_inner(),
			name: connector.get_name(Empty {}).await?.into_inner(),
			description: connector.get_description(Empty {}).await?.into_inner(),
			icon: connector.get_icon_name(Empty {}).await?.into_inner(),
			lists
		};
		Ok(data)
	}

	pub fn dummy(&self) -> PluginData {
		PluginData {
			plugin: self.clone(),
			id: "".to_string(),
			name: self.to_string(),
			description: "".to_string(),
			icon: "".to_string(),
			lists: vec![]
		}
	}
}
