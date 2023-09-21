use serde::{Deserialize, Serialize};

use super::{
	body::ItemBody, checklist_item::ChecklistItem,
	date_time_zone::DateTimeTimeZone, importance::TaskImportance,
	recurrence::TaskRecurrence, status::TaskStatus,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TodoTask {
	pub id: String,
	pub body: ItemBody,
	pub categories: Vec<String>,
	pub completed_date_time: Option<DateTimeTimeZone>,
	pub due_date_time: Option<DateTimeTimeZone>,
	pub importance: TaskImportance,
	pub is_reminder_on: bool,
	pub recurrence: Option<TaskRecurrence>,
	pub title: String,
	pub status: TaskStatus,
	pub has_attachments: bool,
	pub checklist_items: Option<Vec<ChecklistItem>>,
	pub created_date_time: String,
	pub last_modified_date_time: String,
	pub reminder_date_time: Option<DateTimeTimeZone>,
	pub start_date_time: Option<DateTimeTimeZone>,
}
