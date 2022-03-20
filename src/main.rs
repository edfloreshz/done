use adw::prelude::AdwApplicationWindowExt;
use gtk::prelude::{
    BoxExt, OrientableExt, WidgetExt,
};
use relm4::{adw, gtk, Widgets, RelmApp, Model, AppUpdate, RelmComponent};
use crate::adw::glib::Sender;
use crate::list_view::list::ListsModel;
use crate::tasks_view::tasks::TasksModel;

mod tasks_view;
mod list_view;

pub struct AppModel {
    pub show_panel: bool,
}

pub enum AppMsg {
    ShowPanel(bool),
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: Self::Msg, components: &Self::Components, sender: Sender<Self::Msg>) -> bool {
        match msg {
            AppMsg::ShowPanel(show_panel) => self.show_panel = show_panel,
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
            set_width_request: 360,

            set_content: main_box = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                append = &adw::HeaderBar {
                    set_title_widget = Some(&gtk::Label) {
                        set_label: "Tasker",
                    },
                    set_show_start_title_buttons: true
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    append: components.lists.root_widget(),
                    append: components.tasks.root_widget(),
                }
            }
        }
    }
}

fn main() {
    let model = AppModel {
        show_panel: false,
    };
    let relm = RelmApp::new(model);
    relm.run();
}
