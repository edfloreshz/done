use diesel::{Insertable, Queryable};
use glib::Sender;
use gtk4::prelude::ListBoxRowExt;
use relm4::{gtk, gtk::prelude::{BoxExt, CheckButtonExt, OrientableExt}, send, WidgetPlus};
use relm4::factory::{FactoryPrototype, FactoryVec, FactoryView};
use relm4_macros::view;
use uuid::Uuid;

use crate::schema::tasks;
use crate::widgets::content::ContentMsg;

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "tasks"]
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
            last_modified_date_time: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
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
            last_modified_date_time: "".to_string(),
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
            importance: TaskImportance::from_importance_str(task.importance.as_str()),
            is_reminder_on: task.is_reminder_on,
            reminder_date: task.reminder_date,
            status: TaskStatus::from_status_str(task.status.as_str()),
            created_date_time: task.created_date_time,
            last_modified_date_time: task.last_modified_date_time,
        }
    }
}

impl From<Task> for QueryableTask {
    fn from(task: Task) -> Self {
        Self {
            id_task: task.id_task,
            id_list: task.id_list,
            title: task.title,
            body: task.body,
            completed_on: task.completed_on,
            due_date: task.due_date,
            importance: task.importance.to_importance_str(),
            is_reminder_on: task.is_reminder_on,
            reminder_date: task.reminder_date,
            status: task.status.to_status_str(),
            created_date_time: task.created_date_time,
            last_modified_date_time: task.last_modified_date_time,
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
    pub fn from_importance_str(imp: &str) -> Self {
        match imp.to_lowercase().as_str() {
            "normal" => Self::Normal,
            "high" => Self::High,
            _ => Self::Normal,
        }
    }
    pub fn to_importance_str(&self) -> String {
        match self {
            Self::High => "high".to_string(),
            _ => "normal".to_string(),
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
    pub fn from_status_str(status: &str) -> Self {
        match status.to_lowercase().as_str() {
            "completed" => Self::Completed,
            _ => Self::NotStarted,
        }
    }
    pub fn to_status_str(&self) -> String {
        match self {
            Self::Completed => "completed".to_string(),
            _ => "notStarted".to_string(),
        }
    }
    pub fn as_bool(&self) -> bool {
        matches!(self, Self::Completed)
    }
}

#[derive(Debug)]
pub enum TaskMsg {
    Complete,
    Edit,
    Delete,
}

#[derive(Debug)]
pub struct TaskWidgets {
    label: gtk::Label,
    row: gtk::ListBoxRow
}

impl FactoryPrototype for Task {
    type Factory = FactoryVec<Task>;
    type Widgets = TaskWidgets;
    type Root = gtk::ListBoxRow;
    type View = gtk::ListBox;
    type Msg = ContentMsg;

    fn init_view(&self, key: &usize, sender: Sender<Self::Msg>) -> Self::Widgets {
        let index = *key;
        view! {
            row = &gtk::ListBoxRow {
                set_child = Some(&gtk::Box) {
                    set_orientation: gtk::Orientation::Horizontal,
                    append = &gtk::CheckButton {
                        set_margin_all: 12,
                        set_active: self.status.as_bool(),
                        connect_toggled(sender) => move |checkbox| {
                            send!(sender, ContentMsg::SetCompleted((index, checkbox.is_active())));
                        }
                    },
                    append: label = &gtk::Label {
                        set_margin_all: 12,
                        set_label: &self.title
                    }
                }
            }
        }
        TaskWidgets {
            label,
            row,
        }
    }

    fn position(&self, _key: &usize) -> <Self::View as FactoryView<Self::Root>>::Position {}

    fn view(&self, _key: &usize, widgets: &Self::Widgets) {
        let attrs = widgets.label.attributes().unwrap_or_default();
        attrs.change(gtk::pango::AttrInt::new_strikethrough(
            matches!(self.status, TaskStatus::Completed))
        );
        widgets.label.set_attributes(Some(&attrs));
    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.row
    }
}