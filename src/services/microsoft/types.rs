use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Collection<T> {
    pub value: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeTimeZone {
    pub date_time: String,
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ItemBody {
    content: String,
    content_type: BodyType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BodyType {
    Text,
    Html,
}

impl Default for BodyType {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatternedRecurrence {
    pub pattern: RecurrencePattern,
    pub range: RecurrenceRange,
}

impl Default for PatternedRecurrence {
    fn default() -> Self {
        Self {
            pattern: RecurrencePattern::default(),
            range: RecurrenceRange::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecurrencePattern {
    pub day_of_month: Option<i32>,
    pub days_of_week: Option<Vec<String>>,
    pub first_day_of_week: Option<DayOfWeek>,
    pub index: Option<WeekIndex>,
    pub interval: i32,
    pub month: i32,
    #[serde(rename = "type")]
    pub recurrence_type: Option<RecurrenceType>,
}

impl Default for RecurrencePattern {
    fn default() -> Self {
        Self {
            day_of_month: None,
            days_of_week: None,
            first_day_of_week: None,
            index: None,
            interval: 0,
            month: 0,
            recurrence_type: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecurrenceRange {
    pub end_date: Option<DateTime<Utc>>,
    pub number_of_occurrences: i32,
    pub recurrence_time_zone: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub recurrence_range_type: Option<RecurrenceRangeType>,
}

impl Default for RecurrenceRange {
    fn default() -> Self {
        Self {
            end_date: None,
            number_of_occurrences: 0,
            recurrence_time_zone: None,
            start_date: None,
            recurrence_range_type: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum WeekIndex {
    First,
    Second,
    Third,
    Fourth,
    Last,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RecurrenceType {
    Daily,
    Weekly,
    AbsoluteMonthly,
    RelativeMonthly,
    AbsoluteYearly,
    RelativeYearly,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RecurrenceRangeType {
    EndDate,
    NoEnd,
    Numbered,
}
