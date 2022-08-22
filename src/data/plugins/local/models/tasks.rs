use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::models::generic::task_importance::TaskImportance;
use crate::data::models::generic::task_status::TaskStatus;
use crate::data::models::generic::tasks::GenericTask;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocalTask {
	pub id_task: String,
	pub id_list: String,
	pub title: String,
	pub body: Option<String>,
	pub importance: TaskImportance,
	pub favorite: bool,
	pub is_reminder_on: bool,
	pub status: TaskStatus,
	pub completed_on: Option<NaiveDateTime>,
	pub due_date: Option<NaiveDateTime>,
	pub reminder_date: Option<NaiveDateTime>,
	pub created_date_time: Option<NaiveDateTime>,
	pub last_modified_date_time: Option<NaiveDateTime>,
}

impl LocalTask {
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

impl From<GenericTask> for LocalTask {
	fn from(task: GenericTask) -> Self {
		Self {
			id_task: task.id_task,
			id_list: task.id_list,
			title: task.title,
			body: task.body,
			completed_on: task.completed_on,
			due_date: task.due_date,
			importance: task.importance,
			favorite: task.favorite,
			is_reminder_on: task.is_reminder_on,
			reminder_date: task.reminder_date,
			status: task.status,
			created_date_time: task.created_date_time,
			last_modified_date_time: task.last_modified_date_time,
		}
	}
}
