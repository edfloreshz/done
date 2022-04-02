use std::collections::HashMap;

use anyhow::Context;
use cascade::cascade;
use libdmd::config::Config;
use libdmd::element::Element;
use libdmd::format::{ElementFormat, FileType};
use libdmd::{dir, fi};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::list::List;
use crate::services::microsoft::task::Task;
use crate::services::microsoft::types::Collection;
use crate::services::ToDoService;

#[derive(Deserialize)]
pub struct Query {
    pub code: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MicrosoftService {
    pub expires_in: usize,
    pub access_token: String,
    pub refresh_token: String,
}

#[async_trait::async_trait]
impl ToDoService<MicrosoftService> for MicrosoftService {
    fn create_config(config: &mut Config) -> anyhow::Result<Config> {
        config
            .add(dir!("services").child(dir!("microsoft").child(fi!("token.toml"))))
            .write()
    }

    fn is_token_present() -> bool {
        let config = MicrosoftService::current_token_data();
        match config {
            Some(config) => !config.refresh_token.is_empty(),
            None => false,
        }
    }

    fn current_token_data() -> Option<MicrosoftService> {
        Config::get::<MicrosoftService>("do/services/microsoft/token.toml", FileType::TOML)
    }

    fn update_token_data(config: &MicrosoftService) -> anyhow::Result<()> {
        Config::set(
            "do/services/microsoft/token.toml",
            config.clone(),
            FileType::TOML,
        )
    }

    async fn authenticate() -> anyhow::Result<()> {
        let url = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?
            client_id=af13f4ae-b607-4a07-9ddc-6c5c5d59979f
            &response_type=code
            &redirect_uri=do://msft/
            &response_mode=query
            &scope=offline_access%20user.read%20tasks.read%20tasks.read.shared%20tasks.readwrite%20tasks.readwrite.shared%20
            &state=1234";
        open::that(url)?;
        Ok(())
    }

    async fn logout() -> anyhow::Result<()> {
        let token_data = MicrosoftService::default();
        MicrosoftService::update_token_data(&token_data)
    }

    async fn token(code: String) -> anyhow::Result<MicrosoftService> {
        let client = reqwest::Client::new();
        let params = cascade! {
            HashMap::new();
            ..insert("client_id", "af13f4ae-b607-4a07-9ddc-6c5c5d59979f");
            ..insert("scope", "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared");
            ..insert("redirect_uri", "do://msft/");
            ..insert("grant_type", "authorization_code");
            ..insert("code", code.as_str());
        };
        let response = client
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&params)
            .send()
            .await?;
        match response.error_for_status() {
            Ok(response) => {
                let response = response.text().await?;
                let token_data: MicrosoftService = serde_json::from_str(response.as_str())?;
                // token_data.creation_date = DateTime::<Utc>::from(SystemTime::now()).to_rfc3339();
                MicrosoftService::update_token_data(&token_data)?;
                Ok(token_data)
            }
            Err(error) => Err(error.into()),
        }
    }

