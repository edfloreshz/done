use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use cascade::cascade;
use libadwaita as adw;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::CssProvider;
use gtk::gdk::Display;
use libdmd::{config::Config, dir, element::Element, fi, format::{ElementFormat, FileType}};

use relm4_macros::view;
use tokio::sync::mpsc::{channel, Sender, Receiver};
use crate::backends::microsoft::MicrosoftTokenAccess;

use crate::models::list::List;
use crate::models::task::{Task, TaskImportance, TaskStatus};
use crate::backends::ToDoService;
use crate::ui::base::BaseWidgets;

mod models;
mod ui;
mod backends;

const CODE: &str = "M.R3_BAY.70f046ae-81aa-20e5-2ddf-eec65ef51a79";

#[derive(Debug)]
pub enum UiEvent {
    Fetch,
    Login,
    ListSelected(usize),
    TaskCompleted(String, String, bool)
}

#[derive(Debug)]
enum DataEvent {
    UpdateTasks(String, Vec<Task>),
    UpdateLists(Vec<List>),
}

fn main() -> anyhow::Result<()> {
    if !MicrosoftTokenAccess::is_token_present() {
        let mut config = Config::new("ToDo")
            .about("Microsoft To Do Client")
            .author("Eduardo Flores")
            .version("0.1.0")
            .add(dir!("config").child(fi!("config.toml")))
            .write()?;
        MicrosoftTokenAccess::create_config(&mut config)?;
    }
    let application = adw::Application::builder()
        .application_id("com.edfloreshz.github")
        .build();
    let (ui_sender, ui_recv): (Sender<UiEvent>, Receiver<UiEvent>) = channel(1);
    let (data_sender, data_recv): (Sender<DataEvent>, Receiver<DataEvent>) = channel(1);
    let data_sender = Arc::new(Mutex::new(data_sender));
    let data_recv = Rc::new(RefCell::new(Some(data_recv)));
    let ui_sender = Rc::new(RefCell::new(ui_sender));
    let ui_recv = Arc::new(Mutex::new(ui_recv));
    handle_events(ui_recv.clone(), data_sender.clone());
    application.connect_activate(move |app| {
        build_ui(app, ui_sender.clone(), data_recv.clone())
    });
    application.run();
    Ok(())
}

fn build_ui(
    application: &adw::Application,
    ui_event_sender: Rc<RefCell<tokio::sync::mpsc::Sender<UiEvent>>>,
    data_event_receiver: Rc<RefCell<Option<tokio::sync::mpsc::Receiver<DataEvent>>>>,
) {
    view! {
        window = &adw::ApplicationWindow {
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
    ui_event_sender
        .borrow_mut()
        .try_send(UiEvent::Fetch)
        .expect("Send UI event");

    let widgets = BaseWidgets::new(&window);
    let closure_widgets = widgets.clone();
    let ui_e_sender = ui_event_sender.clone();
    let ui_e_sender2 = ui_event_sender.clone();
    widgets.login_button.connect_clicked(move |_| {
        widgets.login_dialog.show();
        ui_e_sender.borrow_mut()
            .try_send(UiEvent::Login)
            .expect("Failed to login.")
    });
    let future = {
        let mut data_event_receiver = data_event_receiver
            .replace(None)
            .take()
            .expect("data_event_receiver");
        async move {
            while let Some(event) = data_event_receiver.recv().await {
                println!("data event: {:?}", event);
                match event {
                    DataEvent::UpdateLists(lists) => {
                        println!("{:#?}", lists);
                        List::fill_lists(&closure_widgets, &lists);
                    }
                    DataEvent::UpdateTasks(task_list_id, tasks) => {
                        println!("{:#?}", tasks);
                        Task::fill_tasks(
                            &closure_widgets,
                            task_list_id,
                            &tasks,
                            ui_e_sender2.clone()
                        );
                    }
                }
            }
        }
    };

    let c = glib::MainContext::default();
    c.spawn_local(future);
    widgets
        .sidebar
        .list
        .connect_row_activated(move |listbox, _| {
            let index = listbox.selected_row().unwrap().index() as usize;
            ui_event_sender
                .borrow_mut()
                .try_send(UiEvent::ListSelected(index))
                .expect("Send UI event");
        });
    window.show();
}

fn handle_events(ui_recv: Arc<Mutex<tokio::sync::mpsc::Receiver<UiEvent>>>, data_sender: Arc<Mutex<tokio::sync::mpsc::Sender<DataEvent>>>) {
    std::thread::spawn(move || {
        use tokio::runtime::Runtime;
        let rt = Runtime::new().expect("create tokio runtime");
        rt.block_on(async {
            if MicrosoftTokenAccess::is_token_present() {
                let config = MicrosoftTokenAccess::current_token_data().unwrap();
                let rq = MicrosoftTokenAccess::refresh_token(config.refresh_token.as_str())
                    .await
                    .unwrap();
                Config::set::<MicrosoftTokenAccess>("ToDo/config/config.toml", rq, FileType::TOML).unwrap();
            } else {
                let token_data = MicrosoftTokenAccess::token(CODE).await.unwrap();
                MicrosoftTokenAccess::update_token_data(&token_data).unwrap();
            }
            let ui_recv = ui_recv.clone();
            let mut ui_recv = ui_recv.lock().unwrap();
            while let Some(event) = ui_recv.recv().await {
                println!("got event: {:?}", event);
                match event {
                    UiEvent::ListSelected(index) => {
                        let lists: Vec<crate::List> = MicrosoftTokenAccess::get_lists().await.unwrap();
                        let task_list_id = lists[index.clone()].clone().task_list_id;
                        let task_list_id_2 = lists[index.clone()].clone().task_list_id;
                        data_sender.clone().lock().unwrap()
                            .send(DataEvent::UpdateTasks(
                                task_list_id,
                                MicrosoftTokenAccess::get_tasks(task_list_id_2.as_str())
                                    .await
                                    .unwrap(),
                            ))
                            .await
                            .expect("Failed to send UpdateTasks event.")
                    }
                    UiEvent::Fetch => {
                        let lists = MicrosoftTokenAccess::get_lists().await.unwrap();

                        data_sender.clone().lock().unwrap()
                            .send(DataEvent::UpdateLists(lists.clone()))
                            .await
                            .expect("Failed to send UpdateLists event.")
                    }
                    UiEvent::TaskCompleted(task_list_id, task_id, completed) => {
                        MicrosoftTokenAccess::set_task_as_completed(task_list_id.as_str(), task_id.as_str(), completed).await;
                    }
                    UiEvent::Login => {
                        println!("Logging in...");
                    }
                }
            }
        })
    });
}