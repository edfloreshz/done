use std::ops::Index;

use glib::clone;
use gtk4::prelude::*;
use relm4::{gtk, send, ComponentUpdate, MicroComponent, Model, Sender, WidgetPlus, Widgets};

use crate::models::list::List;
use crate::services::local::lists::{get_lists, patch_list, post_list};
use crate::widgets::app::AppMsg;
use crate::AppModel;

#[derive(Default)]
pub(crate) struct SidebarModel {
    lists: Vec<MicroComponent<List>>,
}

pub enum SidebarMsg {
    Delete(usize),
    AddList(String),
    SelectList(usize),
    Rename(usize, String),
    UpdateCounter((usize, bool)),
}

impl Model for SidebarModel {
    type Msg = SidebarMsg;
    type Widgets = SidebarWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for SidebarModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        let mut lists = vec![
            MicroComponent::new(List::new("Inbox", "document-save-symbolic"), ()),
            MicroComponent::new(List::new("Today", "display-brightness-symbolic"), ()),
            MicroComponent::new(List::new("Next 7 Days", "x-office-calendar-symbolic"), ()),
            MicroComponent::new(List::new("All", "edit-paste-symbolic"), ()),
            MicroComponent::new(List::new("Starred", "non-starred-symbolic"), ()),
            MicroComponent::new(List::new("Archived", "folder-symbolic"), ()),
        ];
        let fe = &mut get_lists()
            .unwrap()
            .iter()
            .map(|list| MicroComponent::new(list.to_owned(), ()))
            .collect();
        lists.append(fe);
        SidebarModel { lists }
    }

    fn update(
        &mut self,
        msg: Self::Msg,
        _components: &Self::Components,
        _sender: Sender<Self::Msg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            SidebarMsg::Delete(i) => println!("Deleting list at index {i}"),
            SidebarMsg::AddList(name) => {
                let posted_list = post_list(name).unwrap();
                self.lists.push(MicroComponent::new(posted_list, ()))
            }
            SidebarMsg::SelectList(i) => {
                let id_list = &self.lists.index(i).model_mut().unwrap().id_list;
                parent_sender
                    .send(AppMsg::ListSelected((i, id_list.to_owned())))
                    .expect("Failed to get task list.");
            }
            SidebarMsg::Rename(i, name) => println!("Renaming list at index {i} to {name}"),
            SidebarMsg::UpdateCounter((index, add)) => {
                let list = &mut self.lists.index(index).model_mut().unwrap();
                if add {
                    list.count += 1;
                } else {
                    list.count -= 1;
                }
                patch_list(list).expect("Failed to update counter.");
            }
        }
    }
}

#[relm4_macros::widget(pub)]
impl Widgets<SidebarModel, AppModel> for SidebarWidgets {
    view! {
        list_container = &gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            append: scroll_window = &gtk::ScrolledWindow {
                set_child: list = Some(&gtk::ListBox) {
                    set_selection_mode: gtk::SelectionMode::Single,
                    set_vexpand: true,
                    set_margin_all: 2,
                    set_css_classes: &["navigation-sidebar"],
                    connect_row_activated(sender) => move |listbox, _| {
                        let index = listbox.selected_row().unwrap().index() as usize;
                        send!(sender, SidebarMsg::SelectList(index))
                    },
                    append: iterate! {
                        model.lists.iter().map(|list| {
                            list.root_widget() as &gtk::Box
                        }).collect::<Vec<&gtk::Box>>()
                    }
                },
            },
            append: action_buttons = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_top: 10,
                set_margin_bottom: 10,
                set_margin_start: 10,
                set_margin_end: 10,
                set_halign: gtk::Align::Fill,
                append: add_list_button = &gtk::MenuButton {
                    set_label: "Add List",
                    set_direction: gtk::ArrowType::Up,
                    set_popover: new_list_popover = Some(&gtk::Popover) {
                        set_child: stack = Some(&gtk::Stack) {
                            add_child = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 10,
                                append: &gtk::Label::new(Some("List Name")),
                                append: new_list_entry = &gtk::Entry {
                                    connect_activate(sender) => move |entry| {
                                        let buffer = entry.buffer();
                                        if !buffer.text().is_empty() {
                                            send!(sender, SidebarMsg::AddList(buffer.text()))
                                        }
                                    }
                                },
                                append: add_button = &gtk::Button {
                                    set_label: "Create List",
                                    set_css_classes: &["suggested-action"],
                                    connect_clicked: clone!(@weak new_list_entry, @strong sender => move |_| {
                                        let buffer = new_list_entry.buffer();
                                        if !buffer.text().is_empty() {
                                            send!(sender, SidebarMsg::AddList(buffer.text()))
                                        }
                                    })
                                },
                            }
                        }
                    }
                },
                // append: add_group_button = &gtk::MenuButton {
                //     set_label: "Add Group",
                //     set_direction: gtk::ArrowType::Up,
                // } // TODO: Add this when we can
            },
        }
    }
    fn post_view() {
        for list in &model.lists {
            if !list.is_connected() {
                self.list.append(list.root_widget())
            }
        }
    }
}
