use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::core::models::generic::task_importance::TaskImportance;
use crate::core::models::generic::task_status::TaskStatus;
use crate::core::models::queryable::task::QueryableTask;
use crate::core::plugins::local::models::tasks::LocalTask;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GenericTask {
    pub id_task: String,
    pub id_list: String,
    pub title: String,
    pub body: Option<String>,
    pub completed_on: Option<String>,
    pub due_date: Option<String>,
    pub importance: TaskImportance,
    pub favorite: bool,
    pub is_reminder_on: bool,
    pub reminder_date: Option<String>,
    pub status: TaskStatus,
    pub created_date_time: Option<String>,
    pub last_modified_date_time: Option<String>,
}

impl GenericTask {
    pub fn new(title: String, list_id: String) -> Self {
        Self {
            id_task: Uuid::new_v4().to_string(),
            id_list: list_id,
            title,
            body: None,
            completed_on: None,
            due_date: None,
            importance: TaskImportance::default(),
            favorite: false,
            is_reminder_on: false,
            reminder_date: None,
            status: Default::default(),
            created_date_time: None,
            last_modified_date_time: None,
        }
    }
}


impl From<QueryableTask> for GenericTask {
    fn from(task: QueryableTask) -> Self {
        Self {
            id_task: task.id_task,
            id_list: task.id_list,
            title: task.title,
            body: task.body,
            completed_on: task.completed_on,
            due_date: task.due_date,
            importance: TaskImportance::from_str(task.importance.unwrap().as_str())
                .unwrap_or_default(),
            favorite: task.favorite,
            is_reminder_on: task.is_reminder_on,
            reminder_date: task.reminder_date,
            status: TaskStatus::from_str(task.status.unwrap().as_str())
                .unwrap_or_default(),
            created_date_time: task.created_date_time,
            last_modified_date_time: task.last_modified_date_time,
        }
    }
}

impl From<LocalTask> for GenericTask {
    fn from(local_task: LocalTask) -> Self {
        Self {
            id_task: local_task.id_task,
            id_list: local_task.id_list,
            title: local_task.title,
            body: local_task.body,
            completed_on: local_task.completed_on,
            due_date: local_task.due_date,
            importance: local_task.importance,
            favorite: local_task.favorite,
            is_reminder_on: local_task.is_reminder_on,
            reminder_date: local_task.reminder_date,
            status: local_task.status,
            created_date_time: local_task.created_date_time,
            last_modified_date_time: local_task.last_modified_date_time,
        }
    }
}