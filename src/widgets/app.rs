use once_cell::sync::OnceCell;
use relm4::{
    adw,
    adw::prelude::AdwApplicationWindowExt,
    AppUpdate,
    Components,
    gtk,
    gtk::prelude::{
        BoxExt,
        ButtonExt,
        GtkWindowExt,
        OrientableExt,
        WidgetExt
    },
    Model,
    RelmComponent,
    send,
    Sender,
    WidgetPlus,
    Widgets
};
use tokio::runtime::Runtime;

use crate::widgets::content::ContentModel;
use crate::widgets::details::DetailsModel;
use crate::widgets::sidebar::SidebarModel;

static RT: OnceCell<Runtime> = OnceCell::new();

#[derive(Default)]
pub struct AppModel {}

pub enum AppMsg {
    Login,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: Self::Msg, components: &Self::Components, _sender: Sender<Self::Msg>) -> bool {
        match msg {
            AppMsg::Login => {
                println!("Login...")
            }
        }
        true
    }
}

pub struct AppComponents {
    sidebar: RelmComponent<SidebarModel, AppModel>,
    details: RelmComponent<DetailsModel, AppModel>,
    content: RelmComponent<ContentModel, AppModel>,
}

impl Components<AppModel> for AppComponents {
    fn init_components(parent_model: &AppModel, parent_sender: Sender<AppMsg>) -> Self {
        AppComponents {
            sidebar: RelmComponent::new(parent_model, parent_sender.clone()),
            details: RelmComponent::new(parent_model, parent_sender.clone()),
            content: RelmComponent::new(parent_model, parent_sender.clone())
        }
    }

    fn connect_parent(&mut self, _parent_widgets: &AppWidgets) {
    }
}


#[relm4_macros::widget(pub)]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        window = adw::ApplicationWindow {
            set_default_width: 800,
            set_default_height: 700,
            set_width_request: 460,
            set_height_request: 700,

            set_content: leaflet = Some(&adw::Leaflet) {
                set_can_navigate_back: true,
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_width_request: 320,
                    append: list_header = &adw::HeaderBar {
                        set_show_end_title_buttons: false,
                        set_title_widget = Some(&gtk::Label) {
                            set_label: "Do",
                        },
                    },
                    append: &components.sidebar.widgets().unwrap().list_container
                },
                append: &gtk::Separator::default(),
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,
                    set_vexpand: true,
                    append = &adw::HeaderBar {
                        set_hexpand: true,
                        set_show_end_title_buttons: true,
                    },
                    append: &components.content.widgets().unwrap().task_container
                }
            }
        }
    }
}

// #[relm4_macros::widget(pub)]
// impl Widgets<AppModel, ()> for AppWidgets {
//     view! {
//         window = adw::ApplicationWindow {
//             set_default_width: 600,
//             set_default_height: 700,
//             set_width_request: 600,
//             set_height_request: 700,
//
//             set_content: top = Some(&gtk::Box) {
//                 set_orientation: gtk::Orientation::Vertical,
//
//                 append: header = &adw::HeaderBar {
//                     set_title_widget = Some(&gtk::Label) {
//                         set_label: "Do",
//                     },
//                     pack_start: header_box = &gtk::Box {
//                         set_orientation: gtk::Orientation::Horizontal,
//                         append: reveal_button = &gtk::Button {
//                             set_icon_name: "open-menu-symbolic",
//                             connect_clicked(sender) => move |_| {
//                                 send!(sender, AppMsg::RevealSidebar)
//                             }
//                         }
//                     }
//                 },
//                 append: overlay = &gtk::Overlay {
//                     set_child: container = Some(&gtk::Box) {
//                         set_orientation: gtk::Orientation::Horizontal,
//                         append: &components.sidebar.widgets().unwrap().revealer,
//                         append: content = &gtk::Box {
//                             set_margin_all: 12,
//                             set_halign: gtk::Align::Center,
//                             set_hexpand: true,
//                             set_vexpand: true,
//                             append: welcome = &gtk::Box {
//                                 set_orientation: gtk::Orientation::Vertical,
//                                 set_spacing: 20,
//                                 set_valign: gtk::Align::Center,
//                                 set_halign: gtk::Align::Center,
//                                 set_width_request: 100,
//
//                                 append = &gtk::Picture {
//                                     set_filename: Some("/usr/share/icons/hicolor/scalable/apps/do.svg"),
//                                     set_keep_aspect_ratio: true,
//                                     set_can_shrink: true
//                                 },
//                                 append = &gtk::Label {
//                                     set_label: "Do",
//                                     add_css_class: "title"
//                                 },
//                                 append: &gtk::Label::new(Some("Do gives you focus, from work to play.")),
//                                 append: login_button = &gtk::Button {
//                                     set_label: "Login",
//                                     connect_clicked(sender) => move |_| {
//                                         send!(sender, AppMsg::Login)
//                                     }
//                                 }
//                             }
//                         },
//                         append: &gtk::Separator::default(),
//                         append: &components.details.widgets().unwrap().revealer
//                     },
//                     add_overlay: &components.content.widgets().unwrap().revealer
//                 }
//             }
//         }
//     }
// }