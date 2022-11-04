use done_core::services::provider::provider_server::{Provider};
use done_core::services::provider::{Empty, ProviderRequest, ProviderResponse, Task};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct GoogleTaskService {
	pub id: String,
	pub name: String,
	pub description: String,
	pub icon: String,
}

#[tonic::async_trait]
impl Provider for GoogleTaskService {
	async fn get_id(
		&self,
		_request: Request<Empty>,
	) -> Result<Response<String>, Status> {
		Ok(Response::new(self.id.clone()))
	}

	async fn get_name(
		&self,
		_request: Request<Empty>,
	) -> Result<Response<String>, Status> {
		Ok(Response::new(self.name.clone()))
	}

	async fn get_description(
		&self,
		_request: Request<Empty>,
	) -> Result<Response<String>, Status> {
		Ok(Response::new(self.description.clone()))
	}

	async fn get_icon_name(
		&self,
		_request: Request<Empty>,
	) -> Result<Response<String>, Status> {
		Ok(Response::new(self.icon.clone()))
	}

	async fn read_all_tasks(
		&self,
		request: Request<Empty>,
	) -> Result<Response<ProviderResponse>, Status> {
		todo!()
	}

	async fn read_tasks_from_list(
		&self,
		request: Request<ProviderRequest>,
	) -> Result<Response<ProviderResponse>, Status> {
		todo!()
	}

	async fn create_task(
		&self,
		request: Request<ProviderRequest>,
	) -> Result<Response<ProviderResponse>, Status> {
		println!("GoogleTask Service got a request: {:#?}", request);

		let req = request.into_inner();

		let reply = ProviderResponse {
			successful: true,
			message: format!(
				"Task with name \"{}\" added to Google Task Service",
				req.task.unwrap().title
			),
			data: None,
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
		request: Request<Empty>,
	) -> Result<Response<ProviderResponse>, Status> {
		let req = request.into_inner();

		let reply = ProviderResponse {
			successful: true,
			message: format!("read_all_lists"),
			data: Some(serde_json::to_string::<Vec<Task>>(&vec![]).unwrap()),
		};
		Ok(Response::new(reply))
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
