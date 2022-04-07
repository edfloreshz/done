use gtk4::prelude::*;
use gtk4 as gtk;
use relm4::{ComponentUpdate, Model, Widgets, send, Sender};
use relm4_macros::view;
use crate::{AppModel};
use crate::models::list::List;
use crate::widgets::app::AppMsg;

pub(crate) struct SidebarModel {
    lists: Vec<List>
}

pub enum SidebarMsg {
    Delete(usize),
    AddList(String),
    SelectList(usize),
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

    fn update(&mut self, msg: Self::Msg, _components: &Self::Components, _sender: Sender<Self::Msg>, _parent_sender: Sender<AppMsg>) {
        match msg {
            SidebarMsg::Delete(_) => {}
            SidebarMsg::AddList(entry) => println!("{entry}"),
            SidebarMsg::SelectList(i) => println!("{i}"),
            SidebarMsg::Rename(_, _) => {}
        }
    }
}

#[relm4_macros::widget(pub)]
impl Widgets<SidebarModel, AppModel> for SidebarWidgets {
    view! {
        revealer = gtk::Revealer {
            set_child: navigation_box = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                set_width_request: 250,
                append: scroll_window = &gtk::ScrolledWindow {
                    set_child: list = Some(&gtk::ListBox) {
                        set_vexpand: true,
                        connect_row_activated(sender) => move |listbox, _| {
                            let index = listbox.selected_row().unwrap().index() as usize;
                            send!(sender, SidebarMsg::SelectList(index))
                        }
                    }
                },
                append: action_buttons = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 10,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_margin_start: 10,
                    set_margin_end: 10,
                    set_halign: gtk::Align::Center,
                    append: add_list_button = &gtk::MenuButton {
                        set_label: "Add List",
                        set_popover: new_list_popover = Some(&gtk::Popover) {
                            set_child: new_list_entry = Some(&gtk::Entry) {
                                connect_activate(sender) => move |entry| {
                                    let buffer = entry.buffer();
                                    send!(sender, SidebarMsg::AddList(buffer.text()))
                                }
                            }
                        }
                    },
                    append: add_group_button = &gtk::MenuButton {
                        set_label: "Add Group",
                    }
                },
                append: subsection_revealer = &gtk::Revealer {
                    set_child = Some(&gtk::Stack) {},
                    set_transition_type: gtk::RevealerTransitionType::SlideRight
                }
            },
            set_transition_type: gtk::RevealerTransitionType::SlideRight,
        }
    }
    fn post_init() {
        for input in &model.lists {
            view! {
                label = &gtk::Label {
                    set_halign: gtk::Align::Start,
                    set_text: input.display_name.as_str(),
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_margin_start: 15,
                    set_margin_end: 15,
                }
            }
            list.append(&label)
        }
    }
}