use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use uuid::Uuid;

use crate::traits::Task;
use crate::enums::{TaskImportance, TaskStatus};

use crate::schema::tasks;

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = tasks)]
pub struct QueryableTask {
	pub id_task: String,
	pub parent_list: String,
	pub title: String,
	pub body: Option<String>,
    pub importance: i32,
	pub favorite: bool,
	pub is_reminder_on: bool,
    pub status: i32,
	pub completed_on: Option<NaiveDateTime>,
	pub due_date: Option<NaiveDateTime>,
	pub reminder_date: Option<NaiveDateTime>,
	pub created_date_time: Option<NaiveDateTime>,
	pub last_modified_date_time: Option<NaiveDateTime>,
}

impl QueryableTask {
	pub fn new(title: String, parent_list: String) -> Self {
		Self {
			id_task: Uuid::new_v4().to_string(),
			parent_list,
			title,
			body: None,
			completed_on: None,
			due_date: None,
			importance: TaskImportance::Low as i32,
			favorite: false,
			is_reminder_on: false,
			reminder_date: None,
			status: TaskStatus::NotStarted as i32,
			created_date_time: None,
			last_modified_date_time: None,
		}
	}

	pub fn from(task: &impl Task) -> Self {
		Self {
			id_task: task.id(),
			parent_list: task.parent_list(),
			title: task.title(),
			body: task.body(),
			completed_on: task.completed_on(),
			due_date: task.due_date(),
			importance: task.importance() as i32,
			favorite: task.favorite(),
			is_reminder_on: task.is_reminder_on(),
			reminder_date: task.reminder_date(),
            status: task.status() as i32,
			created_date_time: task.created_date_time(),
			last_modified_date_time: task.last_modified_date_time(),
		}
	}
}

impl Task for QueryableTask {
    fn id(&self) -> String {
        self.id_task.clone()
    }

    fn parent_list(&self) -> String {
        self.parent_list.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn body(&self) -> Option<String> {
        self.body.clone()
    }

    fn importance(&self) -> TaskImportance {
        match self.importance {
            3 => TaskImportance::High,
            2 => TaskImportance::Normal,
            _ => TaskImportance::Low
        }
    }

    fn favorite(&self) -> bool {
        self.favorite
    }

    fn is_reminder_on(&self) -> bool {
        self.is_reminder_on
    }

    fn status(&self) -> TaskStatus {
        match self.status {
            2 => TaskStatus::Completed,
            _ => TaskStatus::NotStarted
        }
    }

    fn completed_on(&self) -> Option<NaiveDateTime> {
        self.completed_on
    }

    fn due_date(&self) -> Option<NaiveDateTime> {
        self.due_date
    }

    fn reminder_date(&self) -> Option<NaiveDateTime> {
        self.reminder_date
    }

    fn created_date_time(&self) -> Option<NaiveDateTime> {
        self.created_date_time
    }

    fn last_modified_date_time(&self) -> Option<NaiveDateTime> {
        self.last_modified_date_time
    }
}