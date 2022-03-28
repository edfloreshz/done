use crate::{BaseWidgets, UiEvent};
use chrono::{DateTime, Utc};
use gtk::prelude::*;
use gtk4 as gtk;
use relm4_macros::view;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

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
    pub fn fill_tasks(
        ui: &BaseWidgets,
        task_list_id: String,
        task_list: &Vec<Task>,
        ui_tx: Rc<RefCell<tokio::sync::mpsc::Sender<UiEvent>>>,
    ) {
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
            let container = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .build();
            let gesture = gtk::GestureClick::new();
            let sender2 = ui_tx.clone();
            let task_list_id_gesture = task_list_id.clone();
            let task_gesture = task.clone();
            gesture.connect_released(move |gesture, _, _, _| {
                gesture.set_state(gtk::EventSequenceState::Claimed);
                sender2
                    .borrow_mut()
                    .try_send(UiEvent::TaskSelected(
                        task_list_id_gesture.clone(),
                        task_gesture.clone().id,
                    ))
                    .expect("Failed to complete task.");
                println!("Box pressed!");
            });
            container.add_controller(&gesture);
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
            let sender = ui_tx.clone();
            let task_list_id = task_list_id.clone();
            checkbox.connect_toggled(move |_| {
                sender
                    .borrow_mut()
                    .try_send(UiEvent::TaskCompleted(
                        task_list_id.clone(),
                        task.clone().id,
                        task.completed,
                    ))
                    .expect("Failed to complete task.");
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
    Completed,
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
            _ => TaskStatus::NotStarted,
        }
    }
    pub fn is_completed(status: &str) -> bool {
        status.eq("completed")
    }
}
