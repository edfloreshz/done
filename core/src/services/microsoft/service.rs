use std::pin::Pin;

use crate::models::list::List;
use crate::models::task::Task;
use crate::services::microsoft::models::{
	checklist_item::ChecklistItem, collection::Collection, list::TodoTaskList,
	task::TodoTask,
};
use crate::task_service::TodoProvider;
use anyhow::{bail, Result};
use async_trait::async_trait;
use futures::{Stream, StreamExt};
use graph_rs_sdk::{
	oauth::{AccessToken, OAuth},
	Graph,
};
use reqwest::StatusCode;
use url::Url;

pub const APP_ID: &str = "dev.edfloreshz.Done";
const CLIENT_ID: &str = "75d8509b-cf9b-4245-9550-1e5f1d7c66e4";
const REDIRECT_URI: &str = "done://msft";

#[derive(Debug, Clone)]

pub struct MicrosoftService {
	client: Graph,
	token: AccessToken,
}

#[allow(unused)]
impl MicrosoftService {
	pub fn new() -> Self {
		let mut token = AccessToken::default();

		if let Ok(access_token) = keytar::get_password(APP_ID, "access_token") {
			if (!access_token.password.is_empty()) {
				token = serde_json::from_str(access_token.password.as_str()).unwrap();
			}
		}
		Self {
			client: Graph::new(token.bearer_token()),
			token,
		}
	}

	fn oauth_client() -> OAuth {
		let mut oauth = OAuth::new();
		oauth
			.client_id(CLIENT_ID)
			.redirect_uri(REDIRECT_URI)
			.add_scope("offline_access")
			.add_scope("user.read")
			.add_scope("tasks.read")
			.add_scope("tasks.read.shared")
			.add_scope("tasks.readwrite")
			.add_scope("tasks.readwrite.shared")
			.authorize_url(
				"https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize",
			)
			.access_token_url(
				"https://login.microsoftonline.com/consumers/oauth2/v2.0/token",
			)
			.refresh_token_url(
				"https://login.microsoftonline.com/consumers/oauth2/v2.0/token",
			)
			.response_type("code");
		oauth
	}

	async fn refresh_token(&mut self) -> Result<()> {
		if self.token.is_expired() {
			if let Some(refresh_token) = self.token.refresh_token() {
				let mut oauth = Self::oauth_client();
				oauth.access_token(self.token.clone());
				let token: AccessToken = oauth
					.build_async()
					.authorization_code_grant()
					.refresh_token()
					.send()
					.await?
					.json()
					.await?;
				self.store_token(token)?;
			}
		}
		Ok(())
	}

	fn store_token(&mut self, token: AccessToken) -> Result<()> {
		keytar::set_password(
			APP_ID,
			"access_token",
			&serde_json::to_string(&token)?,
		);
		self.token = token;
		Ok(())
	}

	pub async fn request_token(&mut self, access_code: String) -> Result<()> {
		let mut oauth = Self::oauth_client();
		oauth.access_code(access_code.as_str());
		let mut request = oauth.build_async().authorization_code_grant();

		let response = request.access_token().send().await?;

		match response.error_for_status() {
			Ok(response) => {
				let access_token: AccessToken = response.json().await?;
				oauth.access_token(access_token.clone());
				self.store_token(access_token);
				Ok(())
			},
			Err(error) => Err(error.into()),
		}
	}

	pub async fn update_check_list_items(
		&self,
		todo_task_list_id: &str,
		todo_task_id: &str,
		checklist_items: &Option<Vec<ChecklistItem>>,
	) -> Result<()> {
		if let Some(checklist_items) = checklist_items {
			for item in checklist_items {
				let response = self
					.client
					.me()
					.todo()
					.list(todo_task_list_id)
					.task(todo_task_id)
					.update_checklist_items(&item.id, &serde_json::json!(item))
					.send()
					.await?;

				if let Err(err) = response.error_for_status() {
					tracing::error!(
						"There was an error updating a check list item: {}",
						err.to_string()
					)
				}
			}
		}
		Ok(())
	}
}

