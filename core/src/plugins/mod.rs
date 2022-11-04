use crate::services::provider::provider_client::ProviderClient;
use anyhow::Result;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};
use tonic::transport::Channel;

#[derive(Debug, EnumIter, EnumString, Copy, Clone)]
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
}
