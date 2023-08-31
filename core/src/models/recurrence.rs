use crate::services::microsoft::models::recurrence::DayOfWeek;
use crate::services::microsoft::models::recurrence::TaskRecurrence;
use serde::{Deserialize, Serialize};
#[derive(Debug)]
pub enum Day {
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
	Sunday,
}

impl ToString for Day {
	fn to_string(&self) -> String {
		match self {
			Day::Monday => "Mon".into(),
			Day::Tuesday => "Tue".into(),
			Day::Wednesday => "Wed".into(),
			Day::Thursday => "Thu".into(),
			Day::Friday => "Fri".into(),
			Day::Saturday => "Sat".into(),
			Day::Sunday => "Sun".into(),
		}
	}
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Recurrence {
	pub monday: bool,
	pub tuesday: bool,
	pub wednesday: bool,
	pub thursday: bool,
	pub friday: bool,
	pub saturday: bool,
	pub sunday: bool,
}

impl Recurrence {
	pub fn from_string(value: String) -> Self {
		Self {
			monday: value.contains(Day::Monday.to_string().as_str()),
			tuesday: value.contains(Day::Tuesday.to_string().as_str()),
			wednesday: value.contains(Day::Wednesday.to_string().as_str()),
			thursday: value.contains(Day::Thursday.to_string().as_str()),
			friday: value.contains(Day::Friday.to_string().as_str()),
			saturday: value.contains(Day::Saturday.to_string().as_str()),
			sunday: value.contains(Day::Sunday.to_string().as_str()),
		}
	}
}

impl ToString for Recurrence {
	fn to_string(&self) -> String {
		let mut rec = vec![];
		if self.monday {
			rec.push(Day::Monday.to_string())
		}
		if self.tuesday {
			rec.push(Day::Tuesday.to_string())
		}
		if self.wednesday {
			rec.push(Day::Wednesday.to_string())
		}
		if self.thursday {
			rec.push(Day::Thursday.to_string())
		}
		if self.friday {
			rec.push(Day::Friday.to_string())
		}
		if self.saturday {
			rec.push(Day::Saturday.to_string())
		}
		if self.sunday {
			rec.push(Day::Sunday.to_string())
		}
		rec.join(", ")
	}
}

impl From<TaskRecurrence> for Recurrence {
	fn from(value: TaskRecurrence) -> Self {
		Self {
			monday: value.pattern.days_of_week.contains(&DayOfWeek::Monday),
			tuesday: value.pattern.days_of_week.contains(&DayOfWeek::Tuesday),
			wednesday: value.pattern.days_of_week.contains(&DayOfWeek::Wednesday),
			thursday: value.pattern.days_of_week.contains(&DayOfWeek::Thursday),
			friday: value.pattern.days_of_week.contains(&DayOfWeek::Friday),
			saturday: value.pattern.days_of_week.contains(&DayOfWeek::Saturday),
			sunday: value.pattern.days_of_week.contains(&DayOfWeek::Sunday),
		}
	}
}
