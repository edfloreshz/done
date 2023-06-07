use chrono::{DateTime, Utc};
use msft_todo_types::{checklist_item::ChecklistItem, task::ToDoTask};
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
	pub completion_date: Option<DateTime<Utc>>,
	pub deletion_date: Option<DateTime<Utc>>,
	pub due_date: Option<DateTime<Utc>>,
	pub reminder_date: Option<DateTime<Utc>>,
	pub recurrence: Recurrence,
	pub created_date_time: DateTime<Utc>,
	pub last_modified_date_time: DateTime<Utc>,
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
			created_date_time: now,
			last_modified_date_time: now,
		}
	}
}

impl From<ToDoTask> for Task {
	fn from(task: ToDoTask) -> Self {
		Self {
			id: task.id,
			parent: String::new(),
			title: task.title,
			favorite: false,
			today: task.reminder_date_time.is_some()
				&& task.reminder_date_time.unwrap() == Utc::now(),
			status: task.status.into(),
			priority: task.importance.into(),
			sub_tasks: task
				.checklist_items
				.iter()
				.map(|item| item.clone().into())
				.collect(),
			tags: vec![],
			notes: Some(task.body.content),
			completion_date: task.completed_date_time,
			deletion_date: None,
			due_date: task.due_date_time,
			reminder_date: task.reminder_date_time,
			recurrence: task.recurrence.into(),
			created_date_time: task.created_date_time,
			last_modified_date_time: task.last_modified_date_time,
		}
	}
}

impl From<ChecklistItem> for Task {
	fn from(value: ChecklistItem) -> Self {
		Self {
			id: value.id,
			title: value.display_name,
			status: if value.is_checked {
				Status::Completed
			} else {
				Status::NotStarted
			},
			created_date_time: DateTime::<Utc>::from_utc(
				value.created_date_time,
				Utc,
			),
			..Default::default()
		}
	}
}
