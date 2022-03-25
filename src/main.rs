use std::str::FromStr;
use std::thread::sleep;
use std::time::SystemTime;
use adw::prelude::AdwApplicationWindowExt;
use chrono::DateTime;
use gtk::prelude::{
    BoxExt, WidgetExt, OrientableExt, GtkWindowExt, EntryBufferExtManual
};
use relm4::{adw, AppUpdate, gtk, MicroComponent, Model, RelmApp, RelmComponent, WidgetPlus, Widgets};
use relm4::factory::FactoryVec;

use crate::adw::glib::Sender;
use crate::models::list::{List, ListModel};
use crate::models::task::{Task, TaskImportance, TaskModel, TaskStatus};
use crate::token::Requester;

mod models;
mod token;
mod msft;

const TOKEN: &str = "M.R3_BAY.41fe1b38-22a3-9176-58ca-e959238fdbec";

#[tracker::track]
pub struct AppModel {
    pub selected: usize,
    #[tracker::do_not_track]
    pub lists: Vec<List>,
    #[tracker::do_not_track]
    pub task: MicroComponent<TaskModel>,
    #[tracker::do_not_track]
    pub refresh_token: String
}

pub enum AppMsg {
    Select(usize),
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

pub struct AppComponents {
    lists: RelmComponent<ListModel, AppModel>
}

impl relm4::Components<AppModel> for AppComponents {
    fn init_components(parent_model: &AppModel, parent_sender: Sender<AppMsg>) -> Self {
        AppComponents {
            lists: RelmComponent::new(parent_model, parent_sender)
        }
    }

    fn connect_parent(&mut self, _parent_widgets: &AppWidgets) {
    }
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: Self::Msg, _components: &Self::Components, _sender: Sender<Self::Msg>) -> bool {
        self.reset();
        match msg {
            AppMsg::Select(index) => {
                let rq = Requester::refresh_token_blocking(self.refresh_token.clone().as_str()).unwrap();
                self.set_selected(index);
                let tasks = rq.get_task_blocking(self.lists[self.selected].task_list_id.as_str()).unwrap().iter().map(|task| {
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
                }).collect();
                self.task = MicroComponent::new(
                    TaskModel {
                        tasks: FactoryVec::from_vec(tasks)
                    },
                    ()
                );

            },
        }
        true
    }
}

#[relm4::widget(pub)]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = &adw::ApplicationWindow {
            set_default_width: 600,
            set_default_height: 700,
            set_width_request: 200,
            set_height_request: 200,

            set_content: main_box = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,

                append: leaflet = &adw::Leaflet {
                    set_can_navigate_back: true,
                    append: side_headerbar = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        append = &adw::HeaderBar {
                            set_width_request: 250,
                            set_show_end_title_buttons: false,
                            set_show_start_title_buttons: false,
                            set_title_widget = Some(&gtk::Label) {
                                set_label: "To Do",
                            },
                        },
                        append = &gtk::ScrolledWindow {
                            set_vexpand: true,
                            set_width_request: 250,
                            set_hscrollbar_policy: gtk::PolicyType::Never,

                            set_child: Some(components.lists.root_widget())
                        }
                    },
                    append = &gtk::Separator {
                        set_orientation: gtk::Orientation::Vertical,
                        set_visible: true
                    },

                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        append = &adw::HeaderBar {
                            set_width_request: 250,
                        },
                        append: content = &gtk::Box { }
                    }
                },
            }
        }
    }
    fn pre_view() {
        match self.content.last_child() {
            Some(last) => {
                self.content.remove(&last);
            },
            None => {}
        }
        if !model.task.is_connected() {
            self.content.append(model.task.root_widget());
        }
    }
}

use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn main() -> anyhow::Result<()> {
    let rq = Requester::token_blocking(TOKEN)?;
    let model = AppModel {
        selected: 0,
        lists: rq.get_lists_blocking()?,
        tracker: 0,
        task: MicroComponent::new(TaskModel { tasks: FactoryVec::new() }, ()),
        refresh_token: rq.refresh_token
    };
    let relm = RelmApp::new(model);
    relm.run();
    Ok(())
}