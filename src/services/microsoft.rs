use std::cell::RefCell;
use crate::services::ToDoService;
use crate::{BaseWidgets, List, UiEvent};
use anyhow::Context;
use cascade::cascade;
use chrono::{DateTime, Utc};
use libdmd::config::Config;
use libdmd::element::Element;
use libdmd::format::{ElementFormat, FileType};
use libdmd::{dir, fi};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use gtk4 as gtk;
use gtk::prelude::*;
use relm4_macros::view;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MicrosoftTokenAccess {
    pub expires_in: usize,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Collection<T> {
    pub value: Vec<T>,
}

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
        task_list: &Vec<Task>,
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
        for task in task_list.clone() {
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
            let checkbox = gtk::CheckButton::builder().active(task.is_completed()).build();
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeTimeZone {
    pub date_time: String,
    pub time_zone: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ItemBody {
    content: String,
    content_type: BodyType
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BodyType {
    Text,
    Html
}

impl Default for BodyType {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatternedRecurrence {
    pub pattern: RecurrencePattern,
    pub range: RecurrenceRange
}

impl Default for PatternedRecurrence {
    fn default() -> Self {
        Self {
            pattern: RecurrencePattern::default(),
            range: RecurrenceRange::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecurrencePattern {
    pub day_of_month: Option<i32>,
    pub days_of_week: Option<Vec<String>>,
    pub first_day_of_week: Option<DayOfWeek>,
    pub index: Option<WeekIndex>,
    pub interval: i32,
    pub month: i32,
    #[serde(rename = "type")]
    pub recurrence_type: Option<RecurrenceType>,
}

impl Default for RecurrencePattern {
    fn default() -> Self {
        Self {
            day_of_month: None,
            days_of_week: None,
            first_day_of_week: None,
            index: None,
            interval: 0,
            month: 0,
            recurrence_type: None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecurrenceRange {
    pub end_date: Option<DateTime<Utc>>,
    pub number_of_occurrences: i32,
    pub recurrence_time_zone: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub recurrence_range_type: Option<RecurrenceRangeType>
}

impl Default for RecurrenceRange {
    fn default() -> Self {
        Self {
            end_date: None,
            number_of_occurrences: 0,
            recurrence_time_zone: None,
            start_date: None,
            recurrence_range_type: None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum WeekIndex {
    First,
    Second,
    Third,
    Fourth,
    Last
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RecurrenceType {
    Daily,
    Weekly,
    AbsoluteMonthly,
    RelativeMonthly,
    AbsoluteYearly,
    RelativeYearly,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RecurrenceRangeType {
    EndDate,
    NoEnd,
    Numbered
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TaskImportance {
    Low,
    Normal,
    High
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

#[async_trait::async_trait]
impl ToDoService<MicrosoftTokenAccess> for MicrosoftTokenAccess {
    fn create_config(config: &mut Config) -> anyhow::Result<Config> {
        config
            .add(dir!("services").child(dir!("microsoft").child(fi!("token.toml"))))
            .write()
    }

    fn is_token_present() -> bool {
        let config = MicrosoftTokenAccess::current_token_data();
        match config {
            Some(config) => !config.refresh_token.is_empty(),
            None => false,
        }
    }

    fn current_token_data() -> Option<MicrosoftTokenAccess> {
        Config::get::<MicrosoftTokenAccess>("ToDo/services/microsoft/token.toml", FileType::TOML)
    }

    fn update_token_data(config: &MicrosoftTokenAccess) -> anyhow::Result<()> {
        Config::set(
            "ToDo/services/microsoft/token.toml",
            config.clone(),
            FileType::TOML,
        )
    }

    async fn token(code: &str) -> anyhow::Result<MicrosoftTokenAccess> {
        let client = reqwest::Client::new();
        let params = cascade! {
            HashMap::new();
            ..insert("client_id", "af13f4ae-b607-4a07-9ddc-6c5c5d59979f");
            ..insert("scope", "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared");
            ..insert("redirect_uri", "https://login.microsoftonline.com/common/oauth2/nativeclient");
            ..insert("grant_type", "authorization_code");
            ..insert("code", code);
        };
        let response = client
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&params)
            .send()
            .await?;
        if response.status().is_success() {
            let response = response.text().await?;
            let token_data: MicrosoftTokenAccess = serde_json::from_str(response.as_str())?;
            MicrosoftTokenAccess::update_token_data(&token_data)?;
            Ok(token_data)
        } else {
            // TODO: Let know the user the error.
            Ok(MicrosoftTokenAccess::default())
        }
    }

    async fn refresh_token(refresh_token: &str) -> anyhow::Result<MicrosoftTokenAccess> {
        let client = reqwest::Client::new();
        let params = cascade! {
            HashMap::new();
            ..insert("client_id", "af13f4ae-b607-4a07-9ddc-6c5c5d59979f");
            ..insert("scope", "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared");
            ..insert("redirect_uri", "https://login.microsoftonline.com/common/oauth2/nativeclient");
            ..insert("grant_type", "refresh_token");
            ..insert("refresh_token", refresh_token);
        };
        let response = client
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&params)
            .send()
            .await?;
        if response.status().is_success() {
            let response = response.text().await?;
            let token_data: MicrosoftTokenAccess = serde_json::from_str(response.as_str())?;
            MicrosoftTokenAccess::update_token_data(&token_data)?;
            Ok(token_data)
        } else {
            // TODO: Let know the user the error.
            Ok(MicrosoftTokenAccess::default())
        }
    }

    async fn get_lists() -> anyhow::Result<Vec<List>> {
        let config = MicrosoftTokenAccess::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let client = reqwest::Client::new();
        let response = client
            .get("https://graph.microsoft.com/v1.0/me/todo/lists")
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        if response.status().is_success() {
            let lists = response.text().await?;
            let lists: Collection<List> = serde_json::from_str(lists.as_str())?;
            Ok(lists.value)
        } else {
            Ok(vec![])
        }
    }

    async fn get_tasks(task_list_id: &str) -> anyhow::Result<Vec<Task>> {
        let config = MicrosoftTokenAccess::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks",
                task_list_id
            ))
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        if response.status().is_success() {
            let response = response.text().await?;
            let collection: Collection<Task> = serde_json::from_str(response.as_str())?;
            Ok(collection.value)
        } else {
            Ok(vec![])
        }
    }

    async fn set_task_as_completed(
        task_list_id: &str,
        task_id: &str,
        completed: bool,
    ) -> anyhow::Result<Vec<Task>> {
        let config = MicrosoftTokenAccess::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let status = format!(
            "{}:{}",
            "{\"status\"",
            if completed {
                "\"notStarted\"}"
            } else {
                "\"completed\"}"
            }
        );
        let client = reqwest::Client::new();
        let response = client
            .patch(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks/{}",
                task_list_id, task_id
            ))
            .header("Content-Type", "application/json")
            .body(status)
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        if response.status().is_success() {
            let response = response.text().await?;
            let collection: Collection<Task> = serde_json::from_str(response.as_str())?;
            Ok(collection.value)
        } else {
            Ok(vec![])
        }
    }

    async fn get_task(task_list_id: &str, task_id: &str) -> anyhow::Result<Task> {
        let config = MicrosoftTokenAccess::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks/{}",
                task_list_id, task_id
            ))
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        match response.error_for_status() {
            Ok(response) => {
                let response = response.text().await?;
                let task: Task = serde_json::from_str(response.as_str())?;
                Ok(task.into())
            }
            Err(error) => Err(error.into()),
        }
    }

    async fn push_task(task_list_id: &str, entry: String) -> anyhow::Result<()> {
        let config = MicrosoftTokenAccess::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let client = reqwest::Client::new();
        let task = Task {
            title: entry,
            ..std::default::Default::default()
        };
        let data = serde_json::to_string(&task).unwrap();
        let request = client
            .post(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks",
                task_list_id
            ))
            .header("Content-Type", "application/json")
            .bearer_auth(&config.access_token)
            .body(data);
        let response = request.send().await?;
        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}