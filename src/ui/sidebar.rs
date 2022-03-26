use std::cell::RefCell;
use std::rc::Rc;
use relm4_macros::view;
use gtk4 as gtk;
use gtk4::glib::clone;
use gtk::prelude::*;

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
        let labels = Rc::new(RefCell::new(vec![gtk::Label::new(Some("Test"))]));
        Self {
            revealer,
            navigation_box,
            list,
            labels,
            reveal_button,
            subsection_revealer,
            stack
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
    // pub fn create_scroll_area(navigation_box: &gtk::Box) -> gtk::ScrolledWindow {
    //     view! {
    //         scroll_area = &gtk::ScrolledWindow {
    //             set_vexpand: true,
    //             set_width_request: 250,
    //             set_hscrollbar_policy: gtk::PolicyType::Never,
    //
    //             // set_child: Some(components.lists.root_widget())
    //         }
    //     }
    //     navigation_box.append(&scroll_area);
    //     scroll_area
    // }
    // pub fn create_tree_view(scroll_area: &gtk::ScrolledWindow) -> gtk::TreeView {
    //     view! {
    //         tree_view = &gtk::TreeView {
    //             set_width_request: 200,
    //             set_headers_visible: false,
    //             set_level_indentation: 12,
    //             set_can_focus: false,
    //             set_visible: true,
    //             set_show_expanders: true,
    //
    //             append_column = &gtk::TreeViewColumn {
    //                 set_title: "List"
    //             },
    //             set_model: Some(&gtk::TreeStore::new(&[glib::Type::STRING])),
    //         }
    //     }
    //     append_text_column(&tree_view);
    //
    //     scroll_area.set_child(Some(&tree_view));
    //     tree_view
    // }
}

fn append_text_column(tree: &gtk::TreeView) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();
    cell.set_height(50);

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}