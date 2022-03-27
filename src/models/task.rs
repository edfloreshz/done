use std::cell::RefCell;
use std::rc::Rc;
use chrono::{DateTime, Utc};
use gtk4 as gtk;
use gtk::prelude::*;
use relm4_macros::view;
use serde::{Deserialize, Serialize};
use crate::{BaseWidgets, UiEvent};

#[allow(dead_code)]
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

impl Task {
    pub fn fill_tasks(ui: &BaseWidgets, task_list_id: String, task_list: &Vec<Task>, ui_event_sender: Rc<RefCell<tokio::sync::mpsc::Sender<UiEvent>>>) {
        ui.content.remove(&ui.content.last_child().unwrap());
        view! {
            container = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,
                set_vexpand: true,
                set_spacing: 12,

                append = &gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,
                    set_min_content_height: 360,
                    set_hexpand: true,
                    set_vexpand: true,
                    set_child: tasks = Some(&gtk::ListBox) {

                    }
                },
                append: entry = &gtk::Entry {

                }
            }
        }
        for task in task_list.clone() {
            let task_list_id_2 = task_list_id.clone();
            let container = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .build();
            let checkbox = gtk::CheckButton::builder().active(task.completed).build();
            let label = gtk::Label::builder().label(&task.title).build();

            checkbox.set_margin_end(12);
            checkbox.set_margin_start(12);
            checkbox.set_margin_top(12);
            checkbox.set_margin_bottom(12);
            label.set_margin_end(12);
            label.set_margin_start(12);
            label.set_margin_top(12);
            label.set_margin_bottom(12);

            container.append(&checkbox);
            container.append(&label);
            let sender = ui_event_sender.clone();
            checkbox.connect_toggled(move |_| {
                sender.borrow_mut().try_send(UiEvent::TaskCompleted(task_list_id_2.clone(), task.clone().id, task.completed)).expect("Failed to complete task.");
            });
            tasks.append(&container);
        }
        entry.connect_activate(move |entry| {
            let buffer = entry.buffer();
            buffer.delete_text(0, None);
        });
        ui.content.set_halign(gtk::Align::Fill);
        ui.content.append(&container);
    }

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
            _ => TaskImportance::Normal,
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