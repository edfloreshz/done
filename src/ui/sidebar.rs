use std::cell::RefCell;
use std::rc::Rc;

use gtk4 as gtk;
use gtk4::glib::clone;
use gtk::prelude::*;
use relm4_macros::view;

#[derive(Clone)]
pub struct SidebarWidgets {
    pub revealer: gtk::Revealer,
    pub subsection_revealer: gtk::Revealer,
    pub reveal_button: gtk::Button,
    pub navigation_box: gtk::Box,
    pub list: gtk::ListBox,
    pub labels: Rc<RefCell<Vec<gtk::Label>>>,
    pub stack: gtk::Stack,
}

impl SidebarWidgets {
    pub fn new(header_box: &gtk::Box) -> Self {
        let navigation_box = Self::create_navigation_box();
        let stack = Self::create_stack();
        let revealer = Self::create_revealer(&navigation_box);
        let subsection_revealer = Self::create_subsection_revealer(&stack);
        let reveal_button = Self::create_reveal_button(&header_box, &revealer);
        let list = gtk4::ListBox::builder().vexpand(true).build();
        navigation_box.append(&list);
        navigation_box.append(&subsection_revealer);
        revealer.set_child(Some(&navigation_box));
        let labels = Rc::new(RefCell::new(vec![]));
        Self {
            revealer,
            navigation_box,
            list,
            labels,
            reveal_button,
            subsection_revealer,
            stack,
        }
    }

    fn create_navigation_box() -> gtk::Box {
        view! {
            navigation_box = gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_width_request: 250,
            }
        }
        navigation_box
    }
    fn create_stack() -> gtk::Stack {
        view! {
            stack = gtk::Stack {}
        }
        stack
    }
    fn create_revealer(navigation_box: &gtk4::Box) -> gtk::Revealer {
        view! {
            revealer = gtk::Revealer {
                set_child: Some(navigation_box),
                set_transition_type: gtk::RevealerTransitionType::SlideRight
            }
        }
        revealer
    }
    fn create_subsection_revealer(stack: &gtk::Stack) -> gtk::Revealer {
        gtk::Revealer::builder()
            .child(stack)
            .transition_type(gtk4::RevealerTransitionType::SlideRight)
            .build()
    }
    fn create_reveal_button(header_box: &gtk::Box, revealer: &gtk::Revealer) -> gtk::Button {
        view! {
            button = gtk::Button {
                set_icon_name: "open-menu-symbolic"
            }
        }
        button.connect_clicked(clone!(@weak revealer => move |_| {
            let active = revealer.reveals_child();
            if active {
                revealer.set_reveal_child(false);
            } else {
                revealer.set_reveal_child(true);
            }
        }));
        header_box.append(&button);
        button
    }
}