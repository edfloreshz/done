use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use uuid::Uuid;

use crate::enums::{TaskImportance, TaskStatus};

use crate::schema::tasks;

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
	pub created_date_time: Option<NaiveDateTime>,
	pub last_modified_date_time: Option<NaiveDateTime>,
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
			created_date_time: None,
			last_modified_date_time: None,
		}
	}
}