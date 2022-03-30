use crate::{List};
use anyhow::Result;
use async_trait::async_trait;
use libdmd::config::Config;
use crate::services::microsoft::task::Task;

pub mod microsoft;

#[async_trait]
pub trait ToDoService<T> {
    fn create_config(config: &mut Config) -> Result<Config>;
    fn is_token_present() -> bool;
    fn current_token_data() -> Option<T>;
    fn update_token_data(config: &T) -> Result<()>;
    async fn authenticate() -> Result<String>;
    async fn token(code: String) -> Result<T>;
    async fn refresh_token(refresh_token: &str) -> Result<T>;
    async fn get_lists() -> Result<Vec<List>>;
    async fn get_tasks(task_list_id: &str) -> Result<Vec<Task>>;
    async fn set_task_as_completed(
        task_list_id: &str,
        task_id: &str,
        completed: bool,
    ) -> Result<Vec<Task>>;
    async fn get_task(task_list_id: &str, task_id: &str) -> Result<Task>;
    async fn push_task(task_list_id: &str, entry: String) -> Result<()>;
}

