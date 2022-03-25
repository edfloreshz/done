use std::collections::HashMap;
use cascade::cascade;
use serde::{Serialize, Deserialize};
use crate::List;
use crate::models::task::Task;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Requester {
    pub expires_in: usize,
    pub access_token: String,
    pub refresh_token: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Collection<T> {
    pub value: Vec<T>
}

mod blocking {
    use cascade::cascade;
    use crate::{List, Requester};
    use crate::models::task::{Task, ToDoTask};
    use crate::token::Collection;
    use std::collections::HashMap;

    impl Requester {
        pub fn token_blocking(code: &str) -> anyhow::Result<Requester> {
            let client = reqwest::blocking::Client::new();
            let params = cascade! {
                HashMap::new();
                ..insert("client_id", "af13f4ae-b607-4a07-9ddc-6c5c5d59979f");
                ..insert("scope", "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared");
                ..insert("redirect_uri", "https://login.microsoftonline.com/common/oauth2/nativeclient");
                ..insert("grant_type", "authorization_code");
                ..insert("code", code);
            };
            let response = client.post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
                .form(&params)
                .send()?;
            if response.status().is_success() {
                let response = response.text()?;
                let auth: Requester = serde_json::from_str(response.as_str())?;
                println!("{:#?}", auth);
                Ok(auth)
            } else {
                // TODO: Let know the user the error.
                Ok(Requester::default())
            }
        }
        pub fn refresh_token_blocking(refresh_token: &str) -> anyhow::Result<Requester> {
            let client = reqwest::blocking::Client::new();
            let params = cascade! {
            HashMap::new();
            ..insert("client_id", "af13f4ae-b607-4a07-9ddc-6c5c5d59979f");
            ..insert("scope", "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared");
            ..insert("redirect_uri", "https://login.microsoftonline.com/common/oauth2/nativeclient");
            ..insert("grant_type", "refresh_token");
            ..insert("refresh_token", refresh_token);
        };
            let response = client.post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
                .form(&params)
                .send()?;
            if response.status().is_success() {
                let response = response.text()?;
                let auth: Requester = serde_json::from_str(response.as_str())?;
                println!("{:#?}", auth);
                Ok(auth)
            } else {
                // TODO: Let know the user the error.
                Ok(Requester::default())
            }
        }
        pub fn get_lists_blocking(&self) -> anyhow::Result<Vec<List>> {
            let client = reqwest::blocking::Client::new();
            let response = client.get("https://graph.microsoft.com/v1.0/me/todo/lists").bearer_auth(&self.access_token).send()?;
            if response.status().is_success() {
                let lists = response.text()?;
                let lists: Collection<List> = serde_json::from_str(lists.as_str())?;
                Ok(lists.value)
            } else {
                Ok(vec![])
            }
        }
        pub fn get_task_blocking(&self, task_id: &str) -> anyhow::Result<Vec<ToDoTask>> {
            let client = reqwest::blocking::Client::new();
            let response = client.get(format!("https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks", task_id)).bearer_auth(&self.access_token).send()?;
            if response.status().is_success() {
                let response = response.text()?;
                let collection: Collection<ToDoTask> = serde_json::from_str(response.as_str())?;
                Ok(collection.value)
            } else {
                Ok(vec![])
            }
        }
    }
}

impl Requester {
    pub async fn authorize(state: &str) -> anyhow::Result<String> {
        let response = reqwest::get(format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?
            client_id=af13f4ae-b607-4a07-9ddc-6c5c5d59979f
            &response_type=form_post
            &redirect_uri=https://login.microsoftonline.com/common/oauth2/nativeclient
            &response_mode=query
            &scope=offline_access%20user.read%20tasks.read%20tasks.read.shared%20tasks.readwrite%20tasks.readwrite.shared%20
            &state={}", state)
        ).await?.text().await?;
        println!("{:?}", response);
        Ok(String::new())
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
        let response = client.post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&params)
            .send()
            .await?;
        if response.status().is_success() {
            let response = response.text().await?;
            let auth: Requester = serde_json::from_str(response.as_str())?;
            println!("{:#?}", auth);
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
        let response = client.post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&params)
            .send()
            .await?;
        if response.status().is_success() {
            let response = response.text().await?;
            let auth: Requester = serde_json::from_str(response.as_str())?;
            println!("{:#?}", auth);
            Ok(auth)
        } else {
            // TODO: Let know the user the error.
            Ok(Requester::default())
        }
    }
    pub async fn get_lists(&self) -> anyhow::Result<Vec<List>> {
        let client = reqwest::Client::new();
        let response = client.get("https://graph.microsoft.com/v1.0/me/todo/lists").bearer_auth(&self.access_token).send().await?;
        if response.status().is_success() {
            let lists = response.text().await?;
            let lists: Collection<List> = serde_json::from_str(lists.as_str())?;
            Ok(lists.value)
        } else {
            Ok(vec![])
        }
    }
}