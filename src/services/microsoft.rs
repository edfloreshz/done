use crate::services::ToDoService;
use crate::{List, Task, TaskImportance, TaskStatus};
use anyhow::Context;
use cascade::cascade;
use chrono::DateTime;
use libdmd::config::Config;
use libdmd::element::Element;
use libdmd::format::{ElementFormat, FileType};
use libdmd::{dir, fi};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MicrosoftTokenAccess {
    pub expires_in: usize,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Collection<T> {
    pub value: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ToDoTask {
    pub id: String,
    pub importance: String,
    #[serde(rename = "isReminderOn")]
    pub is_reminder_on: bool,
    pub status: String,
    pub title: String,
    #[serde(rename = "createdDateTime")]
    pub created: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified: String,
}

#[async_trait::async_trait]
impl ToDoService<MicrosoftTokenAccess> for MicrosoftTokenAccess {
    fn create_config(config: &mut Config) -> anyhow::Result<Config> {
        config
            .add(dir!("services").child(dir!("microsoft").child(fi!("token.toml"))))
            .write()
    }

    fn is_token_present() -> bool {
        let config = MicrosoftTokenAccess::current_token_data();
        match config {
            Some(config) => !config.refresh_token.is_empty(),
            None => false,
        }
    }

    fn current_token_data() -> Option<MicrosoftTokenAccess> {
        Config::get::<MicrosoftTokenAccess>("ToDo/services/microsoft/token.toml", FileType::TOML)
    }

    fn update_token_data(config: &MicrosoftTokenAccess) -> anyhow::Result<()> {
        Config::set(
            "ToDo/services/microsoft/token.toml",
            config.clone(),
            FileType::TOML,
        )
    }

    async fn token(code: &str) -> anyhow::Result<MicrosoftTokenAccess> {
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
            let token_data: MicrosoftTokenAccess = serde_json::from_str(response.as_str())?;
            MicrosoftTokenAccess::update_token_data(&token_data)?;
            Ok(token_data)
        } else {
            // TODO: Let know the user the error.
            Ok(MicrosoftTokenAccess::default())
        }
    }

    async fn refresh_token(refresh_token: &str) -> anyhow::Result<MicrosoftTokenAccess> {
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
            let token_data: MicrosoftTokenAccess = serde_json::from_str(response.as_str())?;
            MicrosoftTokenAccess::update_token_data(&token_data)?;
            Ok(token_data)
        } else {
            // TODO: Let know the user the error.
            Ok(MicrosoftTokenAccess::default())
        }
    }

    async fn get_lists() -> anyhow::Result<Vec<List>> {
        let config = MicrosoftTokenAccess::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
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

    async fn get_tasks(task_list_id: &str) -> anyhow::Result<Vec<Task>> {
        let config = MicrosoftTokenAccess::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks",
                task_list_id
            ))
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        if response.status().is_success() {
            let response = response.text().await?;
            let collection: Collection<ToDoTask> = serde_json::from_str(response.as_str())?;
            let collection = collection.value.iter().map(|t| t.to_owned().into()).collect();
            Ok(collection)
        } else {
            Ok(vec![])
        }
    }

    async fn set_task_as_completed(
        task_list_id: &str,
        task_id: &str,
        completed: bool,
    ) -> anyhow::Result<Vec<Task>> {
        let config = MicrosoftTokenAccess::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let status = format!(
            "{}:{}",
            "{\"status\"",
            if completed {
                "\"notStarted\"}"
            } else {
                "\"completed\"}"
            }
        );
        let client = reqwest::Client::new();
        let response = client
            .patch(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks/{}",
                task_list_id, task_id
            ))
            .header("Content-Type", "application/json")
            .body(status)
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        if response.status().is_success() {
            let response = response.text().await?;
            let collection: Collection<ToDoTask> = serde_json::from_str(response.as_str())?;
            let collection = collection.value.iter().map(|t| t.to_owned().into()).collect();
            Ok(collection)
        } else {
            Ok(vec![])
        }
    }

    async fn get_task(task_list_id: &str, task_id: &str) -> anyhow::Result<Task> {
        // TODO: Response does not contain anything...
        let config = MicrosoftTokenAccess::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks/{}",
                task_list_id, task_id
            ))
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        match response.error_for_status() {
            Ok(response) => {
                let response = response.text().await?;
                let task: ToDoTask = serde_json::from_str(response.as_str())?;
                Ok(task.into())
            }
            Err(error) => Err(error.into())
        }
    }
}

impl From<ToDoTask> for Task {
    fn from(task: ToDoTask) -> Self {
        Task {
            id: task.id.clone(),
            importance: TaskImportance::from(task.importance.as_str()),
            is_reminder_on: task.is_reminder_on,
            status: TaskStatus::from(task.status.as_str()),
            title: task.title.clone(),
            created: DateTime::from_str(task.created.as_str()).unwrap(),
            last_modified: DateTime::from_str(task.last_modified.as_str()).unwrap(),
            completed: TaskStatus::is_completed(task.status.as_str()),
        }
    }
}
