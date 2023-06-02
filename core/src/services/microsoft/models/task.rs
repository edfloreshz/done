use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{body::Body, importance::Importance, status::Status};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Task {
	pub id: String,
	pub body: Body,
	pub completed_date_time: Option<NaiveDateTime>,
	pub due_date_time: Option<NaiveDateTime>,
	pub importance: Importance,
	pub is_reminder_on: bool,
	pub reminder_date_time: Option<NaiveDateTime>,
	pub status: Status,
	pub title: String,
	pub created_date_time: String,
	pub last_modified_date_time: String,
}
