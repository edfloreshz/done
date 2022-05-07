// use once_cell::sync::OnceCell;
// use tokio::runtime::Runtime;
use glib::clone;
use relm4::{
    send,
    adw,
    adw::prelude::AdwApplicationWindowExt,
    gtk,
    gtk::prelude::{GtkWindowExt, WidgetExt, BoxExt, OrientableExt, ButtonExt, EntryExt, PopoverExt, EntryBufferExtManual},
    AppUpdate, Components, Model, RelmComponent, Sender, Widgets,
};
use crate::widgets::sidebar::SidebarMsg;

use crate::widgets::details::DetailsModel;
use crate::widgets::sidebar::SidebarModel;

// static RT: OnceCell<Runtime> = OnceCell::new();

pub struct AppModel;

impl AppModel {
    pub fn new() -> Self {
        Self {}
    }
}

pub enum AppMsg {
    AddList(String),
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for AppModel {
    fn update(
        &mut self,
        msg: Self::Msg,
        components: &Self::Components,
        _sender: Sender<Self::Msg>,
    ) -> bool {
        match msg {
            AppMsg::AddList(title) => {
                components.sidebar.send(SidebarMsg::AddList(title)).unwrap();
            }
        }
        true
    }
}

pub struct AppComponents {
    sidebar: RelmComponent<SidebarModel, AppModel>,
    details: RelmComponent<DetailsModel, AppModel>,
}

impl Components<AppModel> for AppComponents {
    fn init_components(parent_model: &AppModel, parent_sender: Sender<AppMsg>) -> Self {
        AppComponents {
            sidebar: RelmComponent::new(parent_model, parent_sender.clone()),
            details: RelmComponent::new(parent_model, parent_sender),
        }
    }

    fn connect_parent(&mut self, _parent_widgets: &AppWidgets) {}
}

#[relm4_macros::widget(pub)]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        window = adw::ApplicationWindow {
            set_default_width: 800,
            set_default_height: 700,
            set_width_request: 460,
            set_height_request: 700,

            set_content = Some(&adw::Leaflet) {
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    append: headerbar_box = &gtk::Box {
                        set_hexpand: true,
                        append: headerbox = &adw::HeaderBar {
                            set_hexpand: true,
                            set_title_widget = Some(&gtk::Label) {
                                set_text: "Done"
                            },
                            pack_start: &gtk::Image::from_icon_name("dev.edfloreshz.Done"),
                            pack_start: new_list_button = &gtk::MenuButton {
                                set_label: "New List",
                                add_css_class: "raised",
                                set_has_frame: true,
                                set_direction: gtk::ArrowType::Down,
                                set_popover: new_list_popover = Some(&gtk::Popover) {
                                    set_child = Some(&gtk::Stack) {
                                        add_child = &gtk::Box {
                                            set_orientation: gtk::Orientation::Vertical,
                                            set_spacing: 10,
                                            append: &gtk::Label::new(Some("List Name")),
                                            append: new_list_entry = &gtk::Entry {
                                                connect_activate(sender) => move |entry| {
                                                    let buffer = entry.buffer();
                                                    if !buffer.text().is_empty() {
                                                        send!(sender, AppMsg::AddList(buffer.text()))
                                                    }
                                                }
                                            },
                                            append: add_button = &gtk::Button {
                                                set_label: "Create List",
                                                set_css_classes: &["suggested-action"],
                                                connect_clicked: clone!(@weak new_list_entry, @strong sender => move |_| {
                                                    let buffer = new_list_entry.buffer();
                                                    if !buffer.text().is_empty() {
                                                        send!(sender, AppMsg::AddList(buffer.text()))
                                                    }
                                                })
                                            },
                                        }
                                    }
                                }
                            },
                            pack_start: toggle_sidebar_button = &gtk::ToggleButton {
                                set_icon_name: "sidebar-show-symbolic",
                                add_css_class: "raised"
                            },
                        }
                    },
                    append: overlay = &gtk::Overlay {
                        set_child: stack = Some(&gtk::Stack) {
                            set_hexpand: true,
                            set_vexpand: true,
                            set_transition_duration: 250,
                            set_transition_type: gtk::StackTransitionType::Crossfade,
                            add_child: &components.sidebar.widgets().unwrap().leaflet
                        }
                    },
                }
            }
        }
    }
}
