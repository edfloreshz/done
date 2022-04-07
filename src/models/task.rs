use chrono::{DateTime, Utc};
use glib::Sender;
use relm4::{gtk, gtk::prelude::{WidgetExt, BoxExt, OrientableExt}, WidgetPlus, MicroModel, MicroWidgets};
use uuid::Uuid;
use diesel::{Insertable, Queryable};
use crate::schema::lists::id_list;
use crate::schema::tasks;
use crate::schema::tasks::importance;

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name="tasks"]
pub struct QueryableTask {
    pub id_task: String,
    pub id_list: String,
    pub title: String,
    pub body: String,
    pub completed_on: Option<String>,
    pub due_date: Option<String>,
    pub importance: String,
    pub is_reminder_on: bool,
    pub reminder_date: Option<String>,
    pub status: String,
    pub created_date_time: String,
    pub last_modified_date_time: String,
}

impl QueryableTask {
    pub fn new(title: String, list_id: String) -> Self {
        Self {
            id_task: Uuid::new_v4().to_string(),
            id_list: list_id,
            title,
            body: "".to_string(),
            completed_on: None,
            due_date: None,
            importance: Default::default(),
            is_reminder_on: false,
            reminder_date: None,
            status: Default::default(),
            created_date_time: "".to_string(),
            last_modified_date_time: "".to_string()
        }
    }
}

#[derive(Debug)]
pub struct Task {
    pub id_task: String,
    pub id_list: String,
    pub title: String,
    pub body: String,
    pub completed_on: Option<String>,
    pub due_date: Option<String>,
    pub importance: TaskImportance,
    pub is_reminder_on: bool,
    pub reminder_date: Option<String>,
    pub status: TaskStatus,
    pub created_date_time: String,
    pub last_modified_date_time: String,
}

impl Task {
    pub fn new(title: String, list_id: String) -> Self {
        Self {
            id_task: Uuid::new_v4().to_string(),
            id_list: list_id,
            title,
            body: "".to_string(),
            completed_on: None,
            due_date: None,
            importance: Default::default(),
            is_reminder_on: false,
            reminder_date: None,
            status: Default::default(),
            created_date_time: "".to_string(),
            last_modified_date_time: "".to_string()
        }
    }
}

impl From<QueryableTask> for Task {
    fn from(task: QueryableTask) -> Self {
        Self {
            id_task: task.id_task,
            id_list: task.id_list,
            title: task.title,
            body: task.body,
            completed_on: task.completed_on,
            due_date: task.due_date,
            importance: TaskImportance::from_str(task.importance.as_str()),
            is_reminder_on: task.is_reminder_on,
            reminder_date: task.reminder_date,
            status: TaskStatus::from_str(task.status.as_str()),
            created_date_time: task.created_date_time,
            last_modified_date_time: task.last_modified_date_time
        }
    }
}

#[derive(Debug, Clone)]
pub enum TaskImportance {
    Low,
    Normal,
    High,
}

impl TaskImportance {
    pub fn from_str(imp: &str) -> Self {
        match imp.to_lowercase().as_str() {
            "normal" => Self::Normal,
            "high" => Self::High,
            _ => Self::Normal
        }
    }
}

impl Default for TaskImportance {
    fn default() -> Self {
        TaskImportance::Normal
    }
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    NotStarted,
    Completed,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::NotStarted
    }
}

impl TaskStatus {
    pub fn from_str(status: &str) -> Self {
        match status.to_lowercase().as_str() {
            "completed" => Self::Completed,
            _ => Self::NotStarted
        }
    }
}

#[derive(Debug)]
pub enum TaskMsg {
    Complete,
    Edit,
    Delete
}

impl MicroModel for Task {
    type Msg = TaskMsg;
    type Widgets = TaskWidgets;
    type Data = ();

    fn update(&mut self, msg: Self::Msg, data: &Self::Data, sender: Sender<Self::Msg>) {
        todo!()
    }
}

#[relm4::micro_widget(pub)]
#[derive(Debug)]
impl MicroWidgets<Task> for TaskWidgets {
    view! {
        task_box = &gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            append = &gtk::CheckButton {
                set_margin_all: 12
            },
            append = &gtk::Label {
                set_margin_all: 12,
                set_label: &model.title
            }
        }
    }
}