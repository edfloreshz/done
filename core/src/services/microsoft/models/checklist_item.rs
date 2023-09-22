use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistItem {
	#[serde(skip_serializing)]
	pub id: String,
	pub display_name: String,
	pub is_checked: bool,
	#[serde(skip_serializing)]
	pub created_date_time: Option<String>,
	pub checked_date_time: Option<String>,
}
