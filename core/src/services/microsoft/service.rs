use futures::StreamExt;
use graph_rs_sdk::{
	oauth::{AccessToken, OAuth},
	Graph,
};
use tokio::sync::mpsc::Sender;

use crate::models::list::List;
use crate::models::task::Task;
use crate::services::microsoft::models::{
	checklist_item::ChecklistItem, collection::Collection, list::TodoTaskList,
	task::TodoTask,
};
use crate::task_service::TodoProvider;
use anyhow::{bail, Result};
use async_trait::async_trait;
use reqwest::StatusCode;
use url::Url;

const APP_ID: &str = "dev.edfloreshz.Done";
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

	fn store_token(&mut self, token: AccessToken) -> Result<()> {
		keytar::set_password(
			"dev.edfloreshz.Done",
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
		keytar::delete_password("dev.edfloreshz.Done", "access_token")?;
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

	fn get_tasks(&mut self, parent_list: String, tx: Sender<Task>) -> Result<()> {
		let mut stream = self
			.client
			.me()
			.todo()
			.list(parent_list.clone())
			.tasks()
			.list_tasks()
			.paging()
			.stream::<Collection<TodoTask>>()?;

		tokio::spawn(async move {
			while let Some(task) = stream.next().await {
				let mut tasks = task?.into_body()?.value;
				for task in tasks {
					let mut task: Task = task.into();
					task.parent = parent_list.clone();
					tx.send(task).await?;
				}
			}
			anyhow::Ok(())
		});

		Ok(())
	}

	async fn read_task(
		&mut self,
		task_list_id: String,
		task_id: String,
	) -> Result<Task> {
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
		let mut todo_task: TodoTask = task.clone().into();
		self
			.update_check_list_items(
				&task.parent,
				&task.id,
				&todo_task.checklist_items,
			)
			.await?;
		todo_task.checklist_items = None;
		let response = self
			.client
			.me()
			.todo()
			.list(task.parent)
			.task(todo_task.id.clone())
			.update_tasks(&serde_json::json!(todo_task))
			.send()
			.await?;

		if response.status() == StatusCode::OK {
			let task: TodoTask = response.json().await?;
			Ok(task.into())
		} else {
			bail!("An error ocurred while updating the list.")
		}
	}

	async fn delete_task(
		&mut self,
		list_id: String,
		task_id: String,
	) -> Result<()> {
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
		let response = self.client.me().todo().lists().list_lists().send().await?;

		let lists: Collection<TodoTaskList> = response.json().await?;
		Ok(lists.value.iter().map(|t| t.clone().into()).collect())
	}

	fn get_lists(&mut self, tx: Sender<List>) -> Result<()> {
		let mut stream = self
			.client
			.me()
			.todo()
			.lists()
			.list_lists()
			.paging()
			.stream::<Collection<TodoTaskList>>()?;

		tokio::spawn(async move {
			while let Some(list) = stream.next().await {
				let mut lists = list?.into_body()?.value;
				for list in lists {
					let list: List = list.into();
					tx.send(list).await?;
				}
			}
			anyhow::Ok(())
		});

		Ok(())
	}

	async fn read_list(&mut self, id: String) -> Result<List> {
		let response = self.client.me().todo().list(id).get_lists().send().await?;
		let list: TodoTaskList = response.json().await?;
		Ok(list.into())
	}

	async fn create_list(&mut self, list: List) -> Result<List> {
		let list: TodoTaskList = list.into();
		let response = self
			.client
			.me()
			.todo()
			.lists()
			.create_lists(&serde_json::json!(list))
			.send()
			.await?;
		if response.status() == StatusCode::CREATED {
			let list: TodoTaskList = response.json().await?;
			Ok(list.into())
		} else {
			bail!("An error ocurred while creating the list.")
		}
	}

	async fn update_list(&mut self, list: List) -> Result<()> {
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
		let response = self
			.client
			.me()
			.todo()
			.list(id)
			.delete_lists()
			.send()
			.await?;
		if response.status() == StatusCode::NO_CONTENT {
			Ok(())
		} else {
			bail!("An error ocurred while deleting the list.")
		}
	}
}
