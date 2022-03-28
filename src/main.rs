use std::sync::mpsc;
use adw::prelude::AdwApplicationWindowExt;
use cascade::cascade;
use gtk::prelude::{
    BoxExt, WidgetExt, OrientableExt, GtkWindowExt
};
use libdmd::config::Config;
use libdmd::{dir, fi, format::ElementFormat, element::Element};
use relm4::{adw, AppUpdate, gtk, MicroComponent, Model, RelmApp, RelmComponent, Widgets};
use relm4::gtk::CssProvider;
use tracker::track;


use crate::models::list::{List, ListModel};
use crate::models::task::{Task, TaskImportance, TaskModel, TaskStatus};

mod models;
mod services;

const TOKEN: &str = "M.R3_BAY.9380a78f-50f7-f43f-f5a7-65ee34feebd0";

#[track]
#[derive(Clone)]
pub struct AppModel {
    pub selected: usize,
    #[do_not_track]
    pub lists: Vec<List>,
    #[do_not_track]
    pub task: TaskModel,
    #[do_not_track]
    pub refresh_token: String,
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
    fn init_components(parent_model: &AppModel, parent_sender: relm4::Sender<AppMsg>) -> Self {
        AppComponents {
            lists: RelmComponent::new(parent_model, parent_sender)
        }
    }

    fn connect_parent(&mut self, _parent_widgets: &AppWidgets) {
    }
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: Self::Msg, _components: &Self::Components, _sender: relm4::Sender<Self::Msg>) -> bool {
        self.reset();
        match msg {
            AppMsg::Select(index) => {
                self.set_selected(index);
                let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
                rt.block_on(async {
                    let tasks = MicrosoftTokenAccess::get_tasks(self.lists[self.selected].task_list_id.as_str()).await.unwrap();
                    self.task = TaskModel {
                        tasks
                    }
                })

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
            set_width_request: 600,
            set_height_request: 700,

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
        let provider = cascade! {
            CssProvider::new();
            ..load_from_data(include_bytes!("ui/style.css"));
        };
        gtk4::StyleContext::add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        match self.content.last_child() {
            Some(last) => {
                self.content.remove(&last);
            },
            None => {}
        }
        let task = MicroComponent::new(model.task.clone(), ());
        if !task.is_connected() {
            self.content.append(task.root_widget());
        }
    }
}

use crate::adw::gdk::Display;
use crate::services::microsoft::MicrosoftTokenAccess;
use crate::services::ToDoService;

fn main() {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        use tokio::runtime::Runtime;
        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async {
            if !MicrosoftTokenAccess::is_token_present() {
                let mut config = Config::new("ToDo")
                    .about("Microsoft To Do Client")
                    .author("Eduardo Flores")
                    .version("0.1.0")
                    .add(dir!("config").child(fi!("config.toml")))
                    .write().unwrap();
                MicrosoftTokenAccess::create_config(&mut config).unwrap();
            }
            let token = MicrosoftTokenAccess::token(TOKEN).await.unwrap();
            let model = AppModel {
                selected: 0,
                lists: MicrosoftTokenAccess::get_lists().await.unwrap(),
                task: TaskModel { tasks: Vec::new() },
                refresh_token: token.refresh_token,
                tracker: 0,
            };
            tx.send(model)
        }).unwrap();
    });
    let relm = RelmApp::new(rx.recv().unwrap());
    relm.run();
}