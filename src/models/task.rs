use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::Uuid;
use crate::models::queryable::task::QueryableTask;

#[tracker::track]
#[derive(Debug, Clone, Default)]
pub struct Task {
    pub id_task: String,
    pub id_list: String,
    pub title: String,
    pub body: Option<String>,
    pub completed_on: Option<String>,
    pub due_date: Option<String>,
    #[tracker::no_eq]
    pub importance: TaskImportance,
    pub favorite: bool,
    pub is_reminder_on: bool,
    pub reminder_date: Option<String>,
    #[tracker::no_eq]
    pub status: TaskStatus,
    pub created_date_time: Option<String>,
    pub last_modified_date_time: Option<String>,
}

impl Task {
    pub fn new(title: String, list_id: String) -> Self {
        Self {
            id_task: Uuid::new_v4().to_string(),
            id_list: list_id,
            title,
            body: None,
            completed_on: None,
            due_date: None,
            importance: Default::default(),
            favorite: false,
            is_reminder_on: false,
            reminder_date: None,
            status: Default::default(),
            created_date_time: None,
            last_modified_date_time: None,
            tracker: 0,
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
            importance: TaskImportance::from_str(task.importance.unwrap().as_str())
                .unwrap_or_default(),
            favorite: task.favorite,
            is_reminder_on: task.is_reminder_on,
            reminder_date: task.reminder_date,
            status: TaskStatus::from_str(task.status.unwrap().as_str()).unwrap_or_default(),
            created_date_time: task.created_date_time,
            last_modified_date_time: task.last_modified_date_time,
            tracker: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TaskImportance {
    Low,
    Normal,
    High,
}

impl Display for TaskImportance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskImportance::Low => write!(f, "low"),
            TaskImportance::Normal => write!(f, "normal"),
            TaskImportance::High => write!(f, "high"),
        }
    }
}

impl FromStr for TaskImportance {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(TaskImportance::Low),
            "normal" => Ok(TaskImportance::Normal),
            "high" => Ok(TaskImportance::High),
            _ => Err(()),
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

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::NotStarted => write!(f, "notStarted"),
            TaskStatus::Completed => write!(f, "completed"),
        }
    }
}

impl FromStr for TaskStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "notstarted" => Ok(TaskStatus::NotStarted),
            "completed" => Ok(TaskStatus::Completed),
            _ => Err(()),
        }
    }
}

impl TaskStatus {
    pub fn as_bool(&self) -> bool {
        matches!(self, Self::Completed)
    }
}