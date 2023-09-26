use std::pin::Pin;

use anyhow::Result;
use async_trait::async_trait;
use futures::Stream;

use crate::models::{list::List, task::Task};

#[async_trait]
pub trait TodoService: Sync + Send {
	/// Gets the task count of a task list.
	async fn task_count(&mut self, task_list_id: String) -> Result<usize>;

	/// Read all the tasks from a service, regardless of parent list.
	async fn read_tasks(&mut self) -> Result<Vec<Task>>;

	/// Returns a stream of tasks from a list and sends it through a channel.
	async fn get_tasks(
		&mut self,
		parent_list: String,
	) -> Result<Pin<Box<dyn Stream<Item = Task> + Send>>>;

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

	/// Returns a stream of lists and sends it through a channel.
	async fn get_lists(
		&mut self,
	) -> Result<Pin<Box<dyn Stream<Item = List> + Send>>>;

	/// Read a single list from a service.
	async fn read_list(&mut self, id: String) -> Result<List>;

	/// Creates a single task list.
	async fn create_list(&mut self, list: List) -> Result<List>;

	/// Updates a single task list.
	async fn update_list(&mut self, list: List) -> Result<()>;

	/// Deletes a single task list.
	async fn delete_list(&mut self, id: String) -> Result<()>;
}
