use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{priority::Priority, recurrence::Recurrence, status::Status};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
	pub id: String,
	pub parent: String,
	pub title: String,
	pub favorite: bool,
	pub today: bool,
	pub status: Status,
	pub priority: Priority,
	pub sub_tasks: Vec<Task>,
	pub tags: Vec<String>,
	pub notes: Option<String>,
	pub completion_date: Option<NaiveDateTime>,
	pub deletion_date: Option<NaiveDateTime>,
	pub due_date: Option<NaiveDateTime>,
	pub reminder_date: Option<NaiveDateTime>,
	pub recurrence: Recurrence,
	pub created_date_time: NaiveDateTime,
	pub last_modified_date_time: NaiveDateTime,
}

impl Task {
	pub fn new(title: String, parent: String) -> Self {
		let now = Utc::now();
		Self {
			id: Uuid::new_v4().to_string(),
			parent,
			title,
			favorite: false,
			today: false,
			status: Status::NotStarted,
			priority: Priority::Low,
			sub_tasks: vec![],
			tags: vec![],
			notes: None,
			completion_date: None,
			deletion_date: None,
			due_date: None,
			reminder_date: None,
			recurrence: Default::default(),
			created_date_time: now.naive_utc(),
			last_modified_date_time: now.naive_utc(),
		}
	}
}
