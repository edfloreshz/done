use anyhow::Result;
use relm4::gtk::gio::ffi::GIcon;
use crate::models::list::List;

pub trait ToDoProvider {
    // Information

    /// The unique identifier of the `ToDoProvider`.
    fn get_id(&self) -> &str;
    /// The user-visible name of the `ToDoProvider`.
    fn get_name(&self) -> &str;
    /// The type of the `ToDoProvider`.
    fn get_provider_type(&self) -> ProviderType;
    /// The description of the `ToDoProvider`, e.g. the account user of a GNOME Online Accounts' account
    fn get_description(&self) -> &str;
    /// Whether the `ToDoProvider` is enabled.
    fn get_enabled(&self) -> bool;
    /// Asks the provider to refresh. Online providers may want to
    /// synchronize tasks and task lists, credentials, etc, when this
    /// is called.
    ///
    /// This is an optional feature. Providers that do not implement
    /// the "refresh" vfunc will be ignored.
    fn refresh(&self);

    // Customs

    /// The icon of the `ToDoProvider`, e.g. the account icon of a GNOME Online Accounts' account.
    fn get_icon(&self) -> GIcon;

    // Tasks

    /// Creates the given task in `self`.
    // fn create_task(&self, list: List, task: Task) -> Result<Task>;
    // /// Updates the given task in `self`.
    // fn update_task(&self, task: Task) -> Result<()>;
    // /// Removes the given task in `self`.
    // fn remove_task(&self, task: Task) -> Result<()>;

    // Task Lists

    /// Updates the given task list in `self`.
    fn create_task_list(&self, list: List) -> Result<List>;
    /// Updates the given task list in `self`.
    fn update_task_list(&self, task: List) -> Result<()>;
    /// Removes the given task list in `self`.
    fn remove_task_list(&self, task: List) -> Result<()>;
}

pub enum ProviderType {
    AllTasks,
    Inbox,
    NextWeek,
    Scheduled,
    Unscheduled,
    Today,
    Local,
    Todoist,
    Essential,
    Welcome,
    Sidebar,
}
