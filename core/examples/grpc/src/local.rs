use tonic::{transport::Server, Request, Response, Status};
use provider::provider_server::{Provider, ProviderServer};
use provider::{ProviderResponse, ProviderRequest, Void};

pub mod provider {
    tonic::include_proto!("provider");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:5123".parse()?;

    let local_service = LocalService {
        id: "Local Service".to_string(),
        name: "Local".to_string(),
        description: "Local tasks are stored here.".to_string(),
        icon: "".to_string()
    };

    Server::builder()
        .add_service(ProviderServer::new(local_service))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct LocalService {
    id: String,
    name: String,
    description: String,
    icon: String,
}

#[tonic::async_trait]
impl Provider for LocalService {
    async fn get_id(
        &self,
        _request: Request<Void>,
    ) -> Result<Response<String>, Status> {
        Ok(Response::new(self.id.clone()))
    }

    async fn get_name(
        &self,
        _request: Request<Void>,
    ) -> Result<Response<String>, Status> {
        Ok(Response::new(self.name.clone()))
    }

    async fn get_description(
        &self,
        _request: Request<Void>,
    ) -> Result<Response<String>, Status> {
        Ok(Response::new(self.description.clone()))
    }

    async fn get_icon_name(
        &self,
        _request: Request<Void>,
    ) -> Result<Response<String>, Status> {
        Ok(Response::new(self.icon.clone()))
    }

    async fn read_all_tasks(
        &self,
        request: Request<ProviderRequest>,
    ) -> Result<Response<ProviderResponse>, Status> {
        todo!()
    }

    async fn create_task(&self, request: Request<ProviderRequest>) -> Result<Response<ProviderResponse>, Status> {
        println!("Local Service got a request: {:#?}", request);

        let req = request.into_inner();

        let reply = ProviderResponse {
            successful: true,
            message: format!("Task with name \"{}\" added to Local Service", req.task.unwrap().title)
        };
        Ok(Response::new(reply))
    }

    async fn read_task(
        &self,
        request: Request<ProviderRequest>,
    ) -> Result<Response<ProviderResponse>, Status> {
        todo!()
    }

    async fn update_task(
        &self,
        request: Request<ProviderRequest>,
    ) -> Result<Response<ProviderResponse>, Status> {
        todo!()
    }

    async fn delete_task(
        &self,
        request: Request<ProviderRequest>,
    ) -> Result<Response<ProviderResponse>, Status> {
        todo!()
    }

    async fn read_all_lists(
        &self,
        request: Request<ProviderRequest>,
    ) -> Result<Response<ProviderResponse>, Status> {
        todo!()
    }

    async fn create_list(
        &self,
        request: Request<ProviderRequest>,
    ) -> Result<Response<ProviderResponse>, Status> {
        todo!()
    }

    async fn read_list(
        &self,
        request: Request<ProviderRequest>,
    ) -> Result<Response<ProviderResponse>, Status> {
        todo!()
    }

    async fn update_list(
        &self,
        request: Request<ProviderRequest>,
    ) -> Result<Response<ProviderResponse>, Status> {
        todo!()
    }

    async fn delete_list(
        &self,
        request: Request<ProviderRequest>,
    ) -> Result<Response<ProviderResponse>, Status> {
        todo!()
    }
}
