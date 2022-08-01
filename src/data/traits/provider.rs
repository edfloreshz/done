use std::fmt::Debug;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::data::models::generic::lists::GenericList;
use crate::data::models::generic::tasks::GenericTask;
use crate::gtk;

pub trait Provider: Debug {
	/// Getters
	///
	/// The unique identifier of the provider.
	fn get_id(&self) -> &str;
	/// The user-visible name of the provider.
	fn get_name(&self) -> &str;
	/// The type of the provider.
	fn get_provider_type(&self) -> ProviderType;
	/// The description of the provider, e.g. the account user of a GNOME Online Accounts' account
	fn get_description(&self) -> &str;
	/// Whether the provider is enabled.
	fn is_enabled(&self) -> bool;
	/// Gets the icon name of the provider.
	fn get_icon_name(&self) -> &str;
	/// Gets the icon of the provider.
	fn get_icon(&self) -> gtk::Image;

	/// # Setters
	///
	/// Sets the provider as enabled.
	fn set_enabled(&mut self);
	/// Sets the provider as disabled.
	fn set_disabled(&mut self);

	/// Methods
	///
	/// Creates a new instance of the provider.
	fn new() -> Self
	where
		Self: Sized;
	/// Asks the provider to refresh. Online providers may want to
	/// synchronize tasks and task lists, credentials, etc, when this
	/// is called.
	fn refresh(&self) -> Result<()>;

	/// Tasks
	///
	/// This method should return the list of tasks in a list.
	fn read_tasks_from_list(&self, id: &str) -> Result<Vec<GenericTask>>;
	/// This method should return the information about a task.
	fn read_task(&self, id: &str) -> Result<GenericTask>;
	/// This method should create a new task and insert it to its respective list.
	fn create_task(
		&mut self,
		list: GenericList,
		task: GenericTask,
	) -> Result<GenericTask>;
	/// This method should update an existing task.
	fn update_task(&mut self, task: GenericTask) -> Result<()>;
	/// This method should remove an existing task.
	fn remove_task(&mut self, task_id: &str) -> Result<()>;

	/// Task Lists
	///
	/// This method should return the lists from a provider.
	fn read_task_lists(&self) -> Result<Vec<GenericList>>;
	/// This method should create a new list for a provider.
	fn create_task_list(
		&mut self,
		list_provider: &str,
		name: &str,
		icon: &str,
	) -> Result<GenericList>;
	/// This method should update an existing list for a provider.
	fn update_task_list(&mut self, list: GenericList, name: &str) -> Result<()>;
	/// This method should remove a list from a provider.
	fn remove_task_list(&mut self, list: GenericList) -> Result<()>;
}

#[derive(Debug, Copy, Serialize, Deserialize, Clone)]
pub enum ProviderType {
	Inbox,
	Today,
	Next7Days,
	All,
	Local,
}
