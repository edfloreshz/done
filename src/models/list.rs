use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub enum ListMsg {
    Delete(usize),
    Create(String),
    Select(usize),
    Rename(usize, String),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct List {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "isOwner")]
    pub is_owner: bool,
    #[serde(rename = "isShared")]
    pub is_shared: bool,
    #[serde(rename = "id")]
    pub task_list_id: String,
}
