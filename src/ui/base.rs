use std::path::Path;

use gtk4 as gtk;
use libadwaita as adw;
use libadwaita::prelude::*;
use relm4_macros::view;

use crate::ui::content::MainWidgets;
use crate::ui::sidebar::SidebarWidgets;

#[derive(Clone)]
pub struct BaseWidgets {
    pub header: adw::HeaderBar,
    pub header_box: gtk::Box,
    pub container: gtk::Box,
    pub sidebar: SidebarWidgets,
    pub main: MainWidgets,
    pub content: gtk::Box,
    pub login_button: gtk::Button,
    pub login_dialog: gtk::Dialog,
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
        let login_button = gtk::Button::builder().label("Login").build();
        let welcome = Self::create_welcome(&login_button);
        let login_dialog = Self::create_login_dialog();
        header.pack_start(&header_box);
        top.append(&header);
        container.append(&sidebar.revealer);
        container.append(&content);
        content.append(&welcome);
        let main = MainWidgets::new(&container);
        top.append(&main.overlay);
        window.set_content(Some(&top));
        Self {
            header,
            header_box,
            container,
            sidebar,
            main,
            content,
            login_button,
            login_dialog,
        }
    }
    fn create_welcome(login_button: &gtk::Button) -> gtk::Box {
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
                append: login_button
            }
        }
        welcome
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
    fn create_login_dialog() -> gtk::Dialog {
        // let context = WebContext::default().unwrap();
        // #[cfg(feature = "v2_4")]
        // context.set_web_extensions_initialization_user_data(&"webkit".to_variant());
        // context.set_web_extensions_directory("../do/target/debug/");
        // #[cfg(feature = "v2_6")]
        // let webview = WebView::new_with_context_and_user_content_manager(&context, &UserContentManager::new());
        // #[cfg(not(feature = "v2_6"))]
        // let webview = webkit2gtk::WebView::with_context(&context);
        // webview.load_uri("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?
        //     client_id=af13f4ae-b607-4a07-9ddc-6c5c5d59979f
        //     &response_type=code
        //     &redirect_uri=https://login.microsoftonline.com/common/oauth2/nativeclient
        //     &response_mode=form_post
        //     &scope=offline_access%20user.read%20tasks.read%20tasks.read.shared%20tasks.readwrite%20tasks.readwrite.shared%20
        //     &state=1234");
        let dialog = gtk::Dialog::new();
        // dialog.set_(Some(&webview));
        // dialog.set_child(Some(&container));
        dialog
    }
}
