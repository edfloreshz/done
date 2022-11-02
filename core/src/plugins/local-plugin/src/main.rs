#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use done_core::provider::provider_server::ProviderServer;
use tonic::transport::Server;

mod service;
mod database;
mod schema;

use service::LocalService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = "[::1]:7007".parse()?;

	let local_service = LocalService {
		id: "local".to_string(),
		name: "Local".to_string(),
		description: "Stores tasks on your computer.".to_string(),
		icon: "home".to_string(),
	};

	Server::builder()
		.add_service(ProviderServer::new(local_service))
		.serve(addr)
		.await?;

	Ok(())
}

