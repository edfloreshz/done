use crate::{
    data::establish_connection,
    models::QueryableTask,
    traits::{List, Task},
};
use anyhow::Result;
use diesel::*;
use std::fmt::Debug;

pub trait Provider: Debug {
    /// Getters
    ///
    /// The unique identifier of the provider.
    fn get_id(&self) -> &str;
    /// The user-visible name of the provider.
    fn get_name(&self) -> &str;
    /// The description of the provider, e.g. the account user of a GNOME Online Accounts' account
    fn get_description(&self) -> &str;
    /// Whether the provider is enabled.
    fn is_enabled(&self) -> bool;
    /// Whether the provider is enabled.
    fn is_smart(&self) -> bool;
    /// Gets the icon name of the provider.
    fn get_icon_name(&self) -> &str;
    /// Gets the icon of the provider.
    fn get_icon(&self) -> &str;

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
    fn read_tasks_from_list(&self, id: &str) -> Result<Vec<Box<dyn Task>>>;
    // TODO
    //        use crate::schema::lists::id_list;
    //        use crate::schema::tasks::dsl::*;
    //
    //        let results = tasks
    //            .filter(id_list.eq(id))
    //            .load::<QueryableTask>(&mut establish_connection()?)?;
    //        let results: Vec<Box<dyn Task>> =
    //        results.iter().map(|task| task.to_owned()).collect();
    //
    //        Ok(results)
    /// This method should return the information about a task.
    fn read_task(&self, id: &str) -> Result<Box<dyn Task>>;
    /// This method should create a new task and insert it to its respective list.
    fn create_task<'a>(&self, task: impl Task + 'a) -> Result<Box<dyn Task + 'a>> {
        use crate::schema::tasks::dsl::*;

        let new_task = QueryableTask::from(&task);

        diesel::insert_into(tasks)
            .values(&new_task)
            .execute(&mut establish_connection()?)?;

        Ok(Box::new(task))
    }
    /// This method should update an existing task.
    fn update_task(&self, task: impl Task) -> Result<()>;
    /// This method should remove an existing task.
    fn remove_task(&self, task_id: &str) -> Result<()>;

    /// Task Lists
    ///
    /// This method should return the lists from a provider.
    fn read_task_lists(&self) -> Result<Vec<Box<dyn List>>>;
    /// This method should create a new list for a provider.
    fn create_task_list(
        &self,
        list_provider: &str,
        name: &str,
        icon: &str,
    ) -> Result<Box<dyn List>>;
    /// This method should update an existing list for a provider.
    fn update_task_list(&self, list: impl List) -> Result<()>;
    /// This method should remove a list from a provider.
    fn remove_task_list(&self, list: impl List) -> Result<()>;
}
