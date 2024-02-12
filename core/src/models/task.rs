use std::str::FromStr;

use crate::services::microsoft::models::{
	body::{BodyType, ItemBody},
	checklist_item::ChecklistItem,
	task::TodoTask,
};
use chrono::{DateTime, Utc};
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

impl From<TodoTask> for Task {
	fn from(task: TodoTask) -> Self {
		Self {
			id: task.id,
			parent: String::new(),
			title: task.title,
			favorite: false,
			today: task.reminder_date_time.is_some()
				&& Into::<DateTime<Utc>>::into(
					task.reminder_date_time.clone().unwrap(),
				) == Utc::now(),
			status: task.status.into(),
			priority: task.importance.into(),
			sub_tasks: task
				.checklist_items
				.unwrap_or_default()
				.iter()
				.map(|item| item.clone().into())
				.collect(),
			tags: vec![],
			notes: Some(task.body.content),
			completion_date: task.completed_date_time.map(|date| date.into()),
			deletion_date: None,
			due_date: task.due_date_time.map(|date| date.into()),
			reminder_date: task.reminder_date_time.map(|date| date.into()),
			recurrence: task.recurrence.unwrap_or_default().into(),
			created_date_time: DateTime::<Utc>::from_str(&task.created_date_time)
				.unwrap(),
			last_modified_date_time: DateTime::<Utc>::from_str(
				&task.last_modified_date_time,
			)
			.unwrap(),
		}
	}
}

impl From<Task> for TodoTask {
	fn from(task: Task) -> Self {
		let checklist_items: Vec<ChecklistItem> =
			task.sub_tasks.iter().map(|t| t.to_owned().into()).collect();
		Self {
			id: task.id,
			body: ItemBody {
				content: task.notes.unwrap_or_default(),
				content_type: BodyType::Text,
			},
			categories: vec![],
			completed_date_time: task.completion_date.map(|date| date.into()),
			due_date_time: task.due_date.map(|date| date.into()),
			importance: task.priority.into(),
			is_reminder_on: task.reminder_date.is_some(),
			recurrence: Default::default(),
			title: task.title,
			status: task.status.into(),
			has_attachments: false,
			checklist_items: Some(checklist_items),
			created_date_time: task
				.created_date_time
				.format("%Y-%m-%dT%H:%M:%S%.fZ")
				.to_string(),
			last_modified_date_time: task
				.last_modified_date_time
				.format("%Y-%m-%dT%H:%M:%S%.fZ")
				.to_string(),
			reminder_date_time: task.reminder_date.map(|date| date.into()),
			start_date_time: None,
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
			created_date_time: DateTime::<Utc>::from_str(
				&value.created_date_time.unwrap_or_default(),
			)
			.unwrap(),
			..Default::default()
		}
	}
}

impl From<Task> for ChecklistItem {
	fn from(task: Task) -> Self {
		Self {
			display_name: task.title,
			created_date_time: None,
			checked_date_time: None,
			is_checked: matches!(task.status, Status::Completed),
			id: task.id,
		}
	}
}
