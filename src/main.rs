use cascade::cascade;
use glib::clone;
use gtk4::glib::Sender;
use once_cell::sync::OnceCell;
use relm4::{AppUpdate, Components, Model, RelmApp, RelmComponent, view, Widgets};
use tokio::runtime::Runtime;
use relm4::{adw, adw::prelude::*, gtk, gtk::prelude::*};
use relm4::adw::gdk::Display;
use crate::models::list::List;
use crate::widgets::content::{ContentModel, ContentWidgets};
use crate::widgets::details::{DetailsModel};
use crate::widgets::sidebar::SidebarModel;
use crate::gtk::CssProvider;

mod task;
mod models;
mod widgets;

static RT: OnceCell<Runtime> = OnceCell::new();

#[derive(Clone)]
struct AppModel {
    lists: Vec<List>
}

enum AppMsg {
    ListSelect
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: Self::Msg, components: &Self::Components, sender: Sender<Self::Msg>) -> bool {
        match msg { AppMsg::ListSelect => println!("List selected") }
        true
    }
}

struct AppComponents {
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

struct AppWidgets {
    window: adw::ApplicationWindow,
    header: adw::HeaderBar,
    header_box: gtk::Box,
    container: gtk::Box,
    content: gtk::Box,
    login_button: gtk::Button,
    welcome: gtk::Box,
    // Content
    pub overlay: gtk::Overlay,
    // Sidebar
    pub reveal_button: gtk::Button,

}

impl AppWidgets {
    pub fn new(components: &AppComponents) -> Self {
        view! {
            window = adw::ApplicationWindow {
                set_default_width: 600,
                set_default_height: 700,
                set_width_request: 600,
                set_height_request: 700,
            }
        }
        let provider = cascade! {
            CssProvider::new();
            ..load_from_data(include_bytes!("resources/style/ui.css"));
        };
        gtk4::StyleContext::add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        let header = Self::create_header();
        let header_box = Self::create_header_box();
        let top = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let container = Self::create_container();
        let content = gtk::Box::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .halign(gtk::Align::Center)
            .hexpand(true)
            .vexpand(true)
            .build();
        let login_button = gtk::Button::builder().label("Login").build();
        let welcome = if true { // TODO: Check if token is present
            Self::create_welcome(None)
        } else {
            Self::create_welcome(Some(&login_button))
        };
        let overlay = gtk::Overlay::builder().child(&container).build();
        let reveal_button = Self::create_reveal_button(&header_box, &components.sidebar.widgets().unwrap().revealer);
        overlay.add_overlay(&components.content.widgets().unwrap().revealer);
        header.pack_start(&header_box);
        top.append(&header);
        container.append(&components.sidebar.widgets().unwrap().revealer);
        container.append(&content);
        container.append(&gtk::Separator::default());
        container.append(&components.details.widgets().unwrap().revealer);
        content.append(&welcome);
        top.append(&overlay);
        window.set_content(Some(&top));
        Self {
            window,
            header,
            header_box,
            reveal_button,
            container,
            content,
            login_button,
            welcome,
            overlay
        }
    }
    pub fn create_welcome(login_button: Option<&gtk::Button>) -> gtk::Box {
        if let Some(button) = login_button {
            view! {
                welcome = gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 20,
                    set_valign: gtk::Align::Center,
                    set_halign: gtk::Align::Center,
                    set_width_request: 100,

                    append = &gtk::Picture {
                        set_filename: Some("/usr/share/icons/hicolor/scalable/apps/do.svg"),
                        set_keep_aspect_ratio: true,
                        set_can_shrink: true
                    },
                    append = &gtk::Label {
                        set_label: "Do",
                        add_css_class: "title"
                    },
                    append: &gtk::Label::new(Some("Do gives you focus, from work to play.")),
                    append: button
                }
            }
            welcome
        } else {
            view! {
                welcome = gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 20,
                    set_valign: gtk::Align::Center,
                    set_halign: gtk::Align::Center,
                    set_width_request: 100,

                    append = &gtk::Picture {
                        set_filename: Some("/usr/share/icons/hicolor/scalable/apps/do.svg"),
                        set_keep_aspect_ratio: true,
                        set_can_shrink: true
                    },
                    append = &gtk::Label {
                        set_label: "Do",
                        add_css_class: "title"
                    },
                    append: &gtk::Label::new(Some("Do gives you focus, from work to play.")),
                }
            }
            welcome
        }
    }
    pub fn update_welcome(&self) {
        let last = self.welcome.last_child().unwrap();
        self.welcome.remove(&last);
    }
    fn create_header() -> adw::HeaderBar {
        view! {
            header = adw::HeaderBar {
                set_title_widget = Some(&gtk::Label) {
                    set_label: "Do",
                },
            }
        }
        header
    }
    fn create_header_box() -> gtk::Box {
        view! {
            header_box = gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
            }
        }
        header_box
    }
    fn create_container() -> gtk::Box {
        view! {
            container = gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
            }
        }
        container
    }
    fn create_reveal_button(header_box: &gtk::Box, revealer: &gtk::Revealer) -> gtk::Button {
        view! {
            button = gtk::Button {
                set_icon_name: "open-menu-symbolic"
            }
        }
        button.connect_clicked(clone!(@weak revealer => move |_| {
            revealer.set_reveal_child(!revealer.reveals_child());
        }));
        header_box.append(&button);
        button
    }
}

impl Widgets<AppModel, ()> for AppWidgets {
    type Root = adw::ApplicationWindow;

    fn init_view(model: &AppModel, components: &AppComponents, sender: Sender<AppMsg>) -> Self {
        AppWidgets::new(components)
    }

    fn root_widget(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(&mut self, model: &AppModel, sender: Sender<AppMsg>) {
        todo!()
    }
}

fn main() {
    let model = AppModel {
        lists: vec![]
    };
    let relm = RelmApp::new(model);
    relm.run()
}
