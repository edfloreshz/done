use std::collections::HashMap;

use crate::models::list::List;
use crate::models::task::Task;
use crate::service::Service;
use crate::task_service::TaskService;
use anyhow::Result;
use async_trait::async_trait;
use cascade::cascade;
use msft_todo_types::{collection::Collection, token::Token};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Microsoft {
	token: Token,
}

#[allow(unused)]
impl Microsoft {
	pub fn new() -> Self {
		Self::default()
	}

	async fn login(&self) -> anyhow::Result<()> {
		let url = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?
            client_id=af13f4ae-b607-4a07-9ddc-6c5c5d59979f
            &response_type=code
            &redirect_uri=do://msft/
            &response_mode=query
            &scope=offline_access%20user.read%20tasks.read%20tasks.read.shared%20tasks.readwrite%20tasks.readwrite.shared%20
            &state=1234";
		open::that(url)?;
		Ok(())
	}

	fn store_token(&mut self, token: Token) -> Result<()> {
		self.token = token;
		Ok(())
	}

	async fn logout(&self) -> anyhow::Result<()> {
		Ok(())
	}

	async fn token(&mut self, code: String) -> Result<()> {
		let client = reqwest::Client::new();
		let params = cascade! {
			HashMap::new();
			..insert("client_id", "af13f4ae-b607-4a07-9ddc-6c5c5d59979f");
			..insert("scope", "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared");
			..insert("redirect_uri", "done://msft/");
			..insert("grant_type", "authorization_code");
			..insert("code", code.as_str());
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
				..insert("client_id", "af13f4ae-b607-4a07-9ddc-6c5c5d59979f");
				..insert("scope", "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared");
				..insert("redirect_uri", "do://msft/");
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

	async fn read_lists(&self) -> Result<Vec<List>> {
		Ok(vec![List::new("Testing Microsoft", Service::Microsoft)])
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
