use std::collections::HashMap;

use crate::models::list::List;
use crate::models::task::Task;
use crate::task_service::TaskService;
use anyhow::Result;
use async_trait::async_trait;
use cascade::cascade;
use msft_todo_types::{
	collection::Collection, list::ToDoTaskList, token::Token,
};
use serde::{Deserialize, Serialize};
use url::form_urlencoded::Parse;

const APP_ID: &str = "dev.edfloreshz.Done";
const CLIENT_ID: &str = "d90593cb-c2b1-4c87-b4f9-da24e1c03203";
const REDIRECT_URI: &str = "done://auth";
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
			model.token.access_token = access_token.password;
		}
		if let Ok(expires_in) = keytar::get_password(APP_ID, "msft_expires_in") {
			model.token.expires_in = expires_in.password.parse().unwrap();
		}
		if let Ok(refresh_token) =
			keytar::get_password(APP_ID, "msft_refresh_token")
		{
			model.token.refresh_token = refresh_token.password;
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

	async fn logout(&self) -> anyhow::Result<()> {
		Ok(())
	}

	async fn token(&mut self) -> Result<()> {
		let client = reqwest::Client::new();
		let params = cascade! {
			HashMap::new();
			..insert("client_id", CLIENT_ID);
			..insert("scope", API_PERMISSIONS);
			..insert("redirect_uri", REDIRECT_URI);
			..insert("grant_type", "authorization_code");
			..insert("code", self.code.as_str());
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
				..insert("redirect_uri", REDIRECT_URI);
				..insert("grant_type", "refresh_token");
				..insert("refresh_token", &self.token.refresh_token);
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
	async fn handle_uri_params(&mut self, mut params: Parse<'_>) -> Result<()> {
		let code = params.next().unwrap().1.to_string();
		self.code = code;
		self.token().await
	}

	fn login(&self) -> anyhow::Result<()> {
		let url = format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?
		client_id={CLIENT_ID}
		&response_type=code
		&redirect_uri={REDIRECT_URI}
		&response_mode=query
		&scope=offline_access%20user.read%20tasks.read%20tasks.read.shared%20tasks.readwrite%20tasks.readwrite.shared%20
		&state=1234");
		open::that(url)?;
		Ok(())
	}

	fn available(&self) -> Result<()> {
		Ok(())
	}

	async fn enable(&self) -> Result<()> {
		Ok(())
	}

	async fn disable(&self) -> Result<()> {
		Ok(())
	}

	async fn read_tasks(&self) -> Result<Vec<Task>> {
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
				let collection: Collection<Task> =
					serde_json::from_str(response.as_str())?;
				Ok(collection.value)
			},
			Err(error) => Err(error.into()),
		}
	}

	async fn read_task(&self, id: String) -> Result<Task> {
		Ok(Task::default())
	}

	async fn create_task(&self, task: Task) -> Result<()> {
		Ok(())
	}

	async fn update_task(&self, task: Task) -> Result<Task> {
		Ok(Task::default())
	}

	async fn delete_task(&self, id: String) -> Result<()> {
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

	async fn read_list(&self, id: String) -> Result<List> {
		Ok(List::default())
	}

	async fn create_list(&self, list: List) -> Result<List> {
		Ok(List::default())
	}

	async fn update_list(&self, list: List) -> Result<()> {
		Ok(())
	}

	async fn delete_list(&self, id: String) -> Result<()> {
		Ok(())
	}
}
