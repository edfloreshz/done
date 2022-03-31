use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use cascade::cascade;
use gtk4 as gtk;
use gtk::CssProvider;
use gtk::gdk::Display;
use gtk::prelude::*;
use libadwaita as adw;
use libdmd::{
    config::Config,
    dir,
    element::Element,
    fi,
    format::ElementFormat,
};
use relm4_macros::view;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::models::list::List;
use crate::services::microsoft::task::Task;
use crate::services::microsoft::token::MicrosoftTokenAccess;
use crate::services::ToDoService;
use crate::ui::base::BaseWidgets;

mod models;
mod services;
mod ui;

#[derive(Debug)]
pub enum UiEvent {
    Fetch,
    Login,
    AddEntry(String, String),
    ListSelected(usize),
    TaskCompleted(String, String, bool),
    TaskSelected(String, String),
}

#[derive(Debug)]
enum DataEvent {
    Login,
    UpdateTasks(String, Vec<Task>),
    UpdateLists(Vec<List>),
    UpdateDetails(String, Task),
}

#[derive(Clone)]
struct EventHandler {
    ui_tx: Rc<RefCell<Sender<UiEvent>>>,
    ui_rv: Arc<Mutex<Receiver<UiEvent>>>,
    data_tx: Arc<Mutex<Sender<DataEvent>>>,
    data_rv: Rc<RefCell<Option<Receiver<DataEvent>>>>,
}

fn main() -> anyhow::Result<()> {
    if !MicrosoftTokenAccess::is_token_present() {
        let mut config = Config::new("do")
            .about("Microsoft To Do Client")
            .author("Eduardo Flores")
            .version("0.1.0")
            .add(dir!("config").child(fi!("config.toml")))
            .write()?;
        MicrosoftTokenAccess::create_config(&mut config)?;
    }
    let application = adw::Application::builder()
        .application_id("do.edfloreshz.github")
        .build();
    let (ui_sender, ui_recv): (Sender<UiEvent>, Receiver<UiEvent>) = channel(1);
    let (data_sender, data_recv): (Sender<DataEvent>, Receiver<DataEvent>) = channel(1);
    let event_handler = EventHandler {
        ui_tx: Rc::new(RefCell::new(ui_sender)),
        ui_rv: Arc::new(Mutex::new(ui_recv)),
        data_tx: Arc::new(Mutex::new(data_sender)),
        data_rv: Rc::new(RefCell::new(Some(data_recv))),
    };
    handle_events(event_handler.clone());
    application.connect_activate(move |app| build_ui(app, event_handler.clone()));
    application.run();
    Ok(())
}

