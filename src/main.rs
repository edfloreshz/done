use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use anyhow::Context;
use cascade::cascade;
use chrono::DateTime;
use tokio::sync::mpsc::{Receiver, Sender};
use glib::clone;
use gtk4 as gtk;
use gtk4::{CssProvider, gdk};
use gtk4::gdk::Display;
use gtk::prelude::*;
use libdmd::config::Config;
use libdmd::{dir, fi};
use relm4_macros::view;
use tokio::sync::mpsc::channel;
use crate::models::list::List;
use crate::models::task::{Task, TaskImportance, TaskStatus, ToDoTask};
use crate::token::Requester;
use crate::ui::base::BaseWidgets;
use libdmd::element::Element;
use libdmd::format::{ElementFormat, FileType};

mod models;
mod ui;
mod token;

const CODE: &str = "M.R3_BAY.224d7a8e-0b4d-9fb3-8858-7910958ad435";

#[derive(Debug)]
enum UiEvent {
    Fetch,
    Login,
    ListSelected(usize),
}

#[derive(Debug)]
enum DataEvent {
    UpdateTasks(Vec<ToDoTask>),
    UpdateLists(Vec<List>),
}

fn main() -> anyhow::Result<()> {
    if !Requester::is_token_present(){
        Config::new("ToDo")
            .about("Microsoft To Do Client")
            .author("Eduardo Flores")
            .version("0.1.0")
            .add(dir!("config").child(fi!("config.toml")))
            .write()?;
    }
    let application = gtk::Application::builder()
        .application_id("com.edfloreshz.github")
        .build();
    let (ui_event_sender, mut ui_event_receiver) = channel(1);
    let (data_event_sender, data_event_receiver) = channel(1);
    std::thread::spawn(move || {
        use tokio::runtime::Runtime;
        let rt = Runtime::new().expect("create tokio runtime");
        rt.block_on(async {
            if Requester::is_token_present() {
                let config = Config::get::<Requester>("ToDo/config/config.toml", FileType::TOML).with_context(|| "Failed to get settings.").unwrap();
                let rq = Requester::refresh_token(config.refresh_token.as_str()).await.unwrap();
                Config::set::<Requester>("ToDo/config/config.toml", rq, FileType::TOML).unwrap();
            } else {
                let rq = Requester::token(CODE).await.unwrap();
                Config::set::<Requester>("ToDo/config/config.toml", rq, FileType::TOML).unwrap();
            }

            while let Some(event) = ui_event_receiver.recv().await {
                println!("got event: {:?}", event);
                match event {
                    UiEvent::ListSelected(index) => {
                        let lists = Requester::get_lists().await.unwrap();
                        data_event_sender
                            .send(DataEvent::UpdateTasks(Requester::get_task(&lists[index].clone().task_list_id).await.unwrap()))
                            .await
                            .expect("send refresh result")
                    },
                    UiEvent::Fetch => {
                        let lists = Requester::get_lists().await.unwrap();

                        data_event_sender
                            .send(DataEvent::UpdateLists(lists.clone()))
                            .await
                            .expect("send refresh result")
                    }
                    UiEvent::Login => {
                        println!("Logging in...");

                    }
                }
            }
        })
    });
    let data_event_receiver = Rc::new(RefCell::new(Some(data_event_receiver)));
    let ui_event_sender = Rc::new(RefCell::new(ui_event_sender));
    application.connect_activate(move |app| {
        build_ui(app, ui_event_sender.clone(), data_event_receiver.clone())
    });
    application.run();
    Ok(())
}

fn build_ui(
    application: &gtk::Application,
    ui_event_sender: Rc<RefCell<tokio::sync::mpsc::Sender<UiEvent>>>,
    data_event_receiver: Rc<RefCell<Option<tokio::sync::mpsc::Receiver<DataEvent>>>>,
) {
    view! {
        window = &gtk::ApplicationWindow {
            set_application: Some(application),
            set_default_width: 600,
            set_default_height: 700,
            set_width_request: 600,
            set_height_request: 700,
        }
    }
    let provider = cascade! {
        CssProvider::new();
        ..load_from_data(include_bytes!("ui/style.css"));
    };
    gtk4::StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    ui_event_sender.borrow_mut().try_send(UiEvent::Fetch).expect("Send UI event");
    let widgets = BaseWidgets::new(&window);
    let closure_widgets = widgets.clone();
    let uie = ui_event_sender.clone();
    widgets.login_button.connect_clicked(move |_| {
        widgets.login_dialog.show();
        uie.borrow_mut().try_send(UiEvent::Login).expect("Failed to login.")
    });
    let future = {
        let mut data_event_receiver = data_event_receiver
            .replace(None)
            .take()
            .expect("data_event_reciver");
        async move {
            while let Some(event) = data_event_receiver.recv().await {
                println!("data event: {:?}", event);
                match event {
                    DataEvent::UpdateLists(lists) => {
                        println!("{:#?}", lists);
                        fill_lists(&closure_widgets, &lists);
                    },
                    DataEvent::UpdateTasks(tasks) => {
                        println!("{:#?}", tasks);
                        fill_tasks(&closure_widgets, &tasks.iter().map(|task| {
                            Task {
                                id: task.id.clone(),
                                importance: TaskImportance::from(task.importance.as_str()),
                                is_reminder_on: task.is_reminder_on,
                                status: TaskStatus::from(task.status.as_str()),
                                title: task.title.clone(),
                                created: DateTime::from_str(task.created.as_str()).unwrap(),
                                last_modified: DateTime::from_str(task.last_modified.as_str()).unwrap(),
                                completed: false
                            }
                        }).collect());
                    },
                }
            }
        }
    };

    let c = glib::MainContext::default();
    c.spawn_local(future);
    widgets.sidebar.list.connect_row_activated(move |listbox, row| {
            let index = listbox.selected_row().unwrap().index() as usize;
            ui_event_sender.borrow_mut().try_send(UiEvent::ListSelected(index))
                .expect("Send UI event");
    });
    window.show();
}

pub fn fill_lists(ui: &BaseWidgets, data: &Vec<List>) {
    for list in data.iter() {
        view! {
            label = &gtk::Label {
                set_text: list.display_name.as_str()
            }
        }
        ui.sidebar.list.append(&label);
    }
}

pub fn fill_tasks(ui: &BaseWidgets, task_list: &Vec<Task>) {
    ui.content.remove(&ui.content.last_child().unwrap());
    view! {
        container = &gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_hexpand: true,
            set_margin_bottom: 12,
            set_spacing: 12,

            append = &gtk::ScrolledWindow {
                set_hscrollbar_policy: gtk::PolicyType::Never,
                set_min_content_height: 360,
                set_vexpand: true,
                set_child: tasks = Some(&gtk::ListBox) {
                }
            },
            append: entry = &gtk::Entry {

            }
        }
    }
    for task in task_list {
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .vexpand(true)
            .build();
        let checkbox = gtk::CheckButton::builder().active(false).build();
        let label = gtk::Label::new(Some(&task.title));

        assert!(!task.completed);

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

        checkbox.connect_toggled(move |checkbox| {
            // send!(sender, TaskMsg::SetCompleted((index, checkbox.is_active())));
        });
        tasks.append(&container);
    }
    entry.connect_activate(move |entry| {
        let buffer = entry.buffer();
        buffer.delete_text(0, None);
    });
    ui.content.add_titled(&container, Some("tasks"), "tasks");
}