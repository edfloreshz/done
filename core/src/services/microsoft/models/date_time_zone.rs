use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(
	Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeTimeZone {
	pub date_time: String,
	pub time_zone: String,
}

impl From<DateTimeTimeZone> for DateTime<Utc> {
	fn from(date: DateTimeTimeZone) -> Self {
		let datetime =
			NaiveDateTime::parse_from_str(&date.date_time, "%Y-%m-%dT%H:%M:%S%.f")
				.or_else(|_| {
					NaiveDateTime::parse_from_str(&date.date_time, "%Y-%m-%dT%H:%M:%S")
				})
				.expect("Failed to parse date string");

		DateTime::<Utc>::from_utc(datetime, Utc)
	}
}

impl From<DateTime<Utc>> for DateTimeTimeZone {
	fn from(date: DateTime<Utc>) -> Self {
		Self {
			date_time: date.to_string(),
			time_zone: date.timezone().to_string(),
		}
	}
}
