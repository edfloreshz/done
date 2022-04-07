use std::time::SystemTime;
use chrono::DateTime;
use glib::clone;
use gtk4 as gtk;
use gtk4::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt, EntryExt, EntryBufferExtManual};
use relm4::{send, ComponentUpdate, Model, Widgets, Sender, MicroComponent, WidgetPlus};
use crate::{AppModel};
use crate::models::task::Task;
use crate::widgets::app::AppMsg;

pub struct ContentModel {
    tasks: Vec<MicroComponent<Task>>
}

pub enum ContentMsg {
    AddTaskEntry(String)
}

impl Model for ContentModel {
    type Msg = ContentMsg;
    type Widgets = ContentWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for ContentModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        let tasks = vec![
            Task {
                title: "Test".to_string(),
                body: "".to_string(),
                completed_on: None,
                due_date: None,
                importance: Default::default(),
                is_reminder_on: false,
                reminder_date: None,
                status: Default::default(),
                created_date_time: DateTime::from(SystemTime::now()),
                last_modified_date_time: DateTime::from(SystemTime::now())
            }
        ];
        ContentModel { tasks: tasks.iter().map(|task| {
            MicroComponent::new(task.to_owned(), ())
        }).collect() }
    }

    fn update(&mut self, msg: Self::Msg, _components: &Self::Components, _sender: Sender<Self::Msg>, _parent_sender: Sender<AppMsg>) {
        match msg {
            ContentMsg::AddTaskEntry(entry) => println!("Adding task with name {entry}")
        }
    }
}

#[relm4_macros::widget(pub)]
impl Widgets<ContentModel, AppModel> for ContentWidgets {
    view! {
        task_container = &gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 12,
            set_margin_all: 12,
            append: main_stack = &gtk::Stack {
                add_child = &gtk::ScrolledWindow {
                    set_vexpand: true,
                    set_hexpand: true,
                    set_child: list_box = Some(&gtk::Box) {
                        append: task_list = &gtk::ListBox {
                            set_hexpand: true,
                            append: iterate! {
                                model.tasks.iter().map(|task| {
                                    task.root_widget() as &gtk::Box
                                }).collect::<Vec<&gtk::Box>>()
                            }
                        },
                    }
                },
            },
            append: entry = &gtk::Entry {
                set_icon_from_icon_name: args!(gtk::EntryIconPosition::Primary, Some("list-add-symbolic")),
                set_placeholder_text: Some("New task..."),
                set_height_request: 42,
                connect_activate(sender) => move |entry| {
                    let buffer = entry.buffer();
                    send!(sender, ContentMsg::AddTaskEntry(buffer.text()));
                    buffer.delete_text(0, None);
                }
            }
        }
    }
    fn post_view() {
        for task in &model.tasks {
            if !task.is_connected() {
                self.task_list.append(task.root_widget())
            }
        }
    }
}