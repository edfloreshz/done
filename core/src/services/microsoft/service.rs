use std::collections::HashMap;

use crate::models::list::List;
use crate::models::task::Task;
use crate::task_service::TaskService;
use anyhow::{bail, Result};
use async_trait::async_trait;
use cascade::cascade;
use msft_todo_types::{
	collection::Collection, list::ToDoTaskList, task::ToDoTask, token::Token,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use url::Url;

const APP_ID: &str = "dev.edfloreshz.Done";
const CLIENT_ID: &str = "75d8509b-cf9b-4245-9550-1e5f1d7c66e4";
const REDIRECT_URI: &str = "done://msft";
const API_PERMISSIONS: &str = "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared";

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Microsoft {
	token: Token,
	code: String,
}

#[allow(unused)]
impl Microsoft {
	pub fn new() -> Self {
		let mut model = Self::default();
		if let Ok(access_token) = keytar::get_password(APP_ID, "msft_access_token")
		{
			if (!access_token.password.is_empty()) {
				model.token.access_token = access_token.password;
			}
		}
		if let Ok(expires_in) = keytar::get_password(APP_ID, "msft_expires_in") {
			if (!expires_in.password.is_empty()) {
				model.token.expires_in = expires_in.password.parse().unwrap();
			}
		}
		if let Ok(refresh_token) =
			keytar::get_password(APP_ID, "msft_refresh_token")
		{
			if (!refresh_token.password.is_empty()) {
				model.token.refresh_token = refresh_token.password;
			}
		}
		model
	}

	fn store_token(&mut self, token: Token) -> Result<()> {
		self.token = token;
		keytar::set_password(
			"dev.edfloreshz.Done",
			"msft_expires_in",
			&self.token.expires_in.to_string(),
		);
		keytar::set_password(
			"dev.edfloreshz.Done",
			"msft_access_token",
			&self.token.access_token,
		);
		keytar::set_password(
			"dev.edfloreshz.Done",
			"msft_refresh_token",
			&self.token.refresh_token,
		);
		Ok(())
	}

	fn logout(&self) -> anyhow::Result<()> {
		keytar::delete_password("dev.edfloreshz.Done", "msft_expires_in")?;
		keytar::delete_password("dev.edfloreshz.Done", "msft_access_token")?;
		keytar::delete_password("dev.edfloreshz.Done", "msft_refresh_token")?;
		Ok(())
	}

	async fn token(&mut self) -> Result<()> {
		let client = reqwest::Client::new();
		let params = cascade! {
			HashMap::new();
			..insert("client_id", CLIENT_ID);
			..insert("scope", API_PERMISSIONS);
			..insert("code", self.code.as_str());
			..insert("redirect_uri", REDIRECT_URI);
			..insert("grant_type", "authorization_code");
		};
		let response = client
			.post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
			.form(&params)
			.send()
			.await?;
		match response.error_for_status() {
			Ok(response) => {
				let response = response.text().await?;
				let token: Token = serde_json::from_str(response.as_str())?;
				self.store_token(token)
			},
			Err(error) => Err(error.into()),
		}
	}

	async fn refresh_token(&mut self) -> anyhow::Result<()> {
		let client = reqwest::Client::new();
		let params = cascade! {
				HashMap::new();
				..insert("client_id", CLIENT_ID);
				..insert("scope", API_PERMISSIONS);
				..insert("refresh_token", &self.token.refresh_token);
				..insert("grant_type", "refresh_token");
		};
		let response = client
			.post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
			.form(&params)
			.send()
			.await?;
		match response.error_for_status() {
			Ok(response) => {
				let response = response.text().await?;
				let token: Token = serde_json::from_str(response.as_str())?;
				self.store_token(token)
			},
			Err(error) => Err(error.into()),
		}
	}
}

#[async_trait]
#[allow(unused)]
impl TaskService for Microsoft {
	async fn handle_uri_params(&mut self, uri: Url) -> Result<()> {
		let mut pairs = uri.query_pairs();
		if uri.as_str().contains("msft") {
			let code = pairs.next().unwrap().1.to_string();
			self.code = code;
			self.token().await;
		}
		Ok(())
	}

	fn login(&self) -> anyhow::Result<()> {
		let url = format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?
		client_id={CLIENT_ID}
		&response_type=code
		&redirect_uri={REDIRECT_URI}
		&response_mode=query
		&scope=offline_access%20user.read%20tasks.read%20tasks.read.shared%20tasks.readwrite%20tasks.readwrite.shared%20");
		open::that(url)?;
		Ok(())
	}

	fn available(&self) -> bool {
		let password = keytar::get_password(APP_ID, "msft_access_token");
		password.is_ok() && !password.unwrap().password.is_empty()
	}

	async fn read_tasks(&mut self) -> Result<Vec<Task>> {
		Ok(vec![])
	}

	async fn read_tasks_from_list(
		&mut self,
		parent_list: String,
	) -> Result<Vec<Task>> {
		self.refresh_token().await?;
		let client = reqwest::Client::new();
		let response = client
			.get(format!(
				"https://graph.microsoft.com/v1.0/me/todo/lists/{parent_list}/tasks",
			))
			.bearer_auth(&self.token.access_token)
			.send()
			.await?;
		match response.error_for_status() {
			Ok(response) => {
				let response = response.text().await?;
				let collection: Collection<ToDoTask> =
					serde_json::from_str(response.as_str())?;
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
			},
			Err(error) => Err(error.into()),
		}
	}

	async fn read_task(
		&mut self,
		task_list_id: String,
		task_id: String,
	) -> Result<Task> {
		self.refresh_token().await?;
		let client = reqwest::Client::new();
		let response = client
			.get(format!(
				"https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks/{}",
				task_list_id, task_id
			))
			.bearer_auth(&self.token.access_token)
			.send()
			.await?;
		match response.error_for_status() {
			Ok(response) => {
				let response = response.text().await?;
				let task: ToDoTask = serde_json::from_str(response.as_str())?;
				let mut task: Task = task.clone().into();
				task.parent = task_list_id;
				Ok(task)
			},
			Err(error) => Err(error.into()),
		}
	}

	async fn create_task(&mut self, task: Task) -> Result<()> {
		self.refresh_token().await?;
		let client = reqwest::Client::new();
		let todo_task: ToDoTask = task.clone().into();
		let data = serde_json::to_string(&todo_task).unwrap();
		let request = client
			.post(format!(
				"https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks",
				task.parent
			))
			.header("Content-Type", "application/json")
			.bearer_auth(&self.token.access_token)
			.body(data);
		let response = request.send().await?;
		match response.error_for_status() {
			Ok(response) => {
				if response.status() == StatusCode::CREATED {
					Ok(())
				} else {
					bail!("An error ocurred while creating the task.")
				}
			},
			Err(err) => Err(err.into()),
		}
	}

	async fn update_task(&mut self, task: Task) -> Result<Task> {
		self.refresh_token().await?;
		let client = reqwest::Client::new();
		let data = serde_json::to_string(&task).unwrap();
		let response = client
			.patch(format!(
				"https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks/{}",
				task.parent, task.id
			))
			.header("Content-Type", "application/json")
			.bearer_auth(&self.token.access_token)
			.body(data)
			.send()
			.await?;
		match response.error_for_status() {
			Ok(response) => {
				if response.status() == StatusCode::OK {
					Ok(task)
				} else {
					bail!("An error ocurred while updating the list.")
				}
			},
			Err(err) => Err(err.into()),
		}
	}

	async fn delete_task(&mut self, id: String) -> Result<()> {
		Ok(())
	}

	async fn read_lists(&mut self) -> Result<Vec<List>> {
		self.refresh_token().await?;
		let client = reqwest::Client::new();
		let response = client
			.get("https://graph.microsoft.com/v1.0/me/todo/lists")
			.bearer_auth(&self.token.access_token)
			.send()
			.await?;
		match response.error_for_status() {
			Ok(response) => {
				let lists = response.text().await?;
				let lists: Collection<ToDoTaskList> =
					serde_json::from_str(lists.as_str())?;
				Ok(lists.value.iter().map(|t| t.clone().into()).collect())
			},
			Err(err) => Err(err.into()),
		}
	}

	async fn read_list(&mut self, id: String) -> Result<List> {
		self.refresh_token().await?;
		let client = reqwest::Client::new();
		let response = client
			.get(format!(
				"https://graph.microsoft.com/v1.0/me/todo/lists/{id}"
			))
			.bearer_auth(&self.token.access_token)
			.send()
			.await?;
		match response.error_for_status() {
			Ok(response) => {
				let response = response.text().await?;
				let list: ToDoTaskList = serde_json::from_str(response.as_str())?;
				Ok(list.into())
			},
			Err(err) => Err(err.into()),
		}
	}

	async fn create_list(&mut self, list: List) -> Result<List> {
		self.refresh_token().await?;
		let client = reqwest::Client::new();
		let list: ToDoTaskList = list.into();
		let data = serde_json::to_string(&list).unwrap();
		let response = client
			.post("https://graph.microsoft.com/v1.0/me/todo/lists")
			.header("Content-Type", "application/json")
			.bearer_auth(&self.token.access_token)
			.body(data)
			.send()
			.await?;
		match response.error_for_status() {
			Ok(response) => {
				if response.status() == StatusCode::CREATED {
					Ok(list.into())
				} else {
					bail!("An error ocurred while creating the list.")
				}
			},
			Err(err) => Err(err.into()),
		}
	}

	async fn update_list(&mut self, list: List) -> Result<()> {
		self.refresh_token().await?;
		let client = reqwest::Client::new();
		let list: ToDoTaskList = list.into();
		let data = serde_json::to_string(&list).unwrap();
		let response = client
			.patch(format!(
				"https://graph.microsoft.com/v1.0/me/todo/lists/{}",
				list.id
			))
			.header("Content-Type", "application/json")
			.bearer_auth(&self.token.access_token)
			.body(data)
			.send()
			.await?;

		match response.error_for_status() {
			Ok(response) => {
				if response.status() == StatusCode::OK {
					Ok(())
				} else {
					bail!("An error ocurred while updating the list.")
				}
			},
			Err(err) => Err(err.into()),
		}
	}

	async fn delete_list(&mut self, id: String) -> Result<()> {
		self.refresh_token().await?;
		let client = reqwest::Client::new();
		let response = client
			.delete(format!(
				"https://graph.microsoft.com/v1.0/me/todo/lists/{id}",
			))
			.bearer_auth(&self.token.access_token)
			.send()
			.await?;
		match response.error_for_status() {
			Ok(response) => {
				if response.status() == StatusCode::NO_CONTENT {
					Ok(())
				} else {
					bail!("An error ocurred while deleting the list.")
				}
			},
			Err(err) => Err(err.into()),
		}
	}
}
