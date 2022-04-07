use chrono::{DateTime, Utc};
use glib::Sender;
use relm4::{gtk, gtk::prelude::{WidgetExt, BoxExt, OrientableExt}, WidgetPlus, MicroModel, MicroWidgets};

#[derive(Debug, Clone)]
pub struct Task {
    pub id_task: String,
    pub title: String,
    pub body: String,
    pub completed_on: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    pub importance: TaskImportance,
    pub is_reminder_on: bool,
    pub reminder_date: Option<DateTime<Utc>>,
    pub status: TaskStatus,
    pub created_date_time: DateTime<Utc>,
    pub last_modified_date_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum TaskImportance {
    Low,
    Normal,
    High,
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