use tonic::{transport::Server, Request, Response, Status};
use provider::provider_server::{Provider, ProviderServer};
use provider::{ProviderResponse, ProviderRequest, Void};

pub mod provider {
    tonic::include_proto!("provider");
}

#[derive(Debug, Default)]
pub struct LocalService {
    id: String
}

#[tonic::async_trait]
impl Provider for LocalService {
    async fn get_id(&self, _request: Request<Void>) -> Result<Response<String>, Status> {
        Ok(Response::new(self.id.clone()))
    }

    async fn add_task(&self, request: Request<ProviderRequest>) -> Result<Response<ProviderResponse>, Status> {
        println!("Local Service got a request: {:#?}", request);

        let req = request.into_inner();

        let reply = ProviderResponse {
            successful: true,
            message: format!("Task with name \"{}\" added to Local Service", req.task.unwrap().title)
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:5123".parse()?;

    let local_service = LocalService {
        id: "Local Service".to_string()
    };

    Server::builder()
        .add_service(ProviderServer::new(local_service))
        .serve(addr)
        .await?;

    Ok(())
}