fn handle_events(event_handler: EventHandler) {
    std::thread::spawn(move || {
        use tokio::runtime::Runtime;
        let rt = Runtime::new().expect("create tokio runtime");
        rt.block_on(async {
            let (ui_recv, data_tx) = (event_handler.ui_rv.clone(), event_handler.data_tx.clone());
            let mut ui_recv = ui_recv.lock().unwrap();
            let data_tx = data_tx.lock().unwrap();
            while let Some(event) = ui_recv.recv().await {
                println!("Received UI event: {:?}", event);
                match event {
                    UiEvent::ListSelected(index) => match MicrosoftTokenAccess::get_lists().await {
                        Ok(lists) => {
                            let task_list_id = lists[index.clone()].clone().task_list_id;
                            match MicrosoftTokenAccess::get_tasks(task_list_id.as_str()).await {
                                Ok(tasks) => data_tx
                                    .send(DataEvent::UpdateTasks(task_list_id.clone(), tasks))
                                    .await
                                    .expect("Failed to send UpdateTasks event."),
                                Err(err) => println!("{err}"),
                            }
                        }
                        Err(err) => println!("{err}"),
                    },
                    UiEvent::Fetch => match MicrosoftTokenAccess::get_lists().await {
                        Ok(lists) => data_tx
                            .send(DataEvent::UpdateLists(lists))
                            .await
                            .expect("Failed to send UpdateLists event."),
                        Err(err) => println!("{err}"),
                    },
                    UiEvent::TaskCompleted(task_list_id, task_id, completed) => {
                        // TODO: When a task is completed in the details view it needs to be updated in the list view.
                        match MicrosoftTokenAccess::set_task_as_completed(
                            task_list_id.as_str(),
                            task_id.as_str(),
                            completed,
                        )
                        .await
                        {
                            Ok(_) => {}
                            Err(err) => println!("{err}"),
                        }
                    }
                    UiEvent::Login => {
                        if MicrosoftTokenAccess::is_token_present() {
                            match MicrosoftTokenAccess::current_token_data() {
                                None => println!("Couldn't find current token data"),
                                Some(config) => {
                                    match MicrosoftTokenAccess::refresh_token(config.refresh_token.as_str()).await {
                                        Ok(token) => {
                                            match MicrosoftTokenAccess::update_token_data(&token) {
                                                Ok(_) => println!("Token configuration updated."),
                                                Err(err) => println!("{err}")
                                            }
                                        }
                                        Err(err) => println!("{err}")
                                    }
                                }
                            };
                        } else {
                            match MicrosoftTokenAccess::authenticate().await {
                                Ok(code) => {
                                    match MicrosoftTokenAccess::token(code).await {
                                        Ok(token_data) => {
                                            match MicrosoftTokenAccess::update_token_data(&token_data) {
                                                Ok(_) => {
                                                    match MicrosoftTokenAccess::get_lists().await {
                                                        Ok(lists) => {
                                                            data_tx.send(DataEvent::Login).await.expect("Failed to send Login event.");
                                                            data_tx.send(DataEvent::UpdateLists(lists)).await.expect("Failed to send Login event.");
                                                        }
                                                        Err(err) => println!("{err}")
                                                    }
                                                    println!("Updated token data.");
                                                },
                                                Err(err) => println!("{err}")
                                            }
                                        }
                                        Err(err) => println!("{err}")
                                    }
                                }
                                Err(err) => println!("{err}")
                            }
                        }
                    }
                    UiEvent::TaskSelected(task_list_id, task_id) => {
                        match MicrosoftTokenAccess::get_task(&*task_list_id, &*task_id).await {
                            Ok(task) => {
                                data_tx
                                    .send(DataEvent::UpdateDetails(task_list_id, task))
                                    .await
                                    .expect("Failed to send UpdateLists event.");
                            }
                            Err(err) => println!("{err}"),
                        }
                    }
                    UiEvent::AddEntry(entry, task_list_id) => {
                        match MicrosoftTokenAccess::push_task(&*task_list_id.clone(), entry).await {
                            Ok(_) => {
                                match MicrosoftTokenAccess::get_tasks(task_list_id.as_str()).await {
                                    Ok(tasks) => data_tx
                                        .send(DataEvent::UpdateTasks(task_list_id.clone(), tasks))
                                        .await
                                        .expect("Failed to send UpdateTasks event."),
                                    Err(err) => println!("{err}"),
                                }
                            }
                            Err(err) => println!("{err}"),
                        }
                    }
                }
            }
        })
    });
}

fn build_ui(application: &adw::Application, event_handler: EventHandler) {
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
    if MicrosoftTokenAccess::is_token_present() {
        event_handler
            .ui_tx
            .borrow_mut()
            .try_send(UiEvent::Fetch)
            .expect("Send UI event");
    }

    let widgets = BaseWidgets::new(&window);
    let closure_widgets = widgets.clone();
    let login_tx = event_handler.ui_tx.clone();
    let ui_tx = event_handler.ui_tx.clone();
    widgets.login_button.connect_clicked(move |_| {
        login_tx
            .borrow_mut()
            .try_send(UiEvent::Login)
            .expect("Failed to login.")
    });
    let future = {
        let mut data_event_receiver = event_handler
            .data_rv
            .replace(None)
            .take()
            .expect("data_event_receiver");
        async move {
            while let Some(event) = data_event_receiver.recv().await {
                println!("Received data event: {:?}", event);
                match event {
                    DataEvent::UpdateLists(lists) => {
                        List::fill_lists(&closure_widgets, &lists);
                    }
                    DataEvent::UpdateTasks(task_list_id, tasks) => {
                        Task::fill_tasks(&closure_widgets, task_list_id, &tasks, ui_tx.clone());
                    }
                    DataEvent::UpdateDetails(task_list_id, task) => {
                        Task::fill_details(&closure_widgets, task_list_id, task, ui_tx.clone())
                    }
                    DataEvent::Login => {
                        closure_widgets.update_welcome();
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
            event_handler
                .ui_tx
                .borrow_mut()
                .try_send(UiEvent::ListSelected(index))
                .expect("Send UI event");
        });
    window.show();
}
