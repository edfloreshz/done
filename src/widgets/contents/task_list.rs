use std::fmt::{Display, Formatter};
use std::str::FromStr;

use glib::Sender;
use relm4::{gtk, gtk::prelude::{
    BoxExt, ButtonExt, CheckButtonExt, EditableExt, OrientableExt, ToggleButtonExt, WidgetExt,
}, send, view, WidgetPlus};
use relm4::factory::{DynamicIndex, FactoryPrototype, FactoryVecDeque, FactoryView};
use relm4::gtk::prelude::{EntryBufferExtManual, EntryExt, ListBoxRowExt};
use uuid::Uuid;

use crate::models::task::QueryableTask;
use crate::widgets::contents::content::ContentMsg;

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

// Relm4

#[derive(Debug, Default)]
pub struct TaskWidgets {
    label: gtk::Entry,
    row: gtk::ListBoxRow,
}

impl FactoryPrototype for Task {
    type Factory = FactoryVecDeque<Task>;
    type Widgets = TaskWidgets;
    type Root = gtk::ListBoxRow;
    type View = gtk::Box;
    type Msg = ContentMsg;

    fn init_view(&self, key: &DynamicIndex, sender: Sender<Self::Msg>) -> Self::Widgets {
        let key = key.clone();
        let key2 = key.clone();
        let key3 = key.clone();
        let key4 = key.clone();
        let key5 = key.clone();
        view! {
            row = &gtk::ListBoxRow {
                set_selectable: false,
                set_child = Some(&gtk::Box) {

                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 5,
                        set_margin_top: 10,
                        set_margin_bottom: 10,
                        set_margin_start: 10,
                        set_margin_end: 10,
                        append = &gtk::CheckButton {
                            set_active: self.status.as_bool(),
                            connect_toggled(sender) => move |checkbox| {
                                send!(sender, ContentMsg::SetCompleted(key.clone(), checkbox.is_active()));
                            }
                        },
                        append = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 15,
                            append: label = &gtk::Entry {
                                add_css_class: "flat",
                                add_css_class: "no-border",
                                set_hexpand: true,
                                set_text: &self.title,
                                connect_activate(sender) => move |entry| {
                                    let buffer = entry.buffer();
                                    send!(sender, ContentMsg::ModifyTitle(key2.clone(), buffer.text()));
                                },
                                connect_changed(sender) => move |entry| {
                                    let buffer = entry.buffer();
                                    send!(sender, ContentMsg::ModifyTitle(key3.clone(), buffer.text()));
                                }
                            },
                            append: favorite = &gtk::ToggleButton {
                                add_css_class: "opaque",
                                add_css_class: "circular",
                                set_class_active: track!(self.changed(Task::favorite()), "favorite", self.favorite),
                                set_active: track!(self.changed(Task::favorite()), self.favorite),
                                set_icon_name: "starred-symbolic",
                                connect_toggled(sender) => move |button| {
                                    if button.is_active() {
                                        button.add_css_class("favorite");
                                    } else {
                                        button.remove_css_class("favorite");
                                    }
                                    send!(sender, ContentMsg::SetFavorite(key4.clone(), button.is_active()));
                                }
                            },
                            append: delete = &gtk::Button {
                                add_css_class: "destructive-action",
                                add_css_class: "circular",
                                set_icon_name: "user-trash-symbolic",
                                connect_clicked(sender) => move |_| {
                                    send!(sender, ContentMsg::Remove(key5.clone()));
                                }
                            }
                        }
                    }
                }
            }
        }
        TaskWidgets { label, row }
    }

    fn position(&self, _key: &DynamicIndex) -> <Self::View as FactoryView<Self::Root>>::Position {}

    fn view(&self, _key: &DynamicIndex, widgets: &Self::Widgets) {
        let attrs = widgets.label.attributes().unwrap_or_default();
        attrs.change(gtk::pango::AttrInt::new_strikethrough(matches!(
            self.status,
            TaskStatus::Completed
        )));
        widgets.label.set_attributes(&attrs);
    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.row
    }
}