#[async_trait]
#[allow(unused)]
impl TodoProvider for MicrosoftService {
	async fn handle_uri_params(&mut self, uri: Url) -> Result<()> {
		let mut pairs = uri.query_pairs();
		if uri.as_str().contains("msft") {
			let code = pairs.next().unwrap().1.to_string();
			self.request_token(code).await?;
		}
		Ok(())
	}

	fn login(&self) -> anyhow::Result<()> {
		let mut oauth = MicrosoftService::oauth_client();
		let mut request = oauth.build_async().authorization_code_grant();
		request.browser_authorization().open()?;
		Ok(())
	}

	fn logout(&self) -> anyhow::Result<()> {
		keytar::delete_password(APP_ID, "access_token")?;
		Ok(())
	}

	fn available(&self) -> bool {
		let password = keytar::get_password(APP_ID, "access_token");
		password.is_ok() && !password.unwrap().password.is_empty()
	}

	fn stream_support(&self) -> bool {
		true
	}

	async fn read_tasks(&mut self) -> Result<Vec<Task>> {
		Ok(vec![])
	}

	async fn read_tasks_from_list(
		&mut self,
		parent_list: String,
	) -> Result<Vec<Task>> {
		self.refresh_token().await?;
		let response = self
			.client
			.me()
			.todo()
			.list(parent_list.clone())
			.tasks()
			.list_tasks()
			.send()
			.await?;
		let collection: Collection<TodoTask> = response.json().await?;
		Ok(
			collection
				.value
				.iter()
				.map(|task| {
					let mut task: Task = task.clone().into();
					task.parent = parent_list.clone();
					task
				})
				.collect(),
		)
	}

	async fn get_tasks(
		&mut self,
		parent_list: String,
	) -> Result<Pin<Box<dyn Stream<Item = Task> + Send>>> {
		self.refresh_token().await?;
		let mut stream = self
			.client
			.me()
			.todo()
			.list(parent_list.clone())
			.tasks()
			.list_tasks()
			.paging()
			.stream::<serde_json::Value>()?
			.filter_map(|response| async move {
				match response {
					Ok(response) => {
						let value = response.into_body().ok()?;
						let tasks: Vec<serde_json::Value> =
							value["value"].as_array().cloned()?;

						let task_list = tasks
							.iter()
							.flat_map(|t| serde_json::from_value(t.clone()).ok())
							.flat_map(|t: TodoTask| Some(t.into()))
							.collect::<Vec<Task>>();

						Some(task_list)
					},
					Err(err) => {
						tracing::error!("There was an error getting the tasks: {}", err);
						None
					},
				}
			})
			.flat_map(futures::stream::iter)
			.boxed();

		Ok(stream)
	}

	async fn read_task(
		&mut self,
		task_list_id: String,
		task_id: String,
	) -> Result<Task> {
		self.refresh_token().await?;
		let response = self
			.client
			.me()
			.todo()
			.list(task_list_id.clone())
			.task(task_id)
			.get_tasks()
			.send()
			.await?;
		let task: TodoTask = response.json().await?;
		let mut task: Task = task.clone().into();
		task.parent = task_list_id;
		Ok(task)
	}

	async fn create_task(&mut self, task: Task) -> Result<()> {
		self.refresh_token().await?;
		let todo_task: TodoTask = task.clone().into();
		let response = self
			.client
			.me()
			.todo()
			.list(task.parent)
			.tasks()
			.create_tasks(&serde_json::json!(todo_task))
			.send()
			.await?;

		if response.status() == StatusCode::CREATED {
			Ok(())
		} else {
			bail!("An error ocurred while creating the task.")
		}
	}

