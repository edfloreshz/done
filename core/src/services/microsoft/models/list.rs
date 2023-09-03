use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TodoTaskList {
	pub id: String,
	pub display_name: String,
	pub is_owner: bool,
	pub is_shared: bool,
	pub wellknown_list_name: Option<WellKnownListName>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub enum WellKnownListName {
	#[default]
	None,
	DefaultList,
	FlaggedEmails,
	UnknownFutureValue,
}
