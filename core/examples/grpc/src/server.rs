use tonic::{transport::Server, Request, Response, Status};
use provider::provider_server::{Provider, ProviderServer};
use provider::{ProviderTaskResponse, ProviderTaskRequest};

pub mod provider {
    tonic::include_proto!("provider");
}

#[derive(Debug, Default)]
pub struct LocalService {

}

#[tonic::async_trait]
impl Provider for LocalService {
    async fn add_task(&self, request: Request<ProviderTaskRequest>) -> Result<Response<ProviderTaskResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = ProviderTaskResponse {
            successful: true, 
            message: format!("Task with name \"{}\" added", req.task.unwrap().title)
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:5123".parse()?;
    let local_service = LocalService::default();

    Server::builder()
        .add_service(ProviderServer::new(local_service))
        .serve(addr)
        .await?;
    
    Ok(())
}
