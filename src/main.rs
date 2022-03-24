use adw::prelude::AdwApplicationWindowExt;
use gtk4::glib;
use gtk4::glib::Type;
use gtk4::prelude::TreeViewExt;
use gtk::prelude::{
    BoxExt, ButtonExt, GridExt, WidgetExt, OrientableExt, GtkWindowExt, EntryExt, EntryBufferExtManual
};
use relm4::{adw, AppUpdate, Components, ComponentUpdate, gtk, MicroComponent, Model, RelmApp, RelmComponent, send, WidgetPlus, Widgets};
use relm4::factory::FactoryVec;

use models::task::Task;

use crate::adw::glib::Sender;
use crate::glib::StaticType;
use crate::models::list::{List, ListModel};
use crate::models::task::TaskModel;

mod models;
mod views;

#[tracker::track]
pub struct AppModel {
    pub selected: usize,
    #[tracker::do_not_track]
    pub lists: Vec<List>,
    #[tracker::do_not_track]
    pub task: MicroComponent<TaskModel>
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

impl Components<AppModel> for AppComponents {
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
                self.set_selected(index);
                self.task = MicroComponent::new(
                    TaskModel {
                        tasks: FactoryVec::from_vec(self.lists[self.selected].clone().tasks)
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

fn main() {
    let model = AppModel {
        selected: 0,
        lists: vec![
            List {
                name: "Shopping 🛍️".into(),
                tasks: vec![
                    Task {
                        name: "Eggs 🥚".into(),
                        completed: false
                    }
                ]
            },
            List {
                name: "Pending 😟".into(),
                tasks: vec![
                    Task {
                        name: "Pay bills 💸".into(),
                        completed: false
                    }
                ]
            }
        ],
        tracker: 0,
        task: MicroComponent::new(TaskModel { tasks: FactoryVec::new() }, ())
    };
    let relm = RelmApp::new(model);
    relm.run();
}

// pub struct AppWidgets {
//     window: adw::ApplicationWindow,
//     tree_view: gtk::TreeView,
//     column: gtk::TreeViewColumn,
// }
//
// impl Widgets<AppModel, ()> for AppWidgets {
//     type Root = adw::ApplicationWindow;
//
//     fn init_view(model: &AppModel, _components: &(), sender: Sender<AppMsg>) -> Self {
//         let window = adw::ApplicationWindow::builder()
//             .title("To Do")
//             .height_request(300)
//             .width_request(500)
//             .build();
//         let tree_view = gtk::TreeView::builder().width_request(200).headers_visible(true).build();
//
//         let column = gtk::TreeViewColumn::builder().title("List Name").build();
//         tree_view.append_column(&column);
//
//         let list_store = gtk::TreeStore::new(&[Type::STRING]);
//         for (i, list) in model.lists.iter().enumerate() {
//             list_store.insert_with_values(None, Some(i as u32), &[(0, &list.name)]);
//         }
//
//         tree_view.set_model(Some(&list_store));
//
//         let main_box = gtk::Box::builder()
//             .orientation(gtk::Orientation::Vertical)
//             .build();
//         let title = gtk::Label::new(Some("ToDoer"));
//         let header = adw::HeaderBar::builder()
//             .title_widget(&title)
//             .show_start_title_buttons(true)
//             .build();
//
//         let toggle_button = gtk::ToggleButton::builder()
//             .icon_name("home")
//             .build();
//         let flap = adw::Flap::builder()
//             .vexpand(true)
//             .locked(true)
//             .modal(true)
//             .swipe_to_open(true)
//             .swipe_to_close(true)
//             .width_request(100)
//             .build();
//         toggle_button.connect_clicked(move |_| {
//             send!(sender, AppMsg::ShowPanel)
//         });
//         header.pack_start(&toggle_button);
//         main_box.append(&header);
//
//         if model.changed(AppModel::show_panel()) {
//             flap.set_reveal_flap(model.show_panel)
//         }
//         flap.set_flap(Some(&tree_view));
//         let container = gtk::Box::new(gtk::Orientation::Horizontal, 6);
//         container.append(&flap);
//         let b = &gtk::Box::builder().width_request(200).build();
//         b.append(&gtk::Label::new(Some("HEllo")));
//         container.append(b);
//         main_box.append(&container);
//         window.set_content(Some(&main_box));
//         Self {
//             window,
//             tree_view,
//             column
//         }
//     }
//
//     fn root_widget(&self) -> Self::Root {
//         self.window.clone()
//     }
//
//     fn view(&mut self, model: &AppModel, _sender: Sender<AppMsg>) {
//         // self.flap.set_reveal_flap(model.show_panel)
//     }
// }
