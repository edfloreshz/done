use serde::{Serialize, Deserialize};

pub enum ListMsg {
    Delete(usize),
    Create(String),
    Select(usize),
    Rename(usize, String),
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
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