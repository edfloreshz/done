use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::task::QueryableTask;

use super::{Priority, Status};

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
	pub recurrence: Option<String>,
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
			recurrence: None,
			created_date_time: now.naive_utc(),
			last_modified_date_time: now.naive_utc(),
		}
	}
}

impl From<QueryableTask> for Task {
	fn from(value: QueryableTask) -> Self {
		Task {
			id: value.id_task,
			parent: value.parent,
			title: value.title,
			favorite: value.favorite,
			today: value.today,
			notes: value.notes,
			status: value.status.into(),
			priority: value.priority.into(),
			sub_tasks: serde_json::from_str(&value.sub_tasks).unwrap(),
			tags: serde_json::from_str(&value.tags).unwrap(),
			completion_date: value.completion_date.map(|date| date.into()),
			deletion_date: value.deletion_date.map(|date| date.into()),
			due_date: value.due_date.map(|date| date.into()),
			reminder_date: value.reminder_date.map(|date| date.into()),
			recurrence: value.recurrence,
			created_date_time: value.created_date_time,
			last_modified_date_time: value.last_modified_date_time,
		}
	}
}
