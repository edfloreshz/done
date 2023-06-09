use anyhow::Result;
use async_trait::async_trait;
use url::Url;

use crate::models::{list::List, task::Task};

#[async_trait]
pub trait TaskService: Sync + Send {
	/// Sets the initial config for this service.
	async fn handle_uri_params(&mut self, uri: Url) -> Result<()>;

	/// Checks to see if the service is available.
	fn login(&self) -> Result<()>;

	/// Checks to see if the service is available.
	fn available(&self) -> bool;

	/// Read all the tasks from a service, regardless of parent list.
	async fn read_tasks(&mut self) -> Result<Vec<Task>>;

	/// Read all the tasks from a list.
	async fn read_tasks_from_list(
		&mut self,
		parent_list: String,
	) -> Result<Vec<Task>>;

	/// Reads a single task by its id.
	async fn read_task(
		&mut self,
		task_list_id: String,
		task_id: String,
	) -> Result<Task>;

	/// Creates a single task.
	async fn create_task(&mut self, task: Task) -> Result<()>;

	/// Updates a single task.
	async fn update_task(&mut self, task: Task) -> Result<Task>;

	/// Deltes a single task.
	async fn delete_task(
		&mut self,
		list_id: String,
		task_id: String,
	) -> Result<()>;

	/// Read all the lists from a service.
	async fn read_lists(&mut self) -> Result<Vec<List>>;

	/// Read a single list from a service.
	async fn read_list(&mut self, id: String) -> Result<List>;

	/// Creates a single task list.
	async fn create_list(&mut self, list: List) -> Result<List>;

	/// Updates a single task list.
	async fn update_list(&mut self, list: List) -> Result<()>;

	/// Deletes a single task list.
	async fn delete_list(&mut self, id: String) -> Result<()>;
}
