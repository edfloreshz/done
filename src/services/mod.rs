use anyhow::Result;
use async_trait::async_trait;
use libdmd::config::Config;
use crate::{List, Task};

#[async_trait]
pub trait ToDoService<T> {
    fn create_config(config: &mut Config) -> Result<Config>;
    fn is_token_present() -> bool;
    fn current_token_data() -> Option<T>;
    fn update_token_data(config: &T) -> Result<()>;
    async fn token(code: &str) -> Result<T>;
    async fn refresh_token(refresh_token: &str) -> Result<T>;
    async fn get_lists() -> Result<Vec<List>>;
    async fn get_tasks(task_list_id: &str) -> Result<Vec<Task>>;
    async fn set_task_as_completed(task_list_id: &str, task_id: &str, completed: bool) -> Result<Vec<Task>>;
}

pub mod microsoft;
