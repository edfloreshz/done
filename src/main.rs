use adw::prelude::AdwApplicationWindowExt;
use gtk4::{CellRendererText, TreeStore, TreeView, TreeViewColumn, Widget};
use gtk4::prelude::{StaticType, TreeViewExt};
use gtk::prelude::{
    BoxExt, OrientableExt, WidgetExt, GtkWindowExt, ButtonExt
};
use relm4::{adw, gtk, send, Widgets, RelmApp, Model, AppUpdate, RelmComponent};
use relm4::factory::FactoryView;
use crate::adw::glib::Sender;
use crate::list_view::list::ListsModel;
use crate::tasks_view::tasks::TasksModel;

mod tasks_view;
mod list_view;

#[tracker::track]
pub struct AppModel {
    pub show_panel: bool,
}

pub enum AppMsg {
    ShowPanel,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: Self::Msg, _components: &Self::Components, _sender: Sender<Self::Msg>) -> bool {
        self.reset();

        match msg {
            AppMsg::ShowPanel => self.set_show_panel(!self.show_panel),
        }
        true
    }
}

#[derive(relm4::Components)]
pub struct AppComponents {
    lists: RelmComponent<ListsModel, AppModel>,
    tasks: RelmComponent<TasksModel, AppModel>,
}

#[relm4::widget(pub)]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = adw::ApplicationWindow {
            set_width_request: 660,
            set_height_request: 660,

            set_content: main_box = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                append = &adw::HeaderBar {
                    set_title_widget = Some(&gtk::Label) {
                        set_label: "Tasker",
                    },
                    set_show_start_title_buttons: true,
                    pack_start = &gtk::ToggleButton {
                        set_icon_name: "home",
                        connect_clicked(sender) => move |_| {
                            send!(sender, AppMsg::ShowPanel)
                        }
                    },
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    append: components.lists.root_widget(),
                    append: components.tasks.root_widget(),
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,

                    append: flap = &adw::Flap {
                        set_reveal_flap: track!(model.changed(AppModel::show_panel()), model.show_panel),
                        set_vexpand: true,
                        set_width_request: 200,

                        set_flap = Some(&gtk::StackSidebar) {
                            set_stack = &gtk::Stack {
                                // TODO: Append lists
                            }
                        }
                    },
                }
            },
        }
    }
}

fn main() {
    let model = AppModel {
        show_panel: false,
        tracker: 0,
    };
    let relm = RelmApp::new(model);
    relm.run();
}

