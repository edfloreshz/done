use crate::database::establish_connection;
use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::models::{QueryableList, QueryableTask};
use crate::schemas::lists::dsl::*;
use crate::schemas::tasks::dsl::*;
use anyhow::{anyhow, Context};
use done_core::provider::provider_server::Provider;
use done_core::provider::{
	Empty, List, ProviderRequest, ProviderResponse, Task,
};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct LocalService {
	pub id: String,
	pub name: String,
	pub description: String,
	pub icon: String,
}

#[tonic::async_trait]
impl Provider for LocalService {
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
		_request: Request<Empty>,
	) -> Result<Response<ProviderResponse>, Status> {
		let mut response = ProviderResponse::default();

		let send_request = || -> anyhow::Result<String> {
			let result: Vec<QueryableTask> = tasks
				.load::<QueryableTask>(&mut establish_connection()?)
				.context("Failed to fetch list of tasks.")?;
			let results: Vec<Task> =
				result.iter().map(|t| t.clone().into()).collect();
			let value = serde_json::to_string(&results)
				.context("Failed to serialize response.")?;
			Ok(value)
		};

		match send_request() {
			Ok(value) => {
				response.data = Some(value);
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn read_tasks_from_list(
		&self,
		request: Request<ProviderRequest>,
	) -> Result<Response<ProviderResponse>, Status> {
		let request = request.into_inner();
		let mut response = ProviderResponse::default();

		let send_request = || -> anyhow::Result<String> {
			let list = request.list.ok_or_else(|| {
				anyhow!("List parameter is required and it was not provided.")
			})?;
			let result: Vec<QueryableTask> = tasks
				.filter(parent_list.eq(list.id))
				.load::<QueryableTask>(&mut establish_connection()?)
				.context("Failed to fetch list of tasks.")?;
			let results: Vec<Task> =
				result.iter().map(|t| t.clone().into()).collect();
			let value = serde_json::to_string(&results)
				.context("Failed to serialize response.")?;
			Ok(value)
		};

		match send_request() {
			Ok(value) => {
				response.data = Some(value);
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn create_task(
		&self,
		request: Request<ProviderRequest>,
	) -> Result<Response<ProviderResponse>, Status> {
		let request = request.into_inner();
		let mut response = ProviderResponse::default();

		let send_request = || -> anyhow::Result<()> {
			let task = request.task.ok_or_else(|| {
				anyhow!("Task parameter is required and it was not provided")
			})?;
			let task: QueryableTask = task.into();

			diesel::insert_into(tasks)
				.values(&task)
				.execute(&mut establish_connection()?)
				.context("Failed to create task.")?;

			Ok(())
		};

		match send_request() {
			Ok(()) => {
				response.data = None;
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn read_task(
		&self,
		request: Request<ProviderRequest>,
	) -> Result<Response<ProviderResponse>, Status> {
		let request = request.into_inner();
		let mut response = ProviderResponse::default();

		let send_request = || -> anyhow::Result<String> {
			let task = request.task.ok_or_else(|| {
				anyhow!("Task parameter is required and it was not provided")
			})?;
			let result: Vec<QueryableTask> = tasks
				.filter(id_task.eq(task.id))
				.load::<QueryableTask>(&mut establish_connection()?)
				.context("Failed to fetch list of tasks.")?;
			let results: Vec<Task> =
				result.iter().map(|t| t.clone().into()).collect();
			let value = serde_json::to_string(&results)
				.context("Failed to serialize response.")?;
			Ok(value)
		};

		match send_request() {
			Ok(value) => {
				response.data = Some(value);
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn update_task(
		&self,
		request: Request<ProviderRequest>,
	) -> Result<Response<ProviderResponse>, Status> {
		let request = request.into_inner();
		let mut response = ProviderResponse::default();

		let send_request = || -> anyhow::Result<()> {
			let task = request.task.ok_or_else(|| {
				anyhow!("Task parameter is required and it was not provided")
			})?;
			let task: QueryableTask = task.into();

			diesel::update(tasks.filter(id_task.eq(task.id_task.clone())))
				.set((
					id_task.eq(task.id_task),
					title.eq(task.title),
					body.eq(task.body),
					completed_on.eq(task.completed_on),
					due_date.eq(task.due_date),
					importance.eq(task.importance),
					favorite.eq(task.favorite),
					is_reminder_on.eq(task.is_reminder_on),
					reminder_date.eq(task.reminder_date),
					status.eq(task.status),
					created_date_time.eq(task.created_date_time),
					last_modified_date_time.eq(task.last_modified_date_time),
				))
				.execute(&mut establish_connection()?)
				.context("Failed to update task.")?;

			Ok(())
		};

		match send_request() {
			Ok(()) => {
				response.data = None;
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn delete_task(
		&self,
		request: Request<ProviderRequest>,
	) -> Result<Response<ProviderResponse>, Status> {
		let request = request.into_inner();
		let mut response = ProviderResponse::default();

		let send_request = || -> anyhow::Result<()> {
			let task = request.task.ok_or_else(|| {
				anyhow!("Task parameter is required and it was not provided")
			})?;

			diesel::delete(tasks.filter(id_task.eq(task.id)))
				.execute(&mut establish_connection()?)
				.context("Failed to delete task")?;

			Ok(())
		};

		match send_request() {
			Ok(()) => {
				response.data = None;
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn read_all_lists(
		&self,
		_request: Request<Empty>,
	) -> Result<Response<ProviderResponse>, Status> {
		let mut response = ProviderResponse::default();

		let send_request = || -> anyhow::Result<String> {
			let results = lists
				.load::<QueryableList>(&mut establish_connection()?)
				.context("Failed to read all lists")?;
			let results: Vec<List> =
				results.iter().map(|t| t.clone().into()).collect();
			let value = serde_json::to_string(&results)
				.context("Failed to serialize response.")?;
			Ok(value)
		};

		match send_request() {
			Ok(value) => {
				response.data = Some(value);
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn create_list(
		&self,
		request: Request<ProviderRequest>,
	) -> Result<Response<ProviderResponse>, Status> {
		let request = request.into_inner();
		let mut response = ProviderResponse::default();

		let send_request = || -> anyhow::Result<()> {
			let list = request.list.ok_or_else(|| {
				anyhow!("List parameter is required and it was not provided.")
			})?;
			let list: QueryableList = list.into();

			diesel::insert_into(lists)
				.values(&list)
				.execute(&mut establish_connection()?)
				.context("Failed to create list.")?;

			Ok(())
		};

		match send_request() {
			Ok(()) => {
				response.data = None;
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn read_list(
		&self,
		request: Request<ProviderRequest>,
	) -> Result<Response<ProviderResponse>, Status> {
		let request = request.into_inner();
		let mut response = ProviderResponse::default();

		let send_request = || -> anyhow::Result<String> {
			let list = request.list.ok_or_else(|| {
				anyhow!("List parameter is required and it was not provided.")
			})?;
			let result: Vec<QueryableList> = lists
				.filter(id_list.eq(list.id))
				.load::<QueryableList>(&mut establish_connection()?)
				.context("Failed to fetch list.")?;
			let results: Vec<List> =
				result.iter().map(|t| t.clone().into()).collect();
			let value = serde_json::to_string(&results)
				.context("Failed to serialize response.")?;
			Ok(value)
		};

		match send_request() {
			Ok(value) => {
				response.data = Some(value);
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn update_list(
		&self,
		request: Request<ProviderRequest>,
	) -> Result<Response<ProviderResponse>, Status> {
		let request = request.into_inner();
		let mut response = ProviderResponse::default();

		let send_request = || -> anyhow::Result<()> {
			let list = request.list.ok_or_else(|| {
				anyhow!("List parameter is required and it was not provided.")
			})?;
			let list: QueryableList = list.into();

			diesel::update(lists.filter(id_list.eq(list.id_list.clone())))
				.set((
					name.eq(list.name.clone()),
					is_owner.eq(list.is_owner),
					icon_name.eq(list.icon_name),
					provider.eq(list.provider),
				))
				.execute(&mut establish_connection()?)
				.context("Failed to update list.")?;

			Ok(())
		};

		match send_request() {
			Ok(()) => {
				response.data = None;
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn delete_list(
		&self,
		request: Request<ProviderRequest>,
	) -> Result<Response<ProviderResponse>, Status> {
		let request = request.into_inner();
		let mut response = ProviderResponse::default();

		let send_request = || -> anyhow::Result<()> {
			let list = request.list.ok_or_else(|| {
				anyhow!("List parameter is required and it was not provided.")
			})?;

			diesel::delete(tasks.filter(id_task.eq(list.id)))
				.execute(&mut establish_connection()?)
				.context("Failed to delete list")?;

			Ok(())
		};

		match send_request() {
			Ok(()) => {
				response.data = None;
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}
}
