use std::fmt::Debug;

use anyhow::Result;
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};

use crate::data::models::generic::lists::GenericList;
use crate::data::models::generic::tasks::GenericTask;
use crate::gtk;

pub trait TaskProvider: Debug {
	/// The unique identifier of the `TaskProvider`.
	fn get_id(&self) -> &str;
	/// The user-visible name of the `TaskProvider`.
	fn get_name(&self) -> &str;
	/// The type of the `TaskProvider`.
	fn get_provider_type(&self) -> ProviderType;
	/// The description of the `TaskProvider`, e.g. the account user of a GNOME Online Accounts' account
	fn get_description(&self) -> &str;
	/// Whether the `TaskProvider` is enabled.
	fn get_enabled(&self) -> bool;
	/// Sets the provider as enabled.
	fn set_enabled(&mut self);
	/// Asks the provider to refresh. Online providers may want to
	/// synchronize tasks and task lists, credentials, etc, when this
	/// is called.
	fn refresh(&self);
	fn get_icon_name(&self) -> String;
	fn get_icon(&self) -> gtk::Image;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderType {
	Inbox,
	Today,
	Next7Days,
	All,
	Local,
}

pub trait ProviderService: Debug {
	fn init() -> Self
	where
		Self: Sized;
	fn establish_connection(&self) -> Result<SqliteConnection>;
	// Fetch tasks from the provider and save them to local storage.
	fn refresh_tasks(&mut self) -> Result<()>;
	fn refresh_lists(&mut self) -> Result<()>;

	fn get_provider(&self) -> Box<dyn TaskProvider>;
	fn get_tasks(&self) -> Vec<GenericTask>;
	fn get_task_lists(&self) -> Vec<GenericList>;

	// Tasks
	/// This method should return the list of tasks in a list.
	fn read_tasks_from_list(&self, id: &str) -> Result<Vec<GenericTask>>;
	/// This method should return the information about a task.
	fn read_task(&self, id: &str) -> Result<GenericTask>;
	/// This method should create a new task and insert it to its respective list.
	fn create_task(
		&self,
		list: GenericList,
		task: GenericTask,
	) -> Result<GenericTask>;
	/// This method should update an existing task.
	fn update_task(&self, task: GenericTask) -> Result<()>;
	/// This method should remove an existing task.
	fn remove_task(&self, task_id: &str) -> Result<()>;

	// Task Lists
	/// This method should return the lists from a provider.
	fn read_task_lists(&self) -> Result<Vec<GenericList>>;
	/// This method should create a new list for a provider.
	fn create_task_list(&self, list: GenericList) -> Result<GenericList>;
	/// This method should update an existing list for a provider.
	fn update_task_list(&self, list: GenericList) -> Result<()>;
	/// This method should remove a list from a provider.
	fn remove_task_list(&self, list: GenericList) -> Result<()>;
}
