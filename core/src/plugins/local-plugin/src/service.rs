use crate::database::establish_connection;
use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::models::{QueryableList, QueryableTask};
use crate::schema::lists::dsl::*;
use crate::schema::tasks::dsl::*;
use anyhow::Context;
use done_core::services::provider::provider_server::Provider;
use done_core::services::provider::{
	CountResponse, Empty, List, ListResponse, Task, TaskResponse,
};
use tokio_stream::wrappers::ReceiverStream;
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

	async fn read_task_count_from_list(
		&self,
		request: Request<String>,
	) -> Result<Response<CountResponse>, Status> {
		let id = request.into_inner();
		let mut response = CountResponse::default();

		let send_request = || -> anyhow::Result<i64> {
			let count: i64 = tasks
				.filter(id_task.eq(id))
				.count()
				.get_result(&mut establish_connection()?)?;
			Ok(count)
		};

		match send_request() {
			Ok(value) => {
				response.count = value;
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn read_all_tasks(
		&self,
		_request: Request<Empty>,
	) -> Result<Response<TaskResponse>, Status> {
		todo!();
		// let mut response = TaskResponse::default();
		//
		// let send_request = || -> anyhow::Result<Vec<Task>> {
		// 	let result: Vec<QueryableTask> = tasks
		// 		.load::<QueryableTask>(&mut establish_connection()?)
		// 		.context("Failed to fetch list of tasks.")?;
		// 	let results: Vec<Task> =
		// 		result.iter().map(|t| t.clone().into()).collect();
		// 	Ok(results)
		// };
		//
		// match send_request() {
		// 	Ok(value) => {
		// 		response.task = Some(value);
		// 		response.successful = true;
		// 	},
		// 	Err(err) => response.message = err.to_string(),
		// }
		// Ok(Response::new(response))
	}

	type ReadTasksFromListStream = ReceiverStream<Result<TaskResponse, Status>>;

	async fn read_tasks_from_list(
		&self,
		request: Request<String>,
	) -> Result<Response<Self::ReadTasksFromListStream>, Status> {
		let (tx, rx) = tokio::sync::mpsc::channel(4);
		let id = request.into_inner();

		let send_request = || -> anyhow::Result<Vec<Task>> {
			let result: Vec<QueryableTask> = tasks
				.filter(parent_list.eq(id))
				.load::<QueryableTask>(&mut establish_connection()?)
				.context("Failed to fetch list of tasks.")?;
			let results: Vec<Task> =
				result.iter().map(|t| t.clone().into()).collect();
			Ok(results)
		};

		let mut response = TaskResponse::default();

		tokio::spawn(async move {
			match send_request() {
				Ok(value) => {
					response.successful = true;
					for task in &value[..] {
						let response = TaskResponse {
							successful: true,
							message: "".to_string(),
							task: Some(task.clone()),
						};
						tx.send(Ok(response)).await.unwrap();
					}
				},
				Err(err) => response.message = err.to_string(),
			}
		});

		Ok(Response::new(ReceiverStream::new(rx)))
	}

	async fn create_task(
		&self,
		request: Request<Task>,
	) -> Result<Response<TaskResponse>, Status> {
		let task = request.into_inner();
		let mut response = TaskResponse::default();

		let send_request = || -> anyhow::Result<()> {
			let task: QueryableTask = task.into();

			diesel::insert_into(tasks)
				.values(&task)
				.execute(&mut establish_connection()?)?;

			Ok(())
		};

		match send_request() {
			Ok(()) => {
				response.task = None;
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn read_task(
		&self,
		request: Request<String>,
	) -> Result<Response<TaskResponse>, Status> {
		let id = request.into_inner();
		let mut response = TaskResponse::default();

		let send_request = || -> anyhow::Result<Task> {
			let result: QueryableTask = tasks
				.find(id)
				.first(&mut establish_connection()?)
				.context("Failed to fetch list of tasks.")?;
			Ok(result.into())
		};

		match send_request() {
			Ok(value) => {
				response.task = Some(value);
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn update_task(
		&self,
		request: Request<Task>,
	) -> Result<Response<TaskResponse>, Status> {
		let task = request.into_inner();
		let mut response = TaskResponse::default();

		let send_request = || -> anyhow::Result<()> {
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
				response.task = None;
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn delete_task(
		&self,
		request: Request<String>,
	) -> Result<Response<TaskResponse>, Status> {
		let id = request.into_inner();
		let mut response = TaskResponse::default();

		let send_request = || -> anyhow::Result<()> {
			diesel::delete(tasks.filter(id_task.eq(id)))
				.execute(&mut establish_connection()?)
				.context("Failed to delete task")?;

			Ok(())
		};

		match send_request() {
			Ok(()) => {
				response.task = None;
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	type ReadAllListsStream = ReceiverStream<Result<ListResponse, Status>>;

	async fn read_all_lists(
		&self,
		_request: Request<Empty>,
	) -> Result<Response<Self::ReadAllListsStream>, Status>{
		let (tx, rx) = tokio::sync::mpsc::channel(4);

		let send_request = || -> anyhow::Result<Vec<List>> {
			let results = lists
				.load::<QueryableList>(&mut establish_connection()?)
				.context("Failed to read all lists")?;
			let results: Vec<List> =
				results.iter().map(|t| t.clone().into()).collect();
			Ok(results)
		};

		let mut response = ListResponse::default();

		tokio::spawn(async move {
			match send_request() {
				Ok(value) => {
					response.successful = true;
					for list in &value[..] {
						let response = ListResponse {
							successful: true,
							message: "".to_string(),
							list: Some(list.clone()),
						};
						tx.send(Ok(response)).await.unwrap();
					}
				},
				Err(err) => response.message = err.to_string(),
			}
		});

		Ok(Response::new(ReceiverStream::new(rx)))
	}

	async fn create_list(
		&self,
		request: Request<List>,
	) -> Result<Response<ListResponse>, Status> {
		let list = request.into_inner();
		let mut response = ListResponse::default();

		let send_request = || -> anyhow::Result<()> {
			let list: QueryableList = list.into();

			diesel::insert_into(lists)
				.values(&list)
				.execute(&mut establish_connection()?)
				.context("Failed to create list.")?;

			Ok(())
		};

		match send_request() {
			Ok(()) => {
				response.list = None;
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn read_list(
		&self,
		request: Request<String>,
	) -> Result<Response<ListResponse>, Status> {
		let id = request.into_inner();
		let mut response = ListResponse::default();

		let send_request = || -> anyhow::Result<List> {
			let result: QueryableList =
				lists.find(id).first(&mut establish_connection()?)?;
			Ok(result.into())
		};

		match send_request() {
			Ok(value) => {
				response.list = Some(value);
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn update_list(
		&self,
		request: Request<List>,
	) -> Result<Response<ListResponse>, Status> {
		let list = request.into_inner();
		let mut response = ListResponse::default();

		let send_request = || -> anyhow::Result<()> {
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
				response.list = None;
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}

	async fn delete_list(
		&self,
		request: Request<String>,
	) -> Result<Response<ListResponse>, Status> {
		let id = request.into_inner();
		let mut response = ListResponse::default();

		let send_request = || -> anyhow::Result<()> {
			diesel::delete(lists.filter(id_list.eq(id)))
				.execute(&mut establish_connection()?)?;

			Ok(())
		};

		match send_request() {
			Ok(()) => {
				response.list = None;
				response.successful = true;
			},
			Err(err) => response.message = err.to_string(),
		}
		Ok(Response::new(response))
	}
}
