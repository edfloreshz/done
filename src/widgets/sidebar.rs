use std::cell::RefCell;
use std::rc::Rc;
use glib::clone;
use gtk4::prelude::*;
use relm4::{ComponentUpdate, Model, Widgets, view};
use crate::{AppModel, AppMsg, gtk, List, Sender};

pub(crate) struct SidebarModel {
    lists: Vec<List>
}

pub enum SidebarMsg {
    Delete(usize),
    Create(String),
    Select(usize),
    Rename(usize, String),
}

impl Model for SidebarModel {
    type Msg = SidebarMsg;
    type Widgets = SidebarWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for SidebarModel {
    fn init_model(parent_model: &AppModel) -> Self {
        SidebarModel {
            lists: parent_model.lists.clone()
        }
    }

    fn update(&mut self, msg: Self::Msg, components: &Self::Components, sender: Sender<Self::Msg>, parent_sender: Sender<AppMsg>) {
        todo!()
    }
}

pub struct SidebarWidgets {
    pub revealer: gtk::Revealer,
    pub subsection_revealer: gtk::Revealer,
    pub add_list_button: gtk::MenuButton,
    pub add_group_button: gtk::MenuButton,
    pub navigation_box: gtk::Box,
    pub list: gtk::ListBox,
    pub labels: Rc<RefCell<Vec<gtk::Label>>>,
    pub stack: gtk::Stack,
    pub new_list_popover: gtk::Popover,
    pub new_list_entry: gtk::Entry,
}

impl SidebarWidgets {
    pub fn new() -> Self {
        let navigation_box = Self::create_navigation_box();
        let scroll_window = Self::create_scrolled_window();
        let stack = Self::create_stack();
        let revealer = Self::create_revealer(&navigation_box);
        let subsection_revealer = Self::create_subsection_revealer(&stack);
        let list = gtk4::ListBox::builder().vexpand(true).build();
        let (action_buttons, add_list_button, add_group_button) = Self::create_action_buttons();
        let (new_list_popover, new_list_entry) = Self::create_new_list_popover();
        add_list_button.set_popover(Some(&new_list_popover));
        scroll_window.set_child(Some(&list));
        navigation_box.append(&scroll_window);
        navigation_box.append(&action_buttons);
        navigation_box.append(&subsection_revealer);
        revealer.set_child(Some(&navigation_box));
        let labels = Rc::new(RefCell::new(vec![]));
        Self {
            revealer,
            navigation_box,
            list,
            labels,
            add_list_button,
            subsection_revealer,
            stack,
            add_group_button,
            new_list_popover,
            new_list_entry,
        }
    }
    fn create_new_list_popover() -> (gtk::Popover, gtk::Entry) {
        let entry = gtk::Entry::default();
        view! {
            popover = gtk::Popover {
                set_child: Some(&entry)
            }
        }
        (popover, entry)
    }
    fn create_action_buttons() -> (gtk::Box, gtk::MenuButton, gtk::MenuButton) {
        view! {
            action_buttons = gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_top: 10,
                set_margin_bottom: 10,
                set_margin_start: 10,
                set_margin_end: 10,
                set_halign: gtk::Align::Center,
                append: add_list_button = &gtk::MenuButton {
                    set_label: "Add List",
                },
                append: add_group_button = &gtk::MenuButton {
                    set_label: "Add Group",
                }
            }
        }
        (action_buttons, add_list_button, add_group_button)
    }
    fn create_scrolled_window() -> gtk::ScrolledWindow {
        view! {
            scrolled = gtk::ScrolledWindow {}
        }
        scrolled
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

}

impl Widgets<SidebarModel, AppModel> for SidebarWidgets {
    type Root = gtk::Revealer;

    fn init_view(model: &SidebarModel, _components: &(), sender: Sender<SidebarMsg>) -> Self {
        SidebarWidgets::new()
    }

    fn root_widget(&self) -> Self::Root {
        self.revealer.clone()
    }

    fn view(&mut self, model: &SidebarModel, sender: Sender<SidebarMsg>) {

    }
}