use crate::services::microsoft::types::{DateTimeTimeZone, ItemBody};
use crate::{BaseWidgets, UiEvent};
use gtk::prelude::*;
use gtk4 as gtk;
use relm4_macros::view;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub body: ItemBody,
    pub completed_date_time: Option<DateTimeTimeZone>,
    pub due_date_time: Option<DateTimeTimeZone>,
    pub importance: TaskImportance,
    pub is_reminder_on: bool,
    // pub recurrence: PatternedRecurrence,
    pub reminder_date_time: Option<DateTimeTimeZone>,
    pub status: TaskStatus,
    pub title: String,
    pub created_date_time: String,
    pub last_modified_date_time: String,
}

impl Task {
    pub fn fill_tasks(
        ui: &BaseWidgets,
        task_list_id: String,
        task_list: &[Task],
        ui_tx: Rc<RefCell<tokio::sync::mpsc::Sender<UiEvent>>>,
    ) {
        ui.content.remove(&ui.content.last_child().unwrap());
        let task_list_id_2 = task_list_id.clone();
        view! {
            container = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,
                set_vexpand: true,
                set_width_request: 500,
                set_spacing: 12,

                append = &gtk::ScrolledWindow {
                    set_min_content_height: 360,
                    set_hexpand: true,
                    set_vexpand: true,
                    set_child: tasks = Some(&gtk::ListBox) {}
                },
                append: entry = &gtk::Entry {
                    connect_activate(ui_tx) => move |entry| {
                        let buffer = entry.buffer();
                        ui_tx.borrow_mut()
                            .try_send(UiEvent::AddEntry(buffer.text(), task_list_id_2.clone()))
                            .expect("Failed to send ");
                        buffer.delete_text(0, None);
                    }
                }
            }
        }
        for task in task_list.iter().cloned() {
            let container = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .build();
            let gesture = gtk::GestureClick::new();
            let sender = ui_tx.clone();
            let task_list_id_gesture = task_list_id.clone();
            let task_gesture = task.clone();
            gesture.connect_released(move |gesture, _, _, _| {
                gesture.set_state(gtk::EventSequenceState::Claimed);
                sender
                    .borrow_mut()
                    .try_send(UiEvent::TaskSelected(
                        task_list_id_gesture.clone(),
                        task_gesture.clone().id,
                    ))
                    .expect("Failed to complete task");
            });
            container.add_controller(&gesture);
            let checkbox = gtk::CheckButton::builder()
                .active(task.is_completed())
                .build();
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
                        task.is_completed(),
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
    pub fn fill_details(
        ui: &BaseWidgets,
        task_list_id: String,
        task: Task,
        ui_tx: Rc<RefCell<tokio::sync::mpsc::Sender<UiEvent>>>,
    ) {
        let reveals = ui.details.revealer.reveals_child();
        if reveals {
            if let Some(child) = ui.details.navigation_box.last_child() {
                ui.details.navigation_box.remove(&child);
            }
            ui.details.revealer.set_reveal_child(!reveals);
        } else {
            view! {
                container = gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,
                    set_vexpand: true,
                    set_margin_start: 15,
                    set_margin_bottom: 15,
                    set_margin_end: 15,
                    set_margin_top: 15,
                    set_spacing: 20,

                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,

                        append = &gtk::CheckButton {
                            set_active: task.is_completed(),

                            connect_toggled(ui_tx) => move |_| {
                                ui_tx.borrow_mut().try_send(UiEvent::TaskCompleted(
                                        task_list_id.clone(),
                                        task.clone().id,
                                        task.is_completed(),
                                    )).expect("");
                            }
                        },
                        append = &gtk::Entry {
                            set_placeholder_text: Some("Title"),
                            set_hexpand: true,
                            set_text: task.title.as_str()
                        },

                    },
                    append = &gtk::Button {
                        set_label: "+ Add Step"
                    },
                    append = &gtk::Separator {},
                    append = &gtk::Button {
                        set_label: "Add to My Day"
                    },
                    append = &gtk::Separator {},
                    append = &gtk::Button {
                        set_label: "Remind me"
                    },
                    append = &gtk::Button {
                        set_label: "Due"
                    },
                    append = &gtk::Button {
                        set_label: "Repeat"
                    },
                    append = &gtk::Separator {},
                    append = &gtk::Button {
                        set_label: "Add file"
                    },
                    append = &gtk::Separator {},
                    append = &gtk::Entry {
                        set_placeholder_text: Some("Add Note"),
                        set_hexpand: true,
                    },
                }
            }
            ui.details.navigation_box.append(&container);
            ui.details.revealer.set_reveal_child(!reveals);
        }
    }
    pub fn is_completed(&self) -> bool {
        self.status == TaskStatus::Completed
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            body: ItemBody::default(),
            completed_date_time: None,
            due_date_time: None,
            importance: TaskImportance::default(),
            is_reminder_on: false,
            // recurrence: Default::default(),
            reminder_date_time: None,
            status: TaskStatus::default(),
            title: "".to_string(),
            created_date_time: String::new(),
            last_modified_date_time: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    NotStarted,
    Started,
    Completed,
    WaitingOnOthers,
    Deferred,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::NotStarted
    }
}
