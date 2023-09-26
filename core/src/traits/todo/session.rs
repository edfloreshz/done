use anyhow::Result;
use async_trait::async_trait;
use url::Url;

#[async_trait]
pub trait TodoSession: Sync + Send {
	/// Sets the initial config for this service.
	async fn handle_uri_params(&mut self, uri: Url) -> Result<()>;

	/// Handles the login action.
	fn login(&self) -> Result<()>;

	/// Handles the logout action.
	fn logout(&self) -> Result<()>;

	/// Checks to see if the service is available.
	fn available(&self) -> bool;

	/// Checks to see if the service is available.
	fn stream_support(&self) -> bool;
}
