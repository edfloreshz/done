use glib::clone;
use gtk4 as gtk;

use gtk::prelude::*;
use relm4_macros::view;
use crate::ui::content::ContentWidgets;
use crate::ui::sidebar::SidebarWidgets;
use crate::ui::selection_row::ListBoxSelectionRow;

#[derive(Clone)]
pub struct BaseWidgets {
    pub header: gtk::HeaderBar,
    pub header_box: gtk::Box,
    pub container: gtk::Box,
    pub sidebar: SidebarWidgets,
    pub content_w: ContentWidgets,
    pub content: gtk::Stack,
}

impl BaseWidgets {
    pub fn new(window: &gtk::ApplicationWindow) -> Self {
        let header = Self::create_header();
        let header_box = Self::create_header_box();
        let container = Self::create_container();
        let content = Self::create_content();
        let sidebar = SidebarWidgets::new(&header_box);
        header.pack_start(&header_box);
        Self::setup_row_active(&sidebar, &content);
        container.append(&sidebar.revealer);
        container.append(&content);
        content.add_titled(&gtk::Label::new(Some("Test")), Some("Test"), "Test");
        let content_w = ContentWidgets::new(&container);
        window.set_child(Some(&content_w.overlay));
        window.set_titlebar(Some(&header));
        Self {
            header,
            header_box,
            container,
            sidebar,
            content_w,
            content
        }
    }
    fn create_header() -> gtk::HeaderBar {
        view! {
            header = gtk::HeaderBar {
                set_show_title_buttons: true,
                set_title_widget = Some(&gtk::Label) {
                    set_label: "To Do",
                }
            }
        }
        header
    }
    fn create_header_box() -> gtk::Box {
        view! {
            header_box = gtk::Box {
                set_orientation: gtk::Orientation::Horizontal
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
    fn create_content() -> gtk::Stack {
        gtk::Stack::new()
    }
    fn setup_row_active(nav: &SidebarWidgets, content: &gtk::Stack) {
        nav.list.connect_row_activated(clone!(
            @weak content,
            @weak nav.labels as labels,
            @weak nav.stack as nav_stack,
            @weak nav.subsection_revealer as nav_stack_revealer => move |_, row| {
			}),
        );
    }
}