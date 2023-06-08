use crate::{
	models::{list::List, task::Task},
	task_service::TaskService,
};
use anyhow::Result;
use async_trait::async_trait;
use url::form_urlencoded::Parse;

#[derive(Debug, Clone, Copy)]
pub struct Smart;

impl Smart {
	pub fn new() -> Self {
		Self
	}
}

#[async_trait]
#[allow(unused)]
impl TaskService for Smart {
	async fn handle_uri_params(&mut self, mut params: Parse<'_>) -> Result<()> {
		Ok(())
	}

	fn login(&self) -> anyhow::Result<()> {
		Ok(())
	}

	fn available(&self) -> bool {
		true
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
		Ok(vec![])
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
		Ok(vec![])
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
