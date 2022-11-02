use done_core::provider::provider_server::ProviderServer;
use tonic::transport::Server;

mod service;

use crate::service::GoogleTaskService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = "[::1]:6006".parse()?;

	let google_task_service = GoogleTaskService {
		id: "google".to_string(),
		name: "Google Task".to_string(),
		description: "Google Tasks are stored here.".to_string(),
		icon: "".to_string(),
	};

	Server::builder()
		.add_service(ProviderServer::new(google_task_service))
		.serve(addr)
		.await?;

	Ok(())
}
