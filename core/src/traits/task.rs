use chrono::NaiveDateTime;
use crate::enums::{TaskStatus, TaskImportance};
pub trait Task {
    fn id(&self) -> String;
    fn parent_list(&self) -> String;
    fn title(&self) -> String;
    fn body(&self) -> Option<String>;
    fn importance(&self) -> TaskImportance;
    fn favorite(&self) -> bool;
    fn is_reminder_on(&self) -> bool;
    fn status(&self) -> TaskStatus;
    fn completed_on(&self) -> Option<NaiveDateTime>;
    fn due_date(&self) -> Option<NaiveDateTime>;
    fn reminder_date(&self) -> Option<NaiveDateTime>;
    fn created_date_time(&self) -> Option<NaiveDateTime>;
    fn last_modified_date_time(&self) -> Option<NaiveDateTime>;
}

