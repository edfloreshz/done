use crate::services::microsoft::models::status::TaskStatus;
use serde::{Deserialize, Serialize};

#[derive(
	Clone,
	Copy,
	Default,
	Debug,
	PartialEq,
	Eq,
	Hash,
	PartialOrd,
	Ord,
	Serialize,
	Deserialize,
)]
pub enum Status {
	#[default]
	NotStarted = 0,
	Completed = 1,
}

impl From<i32> for Status {
	fn from(value: i32) -> Self {
		match value {
			0 => Self::NotStarted,
			1 => Status::Completed,
			_ => panic!("Invalid value for Status"),
		}
	}
}

impl From<Status> for i32 {
	fn from(value: Status) -> Self {
		match value {
			Status::NotStarted => 0,
			Status::Completed => 1,
		}
	}
}

impl Status {
	pub fn as_str_name(&self) -> &'static str {
		match self {
			Status::NotStarted => "NOT_STARTED",
			Status::Completed => "COMPLETED",
		}
	}
	/// Creates an enum from field names used in the ProtoBuf definition.
	pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
		match value {
			"NOT_STARTED" => Some(Self::NotStarted),
			"COMPLETED" => Some(Self::Completed),
			_ => None,
		}
	}
}

impl From<TaskStatus> for Status {
	fn from(value: TaskStatus) -> Self {
		match value {
			TaskStatus::NotStarted => Self::NotStarted,
			TaskStatus::Started => Self::NotStarted,
			TaskStatus::Completed => Self::Completed,
			TaskStatus::WaitingOnOthers => Self::NotStarted,
			TaskStatus::Deferred => Self::NotStarted,
		}
	}
}

impl From<Status> for TaskStatus {
	fn from(value: Status) -> Self {
		match value {
			Status::NotStarted => TaskStatus::NotStarted,
			Status::Completed => TaskStatus::Completed,
		}
	}
}
