use serde::{Deserialize, Serialize};

#[derive(
	Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "camelCase")]
pub struct TaskRecurrence {
	pub pattern: TaskRecurrencePattern,
	pub range: TaskRecurrenceRange,
}

#[derive(
	Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "camelCase")]
pub struct TaskRecurrenceRange {
	#[serde(rename = "type")]
	recurrence_type: RecurrenceRangeType,
	start_date: Option<String>,
	end_date: Option<String>,
	recurrence_time_zone: String,
	number_of_occurrences: i32,
}

#[derive(
	Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "camelCase")]
pub enum RecurrenceRangeType {
	EndDate,
	#[default]
	NoEnd,
	Numbered,
}

#[derive(
	Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "camelCase")]
pub struct TaskRecurrencePattern {
	#[serde(rename = "type")]
	pub recurrence_pattern_type: RecurrencePatternType,
	pub interval: i32,
	pub month: i32,
	pub day_of_month: i32,
	pub days_of_week: Vec<DayOfWeek>,
	pub first_day_of_week: DayOfWeek,
	pub index: Option<WeekIndex>,
}

#[derive(
	Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "camelCase")]
pub enum DayOfWeek {
	#[default]
	Sunday,
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
}

#[derive(
	Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "camelCase")]
pub enum WeekIndex {
	#[default]
	First,
	Second,
	Third,
	Fourth,
	Last,
}

#[derive(
	Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "camelCase")]
pub enum RecurrencePatternType {
	#[default]
	Daily,
	Weekly,
	AbsoluteMonthly,
	RelativeMonthly,
	AbsoluteYearly,
	RelativeYearly,
}
