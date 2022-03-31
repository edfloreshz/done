use std::path::Path;

use gtk4 as gtk;
use libadwaita as adw;
use libadwaita::prelude::*;
use relm4_macros::view;

use crate::ui::content::MainWidgets;
use crate::ui::details::DetailsWidgets;
use crate::ui::sidebar::SidebarWidgets;
use crate::{MicrosoftTokenAccess, ToDoService};

#[derive(Clone)]
pub struct BaseWidgets {
    pub header: adw::HeaderBar,
    pub header_box: gtk::Box,
    pub container: gtk::Box,
    pub sidebar: SidebarWidgets,
    pub details: DetailsWidgets,
    pub main: MainWidgets,
    pub content: gtk::Box,
    pub login_button: gtk::Button,
    pub welcome: gtk::Box,
}

impl BaseWidgets {
    pub fn new(window: &adw::ApplicationWindow) -> Self {
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
        let sidebar = SidebarWidgets::new(&header_box);
        let details = DetailsWidgets::new();
        let login_button = gtk::Button::builder().label("Login").build();
        let welcome = if MicrosoftTokenAccess::is_token_present() {
            Self::create_welcome(None)
        } else {
            Self::create_welcome(Some(&login_button))
        };
        header.pack_start(&header_box);
        top.append(&header);
        container.append(&sidebar.revealer);
        container.append(&content);
        container.append(&gtk::Separator::default());
        container.append(&details.revealer);
        content.append(&welcome);
        let main = MainWidgets::new(&container);
        top.append(&main.overlay);
        window.set_content(Some(&top));
        Self {
            header,
            header_box,
            container,
            sidebar,
            details,
            main,
            content,
            login_button,
            welcome,
        }
    }
    pub fn create_welcome(login_button: Option<&gtk::Button>) -> gtk::Box {
        if login_button.is_some() {
            view! {
                welcome = gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 20,
                    set_valign: gtk::Align::Center,
                    set_halign: gtk::Align::Center,
                    set_width_request: 100,

                    append = &gtk::Picture {
                        set_filename: Some(Path::new("/src/assets/logo.png")),
                        set_keep_aspect_ratio: true,
                        set_can_shrink: true
                    },
                    append = &gtk::Label {
                        set_label: "Microsoft To Do",
                        add_css_class: "title"
                    },
                    append: &gtk::Label::new(Some("To Do gives you focus, from work to play.")),
                    append: login_button.unwrap()
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
                        set_filename: Some(Path::new("src/assets/logo.png")),
                        set_keep_aspect_ratio: true,
                        set_can_shrink: true
                    },
                    append = &gtk::Label {
                        set_label: "Microsoft To Do",
                        add_css_class: "title"
                    },
                    append: &gtk::Label::new(Some("To Do gives you focus, from work to play.")),
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
                    set_label: "To Do",
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
}
