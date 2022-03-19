use adw::prelude::AdwApplicationWindowExt;
use gtk::prelude::{
    BoxExt, EntryBufferExtManual, EntryExt, OrientableExt, WidgetExt,
};
use relm4::factory::FactoryVec;
use relm4::{adw, gtk, send, WidgetPlus, Widgets, RelmApp, Model};
use tasks_view::task::{Task, TaskMsg};

mod tasks_view;
mod list_view;

pub struct AppModel {
    pub show_panel: bool,
}

enum AppMsg {
    ShowPanel(bool),
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    update(&mut self, event: AppMsg) {
        match event {
            AppMsg::ShowPanel(show_panel) => self.show_panel = show_panel,
        }
    }
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

                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_width_request:  200,
                        // TODO: Implement TreeView
                    },

                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_margin_all: 12,
                        set_spacing: 6,
                        set_hexpand: true,
                        

                        append = &gtk::ScrolledWindow {
                            set_hscrollbar_policy: gtk::PolicyType::Never,
                            set_min_content_height: 360,
                            set_vexpand: true,
                            set_child = Some(&gtk::ListBox) {
                                factory!(model.tasks),
                            }
                        },
        
                        append = &gtk::Entry {
                            
                            connect_activate(sender) => move |entry| {
                                let buffer = entry.buffer();
                                send!(sender, TaskMsg::AddEntry(buffer.text()));
                                buffer.delete_text(0, None);
                            }
                        },
                    },
                },
            },
        }
    }
}

fn main() {
    let model = AppModel {
        tasks: FactoryVec::new(),
    };
    let relm = RelmApp::new(model);
    relm.run();
}
