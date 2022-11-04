use done_core::services::provider::provider_server::ProviderServer;
use tonic::transport::Server;

mod service;

use crate::service::NextcloudService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = "[::1]:4004".parse()?;

	let nextcloud_to_do_service = NextcloudService {
		id: "nextcloud".to_string(),
		name: "Nextcloud".to_string(),
		description: "Nextcloud tasks are stored here.".to_string(),
		icon: "".to_string(),
	};

	Server::builder()
		.add_service(ProviderServer::new(nextcloud_to_do_service))
		.serve(addr)
		.await?;

	Ok(())
}
