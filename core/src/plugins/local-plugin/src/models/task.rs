use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable};
use uuid::Uuid;

use done_core::provider::{Task, TaskImportance, TaskStatus};

use crate::schemas::tasks;

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = tasks)]
pub struct QueryableTask {
	pub id_task: String,
	pub parent_list: String,
	pub title: String,
	pub body: Option<String>,
	pub importance: i32,
	pub favorite: bool,
	pub is_reminder_on: bool,
	pub status: i32,
	pub completed_on: Option<NaiveDateTime>,
	pub due_date: Option<NaiveDateTime>,
	pub reminder_date: Option<NaiveDateTime>,
	pub created_date_time: NaiveDateTime,
	pub last_modified_date_time: NaiveDateTime,
}

impl QueryableTask {
	pub fn new(title: String, parent_list: String) -> Self {
		Self {
			id_task: Uuid::new_v4().to_string(),
			parent_list,
			title,
			body: None,
			completed_on: None,
			due_date: None,
			importance: TaskImportance::Low as i32,
			favorite: false,
			is_reminder_on: false,
			reminder_date: None,
			status: TaskStatus::NotStarted as i32,
			created_date_time: Utc::now().naive_utc(),
			last_modified_date_time: Utc::now().naive_utc(),
		}
	}
}

impl Into<Task> for QueryableTask {
	fn into(self) -> Task {
		Task {
			id: self.id_task,
			parent: self.parent_list,
			title: self.title,
			body: self.body,
			importance: self.importance,
			favorite: self.favorite,
			is_reminder_on: self.is_reminder_on,
			status: self.status,
			completed_on: self.completed_on.map(|d| d.timestamp()),
			due_date: self.due_date.map(|d| d.timestamp()),
			reminder_date: self.reminder_date.map(|d| d.timestamp()),
			created_date_time: self.created_date_time.timestamp(),
			last_modified_date_time: self.last_modified_date_time.timestamp(),
		}
	}
}

impl From<Task> for QueryableTask {
	fn from(task: Task) -> Self {
		Self {
			id_task: task.id,
			parent_list: task.parent,
			title: task.title,
			body: task.body,
			importance: task.importance,
			favorite: task.favorite,
			is_reminder_on: task.is_reminder_on,
			status: task.status,
			completed_on: task.completed_on.map(|d| NaiveDateTime::from_timestamp(d, 0)),
			due_date: task.due_date.map(|d| NaiveDateTime::from_timestamp(d, 0)),
			reminder_date: task.reminder_date.map(|d| NaiveDateTime::from_timestamp(d, 0)),
			created_date_time: NaiveDateTime::from_timestamp(task.created_date_time, 0),
			last_modified_date_time: NaiveDateTime::from_timestamp(task.last_modified_date_time, 0),
		}
	}
}
