use serde::{Deserialize, Serialize};

#[derive(
	Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
	#[default]
	NotStarted,
	Started,
	Completed,
	WaitingOnOthers,
	Deferred,
}
