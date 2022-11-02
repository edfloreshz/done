use done_core::provider::provider_server::ProviderServer;
use tonic::transport::Server;

mod service;

use crate::service::MicrosoftToDoService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = "[::1]:3003".parse()?;

	let microsoft_to_do_service = MicrosoftToDoService {
		id: "microsoft".to_string(),
		name: "Microsoft To Do".to_string(),
		description: "Microsoft To Do tasks are stored here.".to_string(),
		icon: "".to_string(),
	};

	Server::builder()
		.add_service(ProviderServer::new(microsoft_to_do_service))
		.serve(addr)
		.await?;

	Ok(())
}
