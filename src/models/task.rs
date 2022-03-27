use std::time::SystemTime;
use chrono::{DateTime, Utc};
use gtk::prelude::{
    BoxExt, CheckButtonExt, EntryBufferExtManual, EntryExt, OrientableExt, WidgetExt,
};
use relm4::factory::{Factory, FactoryPrototype, FactoryVec};
use relm4::{gtk, send, Sender, WidgetPlus, MicroModel, MicroWidgets};
use serde::{Serialize, Deserialize};
pub enum TaskMsg {
    SetCompleted((usize, bool)),
    AddEntry(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: String,
    pub importance: TaskImportance,
    #[serde(rename = "isReminderOn")]
    pub is_reminder_on: bool,
    pub status: TaskStatus,
    pub title: String,
    pub created: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub completed: bool,
}

unsafe impl Send for Task {}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ToDoTask {
    pub id: String,
    pub importance: String,
    #[serde(rename = "isReminderOn")]
    pub is_reminder_on: bool,
    pub status: String,
    pub title: String,
    #[serde(rename = "createdDateTime")]
    pub created: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskImportance {
    Normal,
}

impl Default for TaskImportance {
    fn default() -> Self {
        TaskImportance::Normal
    }
}

impl TaskImportance {
    pub fn from(importance: &str) -> Self {
        match importance {
            "normal" => TaskImportance::Normal,
            _ => TaskImportance::Normal
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    NotStarted,
    Started,
    Completed
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::NotStarted
    }
}

impl TaskStatus {
    pub fn from(status: &str) -> Self {
        match status {
            "notStarted" => TaskStatus::NotStarted,
            "started" => TaskStatus::Started,
            "completed" => TaskStatus::Completed,
            _ => TaskStatus::NotStarted
        }
    }
    pub fn is_completed(status: &str) -> bool {
        status.eq("completed")
    }
}

#[derive(Debug)]
pub struct TaskWidgets {
    label: gtk::Label,
    container: gtk::Box
}

impl FactoryPrototype for Task {
    type Factory = FactoryVec<Task>;
    type Widgets = TaskWidgets;
    type Root = gtk::Box;
    type View = gtk::ListBox;
    type Msg = TaskMsg;

    fn init_view(&self, key: &<Self::Factory as Factory<Self, Self::View>>::Key, sender: Sender<Self::Msg>) -> Self::Widgets {
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        let checkbox = gtk::CheckButton::builder().active(false).build();
        let label = gtk::Label::new(Some(&self.title));

        checkbox.set_margin_all(12);
        label.set_margin_all(12);

        container.append(&checkbox);
        container.append(&label);

        let index = *key;
        checkbox.connect_toggled(move |checkbox| {
            send!(sender, TaskMsg::SetCompleted((index, checkbox.is_active())));
        });

        TaskWidgets { label, container }
    }

    fn position(&self, _index: &usize) {}

    fn view(&self, key: &usize, widgets: &Self::Widgets) {
        let attrs = widgets.label.attributes().unwrap_or_default();
        attrs.change(gtk::pango::AttrInt::new_strikethrough(self.completed));
        widgets.label.set_attributes(Some(&attrs));
    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.container
    }
}

#[derive(Clone)]
pub struct TaskModel {
    pub tasks: Vec<Task>
}

impl MicroModel for TaskModel {
    type Msg = TaskMsg;
    type Widgets = TaskModelWidgets;
    type Data = ();

    fn update(&mut self, msg: Self::Msg, data: &Self::Data, sender: Sender<Self::Msg>) {
        match msg {
            TaskMsg::SetCompleted((index, completed)) => {
                if let Some(task) = self.tasks.get_mut(index) {
                    task.completed = completed;
                }
            }
            TaskMsg::AddEntry(name) => {
                self.tasks.push(Task {
                    id: "".to_string(),
                    importance: TaskImportance::Normal,
                    is_reminder_on: false,
                    status: TaskStatus::NotStarted,
                    title: name,
                    created: DateTime::from(SystemTime::now()),
                    last_modified: DateTime::from(SystemTime::now()),
                    completed: false,
                });
            }
        }
    }
}

#[relm4::micro_widget(pub)]
#[derive(Debug)]
impl MicroWidgets<TaskModel> for TaskModelWidgets {
    view! {
        container = &gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_hexpand: true,
            set_margin_all: 12,
            set_spacing: 6,

            append = &gtk::ScrolledWindow {
                set_hscrollbar_policy: gtk::PolicyType::Never,
                set_min_content_height: 360,
                set_vexpand: true,
                set_child = Some(&gtk::ListBox) {
                    factory!(FactoryVec::from_vec(model.tasks.clone()))
                }
            },
            append = &gtk::Entry {
                connect_activate(sender) => move |entry| {
                    let buffer = entry.buffer();
                    send!(sender, TaskMsg::AddEntry(buffer.text()));
                    buffer.delete_text(0, None);
                }
            }
        }
    }
}