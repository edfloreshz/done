use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use chrono::DateTime;
use tokio::sync::mpsc::{Receiver, Sender};
use glib::clone;
use gtk4 as gtk;
use gtk::prelude::*;
use relm4_macros::view;
use tokio::sync::mpsc::channel;
use crate::models::list::List;
use crate::models::task::{Task, TaskImportance, TaskStatus, ToDoTask};
use crate::token::Requester;
use crate::ui::base::BaseWidgets;
use crate::ui::selection_row::ListBoxSelectionRow;

// use crate::models::list::{List, ListModel};
// use crate::models::task::{Task, TaskImportance, TaskModel, TaskStatus};

mod models;
mod ui;
mod token;
// mod msft;

const CODE: &str = "M.R3_BAY.bf9e2adf-6821-c184-5ee1-21ad18572937";

// #[tracker::track]
// pub struct AppModel {
//     pub selected: usize,
//     #[tracker::do_not_track]
//     pub lists: Vec<List>,
//     #[tracker::do_not_track]
//     pub task: MicroComponent<TaskModel>,
//     #[tracker::do_not_track]
//     pub refresh_token: String
// }
//
// pub enum AppMsg {
//     Select(usize),
// }
//
// impl Model for AppModel {
//     type Msg = AppMsg;
//     type Widgets = AppWidgets;
//     type Components = AppComponents;
// }
//
// pub struct AppComponents {
//     lists: RelmComponent<ListModel, AppModel>
// }
//
// impl relm4::Components<AppModel> for AppComponents {
//     fn init_components(parent_model: &AppModel, parent_sender: Sender<AppMsg>) -> Self {
//         AppComponents {
//             lists: RelmComponent::new(parent_model, parent_sender)
//         }
//     }
//
//     fn connect_parent(&mut self, _parent_widgets: &AppWidgets) {
//     }
// }
//
// impl AppUpdate for AppModel {
//     fn update(&mut self, msg: Self::Msg, _components: &Self::Components, _sender: Sender<Self::Msg>) -> bool {
//         self.reset();
//         match msg {
//             AppMsg::Select(index) => {
//                 let rq = Requester::refresh_token_blocking(self.refresh_token.clone().as_str()).unwrap();
//                 self.set_selected(index);
//                 let tasks = rq.get_task_blocking(self.lists[self.selected].task_list_id.as_str()).unwrap().iter().map(|task| {
//                     Task {
//                         id: task.id.clone(),
//                         importance: TaskImportance::from(task.importance.as_str()),
//                         is_reminder_on: task.is_reminder_on,
//                         status: TaskStatus::from(task.status.as_str()),
//                         title: task.title.clone(),
//                         created: DateTime::from_str(task.created.as_str()).unwrap(),
//                         last_modified: DateTime::from_str(task.last_modified.as_str()).unwrap(),
//                         completed: false
//                     }
//                 }).collect();
//                 self.task = MicroComponent::new(
//                     TaskModel {
//                         tasks: FactoryVec::from_vec(tasks)
//                     },
//                     ()
//                 );
//
//             },
//         }
//         true
//     }
// }
//
// #[relm4::widget(pub)]
// impl Widgets<AppModel, ()> for AppWidgets {
//
//     fn pre_view() {
//         match self.content.last_child() {
//             Some(last) => {
//                 self.content.remove(&last);
//             },
//             None => {}
//         }
//         if !model.task.is_connected() {
//             self.content.append(model.task.root_widget());
//         }
//     }
// }

#[derive(Debug)]
enum UiEvent {
    Fetch,
    ListSelected(usize),
}

#[derive(Debug)]
enum DataEvent {
    UpdateTasks(Vec<ToDoTask>),
    UpdateLists(Vec<List>),
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
    ui_event_sender.borrow_mut().try_send(UiEvent::Fetch).expect("Send UI event");
    let widgets = BaseWidgets::new(&window);
    let closure_widgets = widgets.clone();
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

fn main() -> anyhow::Result<()> {
    // let model = AppModel {
    //     selected: 0,
    //     lists: rq.get_lists_blocking()?,
    //     tracker: 0,
    //     task: MicroComponent::new(TaskModel { tasks: FactoryVec::new() }, ()),
    //     refresh_token: rq.refresh_token
    // };
    let application = gtk::Application::builder()
        .application_id("com.edfloreshz.github")
        .build();
    let (ui_event_sender, mut ui_event_receiver) = channel(1);
    let (mut data_event_sender, data_event_receiver) = channel(1);
    std::thread::spawn(move || {
        use tokio::runtime::Runtime;
        let mut rt = Runtime::new().expect("create tokio runtime");
        rt.block_on(async {
            let rq = Requester::token(CODE).await.unwrap();
            let lists = rq.get_lists().await.unwrap();

            while let Some(event) = ui_event_receiver.recv().await {
                println!("got event: {:?}", event);
                match event {
                    UiEvent::ListSelected(index) => {
                        data_event_sender
                            .send(DataEvent::UpdateTasks(rq.get_task(&lists[index].clone().task_list_id).await.unwrap()))
                            .await
                            .expect("send refresh result")
                    },
                    UiEvent::Fetch => {
                        data_event_sender
                            .send(DataEvent::UpdateLists(lists.clone()))
                            .await
                            .expect("send refresh result")
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
            set_spacing: 6,

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