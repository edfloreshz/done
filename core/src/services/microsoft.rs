use crate::{
	models::{List, Task},
	task_service::TaskService,
};
use anyhow::Result;

use async_trait::async_trait;

use super::Service;

pub struct Microsoft {
	token: String,
}

impl Microsoft {
	pub(crate) fn new() -> Self {
		Self {
			token: String::new(),
		}
	}
}

#[async_trait]
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
		&self,
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