	async fn update_task(&mut self, task: Task) -> Result<Task> {
		self.refresh_token().await?;
		let mut todo_task: TodoTask = task.clone().into();
		self
			.update_check_list_items(
				&task.parent,
				&task.id,
				&todo_task.checklist_items,
			)
			.await?;
		todo_task.checklist_items = None;
		println!("{}", serde_json::json!(todo_task));
		let response = self
			.client
			.me()
			.todo()
			.list(task.parent)
			.task(todo_task.id.clone())
			.update_tasks(&serde_json::json!(todo_task))
			.send()
			.await?;

		let status = response.status();
		match response.error_for_status() {
			Ok(response) => {
				let task: TodoTask = response.json().await?;
				Ok(task.into())
			},
			Err(err) => {
				bail!("An error ocurred while updating the list: {err}")
			},
		}
	}

	async fn delete_task(
		&mut self,
		list_id: String,
		task_id: String,
	) -> Result<()> {
		self.refresh_token().await?;
		let response = self
			.client
			.me()
			.todo()
			.list(list_id)
			.task(task_id)
			.delete_tasks()
			.send()
			.await?;
		if response.status() == StatusCode::NO_CONTENT {
			Ok(())
		} else {
			bail!("An error ocurred while deleting the task.")
		}
	}

	async fn read_lists(&mut self) -> Result<Vec<List>> {
		self.refresh_token().await?;
		let response = self.client.me().todo().lists().list_lists().send().await?;

		let lists: Collection<TodoTaskList> = response.json().await?;
		Ok(lists.value.iter().map(|t| t.clone().into()).collect())
	}

	async fn get_lists(
		&mut self,
	) -> Result<Pin<Box<dyn Stream<Item = List> + Send>>> {
		self.refresh_token().await?;
		let mut stream = self
			.client
			.me()
			.todo()
			.lists()
			.list_lists()
			.paging()
			.stream::<serde_json::Value>()?
			.filter_map(|response| async move {
				match response {
					Ok(response) => {
						let value = response.into_body().ok()?;
						let lists: Vec<serde_json::Value> =
							value["value"].as_array().cloned()?;

						let list = lists
							.iter()
							.flat_map(|t| serde_json::from_value(t.clone()).ok())
							.flat_map(|t: TodoTaskList| Some(t.into()))
							.collect::<Vec<List>>();

						Some(list)
					},
					Err(err) => {
						tracing::error!("There was an error getting the tasks: {}", err);
						None
					},
				}
			})
			.flat_map(futures::stream::iter)
			.boxed();

		Ok(stream)
	}

	async fn read_list(&mut self, id: String) -> Result<List> {
		self.refresh_token().await?;
		let response = self.client.me().todo().list(id).get_lists().send().await?;
		let list: TodoTaskList = response.json().await?;
		Ok(list.into())
	}

	async fn create_list(&mut self, list: List) -> Result<List> {
		self.refresh_token().await?;
		let list: TodoTaskList = list.into();
		println!("{}", serde_json::json!(list));
		let response = self
			.client
			.me()
			.todo()
			.lists()
			.create_lists(&serde_json::json!(list))
			.send()
			.await?;
		match response.error_for_status() {
			Ok(response) => {
				let list: TodoTaskList = response.json().await?;
				Ok(list.into())
			},
			Err(err) => bail!("An error ocurred while creating the list: {err}"),
		}
	}

	async fn update_list(&mut self, list: List) -> Result<()> {
		self.refresh_token().await?;
		let list: TodoTaskList = list.into();
		let response = self
			.client
			.me()
			.todo()
			.list(list.id.clone())
			.update_lists(&serde_json::json!(list))
			.send()
			.await?;

		if response.status() == StatusCode::OK {
			Ok(())
		} else {
			bail!("An error ocurred while updating the list.")
		}
	}

	async fn delete_list(&mut self, id: String) -> Result<()> {
		self.refresh_token().await?;
		let response = self
			.client
			.me()
			.todo()
			.list(id)
			.delete_lists()
			.send()
			.await?;

		match response.error_for_status() {
			Ok(_) => Ok(()),
			Err(err) => bail!("An error ocurred while deleting the list: {err}"),
		}
	}
}
