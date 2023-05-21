use chrono::Weekday;
use serde::{Deserialize, Serialize};

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
			monday: value.contains(Weekday::Mon.to_string().as_str()),
			tuesday: value.contains(Weekday::Tue.to_string().as_str()),
			wednesday: value.contains(Weekday::Wed.to_string().as_str()),
			thursday: value.contains(Weekday::Thu.to_string().as_str()),
			friday: value.contains(Weekday::Fri.to_string().as_str()),
			saturday: value.contains(Weekday::Sat.to_string().as_str()),
			sunday: value.contains(Weekday::Sun.to_string().as_str()),
		}
	}
}

impl ToString for Recurrence {
	fn to_string(&self) -> String {
		let mut rec = vec![];
		if self.monday {
			rec.push(Weekday::Mon.to_string())
		}
		if self.tuesday {
			rec.push(Weekday::Tue.to_string())
		}
		if self.wednesday {
			rec.push(Weekday::Wed.to_string())
		}
		if self.thursday {
			rec.push(Weekday::Thu.to_string())
		}
		if self.friday {
			rec.push(Weekday::Fri.to_string())
		}
		if self.saturday {
			rec.push(Weekday::Sat.to_string())
		}
		if self.sunday {
			rec.push(Weekday::Sun.to_string())
		}
		rec.join(", ")
	}
}
