use crate::services::ToDoService;
use crate::{List, Task};
use anyhow::Context;
use cascade::cascade;
use libdmd::config::Config;
use libdmd::element::Element;
use libdmd::format::{ElementFormat, FileType};
use libdmd::{dir, fi};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc;
use crate::services::microsoft::types::Collection;
use warp::Filter;
use warp::http::Uri;

#[derive(Deserialize)]
pub struct Query {
    pub code: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MicrosoftTokenAccess {
    pub expires_in: usize,
    pub access_token: String,
    pub refresh_token: String,
}

async fn receive_query() -> Query {
    let (sender, receiver) = mpsc::sync_channel(1);
    let route = warp::get()
        .and(warp::filters::query::query())
        .map(move |query: Query| {
            warp::redirect(Uri::from_static("do://open"));
            query
        })
        .map(move |query: Query| {
            sender.send(query).expect("failed to send query");
            "Go back to the app to continue."
        });

    tokio::task::spawn(warp::serve(route).run(([127, 0, 0, 1], 8000)));

    receiver.recv().expect("channel has hung up")
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

    async fn authenticate() -> anyhow::Result<String> {
        let url = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?
            client_id=af13f4ae-b607-4a07-9ddc-6c5c5d59979f
            &response_type=code
            &redirect_uri=http://127.0.0.1:8000
            &response_mode=query
            &scope=offline_access%20user.read%20tasks.read%20tasks.read.shared%20tasks.readwrite%20tasks.readwrite.shared%20
            &state=1234";
        open::that(url)?;
        let query = receive_query().await;
        Ok(query.code)
    }

    async fn token(code: String) -> anyhow::Result<MicrosoftTokenAccess> {
        let client = reqwest::Client::new();
        let params = cascade! {
            HashMap::new();
            ..insert("client_id", "af13f4ae-b607-4a07-9ddc-6c5c5d59979f");
            ..insert("scope", "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared");
            ..insert("redirect_uri", "http://127.0.0.1:8000");
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
                let token_data: MicrosoftTokenAccess = serde_json::from_str(response.as_str())?;
                MicrosoftTokenAccess::update_token_data(&token_data)?;
                Ok(token_data)
            }
            Err(error) => {
                Err(error.into())
            },
        }
    }

    async fn refresh_token(refresh_token: &str) -> anyhow::Result<MicrosoftTokenAccess> {
        let client = reqwest::Client::new();
        let params = cascade! {
            HashMap::new();
            ..insert("client_id", "af13f4ae-b607-4a07-9ddc-6c5c5d59979f");
            ..insert("scope", "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared");
            ..insert("redirect_uri", "http://127.0.0.1:8000");
            ..insert("grant_type", "refresh_token");
            ..insert("refresh_token", refresh_token);
        };
        let response = client
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&params)
            .send()
            .await?;
        match response.error_for_status() {
            Ok(response) => {
                let response = response.text().await?;
                let token_data: MicrosoftTokenAccess = serde_json::from_str(response.as_str())?;
                MicrosoftTokenAccess::update_token_data(&token_data)?;
                Ok(token_data)
            }
            Err(error) => {
                Err(error.into())
            },
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
        match response.error_for_status() {
            Ok(response) => {
                let lists = response.text().await?;
                let lists: Collection<List> = serde_json::from_str(lists.as_str())?;
                Ok(lists.value)
            }
            Err(error) => {
                Err(error.into())
            },
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
        match response.error_for_status() {
            Ok(response) => {
                let response = response.text().await?;
                let collection: Collection<Task> = serde_json::from_str(response.as_str())?;
                Ok(collection.value)
            }
            Err(error) => {
                Err(error.into())
            },
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
        match response.error_for_status() {
            Ok(response) => {
                let response = response.text().await?;
                let collection: Collection<Task> = serde_json::from_str(response.as_str())?;
                Ok(collection.value)
            }
            Err(error) => {
                Err(error.into())
            },
        }
    }

    async fn get_task(task_list_id: &str, task_id: &str) -> anyhow::Result<Task> {
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
                let task: Task = serde_json::from_str(response.as_str())?;
                Ok(task.into())
            }
            Err(error) => Err(error.into()),
        }
    }

    async fn push_task(task_list_id: &str, entry: String) -> anyhow::Result<()> {
        let config = MicrosoftTokenAccess::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
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
}