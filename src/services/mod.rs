use anyhow::Result;
use async_trait::async_trait;
use libdmd::config::Config;

use crate::models::list::List;
use crate::services::microsoft::task::Task;

pub mod microsoft;

#[async_trait]
pub trait ToDoService<T> {
    // Settings
    fn create_config(config: &mut Config) -> Result<Config>;
    // Token management
    fn is_token_present() -> bool;
    fn current_token_data() -> Option<T>;
    fn update_token_data(config: &T) -> Result<()>;
    // Authentication
    async fn authenticate() -> Result<()>;
    async fn logout() -> Result<()>;
    async fn token(code: String) -> Result<T>;
    async fn refresh_token(&mut self) -> Result<T>;
    // Lists
    async fn get_lists() -> Result<Vec<List>>;
    async fn delete_list(list_id: &str) -> Result<()>;
    async fn post_list(name: String) -> Result<()>;
    async fn update_list(list_id: &str, name: String) -> Result<()>;
    // List groups
    // async fn get_list_groups() -> Result<Vec<List>>;
    // async fn delete_list_groups(list_group_id: &str) -> Result<()>;
    // async fn post_list_groups(list_group_id: &str, group: ListGroup) -> Result<()>;
    // async fn update_list_groups(list_group_id: &str, group: ListGroup) -> Result<()>;
    // Tasks
    async fn get_tasks(list_id: &str) -> Result<Vec<Task>>;
    async fn get_task(list_id: &str, task_id: &str) -> Result<Task>;
    async fn delete_task(list_id: &str, task_id: &str) -> Result<()>;
    async fn post_task(list_id: &str, entry: String) -> Result<()>;
    async fn update_task(list_id: &str, task_id: &str, task: Task) -> Result<()>;
    async fn complete_task(list_id: &str, task_id: &str, completed: bool) -> Result<Vec<Task>>;
}
