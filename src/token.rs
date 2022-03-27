use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Context;
use cascade::cascade;
use chrono::DateTime;
use libdmd::config::Config;
use libdmd::format::FileType;
use serde::{Deserialize, Serialize};

use crate::{List, Task, TaskImportance, TaskStatus};
use crate::models::task::ToDoTask;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Requester {
    pub expires_in: usize,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Collection<T> {
    pub value: Vec<T>,
}

impl Requester {
    pub fn is_token_present() -> bool {
        let config = Requester::current_config();
        match config {
            Some(config) => !config.refresh_token.is_empty(),
            None => false,
        }
    }
    pub fn current_config() -> Option<Self> {
        Config::get::<Requester>("ToDo/config/config.toml", FileType::TOML)
    }
    pub async fn token(code: &str) -> anyhow::Result<Requester> {
        let client = reqwest::Client::new();
        let params = cascade! {
            HashMap::new();
            ..insert("client_id", "af13f4ae-b607-4a07-9ddc-6c5c5d59979f");
            ..insert("scope", "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared");
            ..insert("redirect_uri", "https://login.microsoftonline.com/common/oauth2/nativeclient");
            ..insert("grant_type", "authorization_code");
            ..insert("code", code);
        };
        let response = client
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&params)
            .send()
            .await?;
        if response.status().is_success() {
            let response = response.text().await?;
            let auth: Requester = serde_json::from_str(response.as_str())?;
            Config::set("ToDo/config/config.toml", auth.clone(), FileType::TOML)?;
            Ok(auth)
        } else {
            // TODO: Let know the user the error.
            Ok(Requester::default())
        }
    }
    pub async fn refresh_token(refresh_token: &str) -> anyhow::Result<Requester> {
        let client = reqwest::Client::new();
        let params = cascade! {
            HashMap::new();
            ..insert("client_id", "af13f4ae-b607-4a07-9ddc-6c5c5d59979f");
            ..insert("scope", "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared");
            ..insert("redirect_uri", "https://login.microsoftonline.com/common/oauth2/nativeclient");
            ..insert("grant_type", "refresh_token");
            ..insert("refresh_token", refresh_token);
        };
        let response = client
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&params)
            .send()
            .await?;
        if response.status().is_success() {
            let response = response.text().await?;
            let auth: Requester = serde_json::from_str(response.as_str())?;
            Config::set("ToDo/config/config.toml", auth.clone(), FileType::TOML)?;
            Ok(auth)
        } else {
            // TODO: Let know the user the error.
            Ok(Requester::default())
        }
    }
    pub async fn get_lists() -> anyhow::Result<Vec<List>> {
        let config = Requester::current_config().with_context(|| "Failed to get current configuration.")?;
        let client = reqwest::Client::new();
        let response = client
            .get("https://graph.microsoft.com/v1.0/me/todo/lists")
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        if response.status().is_success() {
            let lists = response.text().await?;
            let lists: Collection<List> = serde_json::from_str(lists.as_str())?;
            Ok(lists.value)
        } else {
            Ok(vec![])
        }
    }
    pub async fn get_task(task_id: &str) -> anyhow::Result<Vec<Task>> {
        let config = Requester::current_config().with_context(|| "Failed to get current configuration.")?;
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks",
                task_id
            ))
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        if response.status().is_success() {
            let response = response.text().await?;
            let collection: Collection<ToDoTask> = serde_json::from_str(response.as_str())?;
            let collection = collection.value.iter()
                .map(|task| Task {
                    id: task.id.clone(),
                    importance: TaskImportance::from(task.importance.as_str()),
                    is_reminder_on: task.is_reminder_on,
                    status: TaskStatus::from(task.status.as_str()),
                    title: task.title.clone(),
                    created: DateTime::from_str(task.created.as_str()).unwrap(),
                    last_modified: DateTime::from_str(task.last_modified.as_str())
                        .unwrap(),
                    completed: false,
                })
                .collect();
            Ok(collection)
        } else {
            Ok(vec![])
        }
    }
}
