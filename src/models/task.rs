use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub enum TaskMsg {
    SetCompleted((usize, bool)),
    AddEntry(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: String,
    pub importance: TaskImportance,
    #[serde(rename = "isReminderOn")]
    pub is_reminder_on: bool,
    pub status: TaskStatus,
    pub title: String,
    pub created: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub completed: bool,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskImportance {
    Normal,
}

impl Default for TaskImportance {
    fn default() -> Self {
        TaskImportance::Normal
    }
}

impl TaskImportance {
    pub fn from(importance: &str) -> Self {
        match importance {
            "normal" => TaskImportance::Normal,
            _ => TaskImportance::Normal,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    NotStarted,
    Started,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::NotStarted
    }
}

impl TaskStatus {
    pub fn from(status: &str) -> Self {
        match status {
            "notStarted" => TaskStatus::NotStarted,
            "started" => TaskStatus::Started,
            _ => TaskStatus::NotStarted,
        }
    }
}