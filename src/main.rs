use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use cascade::cascade;
use libadwaita as adw;
use gtk4 as gtk;
use gtk4::CssProvider;
use gtk4::gdk::Display;
use gtk::prelude::*;
use libdmd::{config::Config, dir, element::Element, fi, format::{ElementFormat, FileType}};

use relm4_macros::view;
use tokio::sync::mpsc::{channel, Sender, Receiver};

use crate::models::list::List;
use crate::models::task::{Task, TaskImportance, TaskStatus};
use crate::token::Requester;
use crate::ui::base::BaseWidgets;

mod models;
mod token;
mod ui;

const CODE: &str = "M.R3_BAY.80886bc9-72a6-ed9a-4006-e652d4a42dfb";

#[derive(Debug)]
enum UiEvent {
    Fetch,
    Login,
    ListSelected(usize),
}

#[derive(Debug)]
enum DataEvent {
    UpdateTasks(Vec<Task>),
    UpdateLists(Vec<List>),
}

fn main() -> anyhow::Result<()> {
    if !Requester::is_token_present() {
        Config::new("ToDo")
            .about("Microsoft To Do Client")
            .author("Eduardo Flores")
            .version("0.1.0")
            .add(dir!("config").child(fi!("config.toml")))
            .write()?;
    }
    let application = adw::Application::builder()
        .application_id("com.edfloreshz.github")
        .build();
    let (ui_sender, ui_recv): (Sender<UiEvent>, Receiver<UiEvent>) = channel(1);
    let (data_sender, data_recv): (Sender<DataEvent>, Receiver<DataEvent>) = channel(1);
    let data_recv = Rc::new(RefCell::new(Some(data_recv)));
    let data_sender = Arc::new(Mutex::new(data_sender));
    let ui_sender = Rc::new(RefCell::new(ui_sender));
    let ui_recv = Arc::new(Mutex::new(ui_recv));
    handle_events(ui_recv.clone(), data_sender.clone());
    application.connect_activate(move |app| {
        build_ui(app, ui_sender.clone(), data_recv.clone())
    });
    application.run();
    Ok(())
}

fn handle_events(ui_recv: Arc<Mutex<tokio::sync::mpsc::Receiver<UiEvent>>>, data_sender: Arc<Mutex<tokio::sync::mpsc::Sender<DataEvent>>>) {
    std::thread::spawn(move || {
        use tokio::runtime::Runtime;
        let rt = Runtime::new().expect("create tokio runtime");
        rt.block_on(async {
            if Requester::is_token_present() {
                let config = Requester::current_config().unwrap();
                let rq = Requester::refresh_token(config.refresh_token.as_str())
                    .await
                    .unwrap();
                Config::set::<Requester>("ToDo/config/config.toml", rq, FileType::TOML).unwrap();
            } else {
                let rq = Requester::token(CODE).await.unwrap();
                Config::set::<Requester>("ToDo/config/config.toml", rq, FileType::TOML).unwrap();
            }
            let ui_recv = ui_recv.clone();
            let mut ui_recv = ui_recv.lock().unwrap();
            while let Some(event) = ui_recv.recv().await {
                println!("got event: {:?}", event);
                match event {
                    UiEvent::ListSelected(index) => {
                        let lists: Vec<crate::List> = Requester::get_lists().await.unwrap();
                        let task_id = lists[index.clone()].clone().task_list_id;
                        data_sender.clone().lock().unwrap()
                            .send(DataEvent::UpdateTasks(
                                Requester::get_task(task_id.as_str())
                                    .await
                                    .unwrap(),
                            ))
                            .await
                            .expect("send refresh result")
                    }
                    UiEvent::Fetch => {
                        let lists = Requester::get_lists().await.unwrap();

                        data_sender.clone().lock().unwrap()
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
    let uie = ui_event_sender.clone();
    widgets.login_button.connect_clicked(move |_| {
        widgets.login_dialog.show();
        uie.borrow_mut()
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
                        fill_lists(&closure_widgets, &lists);
                    }
                    DataEvent::UpdateTasks(tasks) => {
                        println!("{:#?}", tasks);
                        fill_tasks(
                            &closure_widgets,
                            &tasks,
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

pub fn fill_lists(ui: &BaseWidgets, data: &Vec<List>) {
    for list in data.iter() {
        view! {
            label = &gtk::Label {
                set_text: list.display_name.as_str(),
                set_height_request: 40,
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
    for task in task_list {
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        let checkbox = gtk::CheckButton::builder().active(false).build();
        let label = gtk::Label::builder().label(&task.title).build();

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

        checkbox.connect_toggled(move |_| {
            // send!(sender, TaskMsg::SetCompleted((index, checkbox.is_active())));
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