    async fn refresh_token(&mut self) -> anyhow::Result<MicrosoftService> {
        let client = reqwest::Client::new();
        let params = cascade! {
            HashMap::new();
            ..insert("client_id", "af13f4ae-b607-4a07-9ddc-6c5c5d59979f");
            ..insert("scope", "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared");
            ..insert("redirect_uri", "do://msft/");
            ..insert("grant_type", "refresh_token");
            ..insert("refresh_token", &self.refresh_token);
        };
        let response = client
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&params)
            .send()
            .await?;
        match response.error_for_status() {
            Ok(response) => {
                let response = response.text().await?;
                let mut token_data: MicrosoftService = serde_json::from_str(response.as_str())?;
                // token_data.creation_date = DateTime::<Utc>::from(SystemTime::now()).to_rfc3339();
                MicrosoftService::update_token_data(&token_data)?;
                self = &mut token_data;
                Ok(token_data)
            }
            Err(error) => Err(error.into()),
        }
    }

    async fn get_lists() -> anyhow::Result<Vec<List>> {
        let mut config = MicrosoftService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        config.refresh_token().await?;
        let client = reqwest::Client::new();
        let response = client
            .get("https://graph.microsoft.com/v1.0/me/todo/lists")
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        match response.error_for_status() {
            Ok(response) => {
                let lists = response.text().await?;
                let lists: Collection<List> = serde_json::from_str(lists.as_str())?;
                Ok(lists.value)
            }
            Err(err) => Err(err.into()),
        }
    }

    async fn delete_list(list_id: &str) -> anyhow::Result<()> {
        let mut config = MicrosoftService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        config.refresh_token().await?;
        let client = reqwest::Client::new();
        let response = client
            .delete(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}",
                list_id
            ))
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        if response.status() == StatusCode::NO_CONTENT {
            return Ok(());
        }
        if let Err(err) = response.error_for_status() {
            return Err(err.into());
        }
        Ok(())
    }

    async fn post_list(name: String) -> anyhow::Result<()> {
        let mut config = MicrosoftService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        config.refresh_token().await?;
        let client = reqwest::Client::new();
        let list = List {
            display_name: name,
            ..std::default::Default::default()
        };
        let data = serde_json::to_string(&list).unwrap();
        let response = client
            .post("https://graph.microsoft.com/v1.0/me/todo/lists")
            .header("Content-Type", "application/json")
            .bearer_auth(&config.access_token)
            .body(data)
            .send()
            .await?;
        if response.status() == StatusCode::CREATED {
            return Ok(());
        }
        if let Err(err) = response.error_for_status() {
            return Err(err.into());
        }
        Ok(())
    }

    async fn update_list(list_id: &str, name: String) -> anyhow::Result<()> {
        let mut config = MicrosoftService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        config.refresh_token().await?;
        let client = reqwest::Client::new();
        let list = List {
            display_name: name,
            ..std::default::Default::default()
        };
        let data = serde_json::to_string(&list).unwrap();
        let response = client
            .patch(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}",
                list_id
            ))
            .header("Content-Type", "application/json")
            .bearer_auth(&config.access_token)
            .body(data)
            .send()
            .await?;
        if response.status() == StatusCode::OK {
            return Ok(());
        }
        if let Err(err) = response.error_for_status() {
            return Err(err.into());
        }
        Ok(())
    }

    async fn get_tasks(task_list_id: &str) -> anyhow::Result<Vec<Task>> {
        let mut config = MicrosoftService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        config.refresh_token().await?;
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks",
                task_list_id
            ))
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        match response.error_for_status() {
            Ok(response) => {
                let response = response.text().await?;
                let collection: Collection<Task> = serde_json::from_str(response.as_str())?;
                Ok(collection.value)
            }
            Err(error) => Err(error.into()),
        }
    }

    async fn get_task(task_list_id: &str, task_id: &str) -> anyhow::Result<Task> {
        let mut config = MicrosoftService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        config.refresh_token().await?;
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
                let task: Task = serde_json::from_str(response.as_str())?;
                Ok(task)
            }
            Err(error) => Err(error.into()),
        }
    }

    async fn delete_task(list_id: &str, task_id: &str) -> anyhow::Result<()> {
        let mut config = MicrosoftService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        config.refresh_token().await?;
        let client = reqwest::Client::new();
        let request = client
            .delete(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks/{}",
                list_id, task_id
            ))
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        if request.status() == StatusCode::NO_CONTENT {
            return Ok(());
        }
        if let Err(err) = request.error_for_status() {
            return Err(err.into());
        }
        Ok(())
    }

    async fn post_task(task_list_id: &str, entry: String) -> anyhow::Result<()> {
        let mut config = MicrosoftService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        config.refresh_token().await?;
        let client = reqwest::Client::new();
        let task = Task {
            title: entry,
            ..std::default::Default::default()
        };
        let data = serde_json::to_string(&task).unwrap();
        let request = client
            .post(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks",
                task_list_id
            ))
            .header("Content-Type", "application/json")
            .bearer_auth(&config.access_token)
            .body(data);
        let response = request.send().await?;
        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    async fn update_task(list_id: &str, task_id: &str, task: Task) -> anyhow::Result<()> {
        let mut config = MicrosoftService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        config.refresh_token().await?;
        let client = reqwest::Client::new();
        let data = serde_json::to_string(&task).unwrap();
        let response = client
            .patch(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks/{}",
                list_id, task_id
            ))
            .header("Content-Type", "application/json")
            .bearer_auth(&config.access_token)
            .body(data)
            .send()
            .await?;
        if response.status() == StatusCode::OK {
            return Ok(());
        }
        if let Err(err) = response.error_for_status() {
            return Err(err.into());
        }
        Ok(())
    }

    async fn complete_task(
        task_list_id: &str,
        task_id: &str,
        completed: bool,
    ) -> anyhow::Result<Vec<Task>> {
        let mut config = MicrosoftService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        config.refresh_token().await?;
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
        match response.error_for_status() {
            Ok(response) => {
                let response = response.text().await?;
                let collection: Collection<Task> = serde_json::from_str(response.as_str())?;
                Ok(collection.value)
            }
            Err(error) => Err(error.into()),
        }
    }
}
