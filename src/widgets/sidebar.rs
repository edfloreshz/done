use std::ops::Index;

use gtk4::prelude::*;
use relm4::{
    adw, gtk, send, ComponentUpdate, Components, MicroComponent, Model, RelmComponent, Sender,
    WidgetPlus, Widgets,
};

use crate::core::local::lists::{get_lists, post_list};
use crate::core::local::tasks::{get_all_tasks, get_favorite_tasks, get_tasks};
use crate::widgets::app::AppMsg;
use crate::widgets::content::{ContentModel, ContentMsg};
use crate::widgets::list::List;
use crate::AppModel;

pub struct SidebarModel {
    lists: Vec<MicroComponent<List>>,
    pub selected_list: (usize, String),
}

pub enum SidebarMsg {
    Delete(usize),
    AddList(String),
    ListSelected(usize),
    Rename(usize, String),
    UpdateCounters,
}

impl Model for SidebarModel {
    type Msg = SidebarMsg;
    type Widgets = SidebarWidgets;
    type Components = SidebarComponents;
}

pub struct SidebarComponents {
    content: RelmComponent<ContentModel, SidebarModel>,
}

impl Components<SidebarModel> for SidebarComponents {
    fn init_components(parent_model: &SidebarModel, parent_sender: Sender<SidebarMsg>) -> Self {
        SidebarComponents {
            content: RelmComponent::new(parent_model, parent_sender),
        }
    }

    fn connect_parent(&mut self, _parent_widgets: &SidebarWidgets) {}
}

impl ComponentUpdate<AppModel> for SidebarModel {
    fn init_model(_: &AppModel) -> Self {
        let mut lists = vec![
            List::new_mc("Inbox", "document-save-symbolic", 0),
            List::new_mc("Today", "display-brightness-symbolic", 0),
            List::new_mc("Next 7 Days", "x-office-calendar-symbolic", 0),
            List::new_mc(
                "All",
                "edit-paste-symbolic",
                get_all_tasks().unwrap_or_default().len() as i32,
            ),
            List::new_mc(
                "Starred",
                "non-starred-symbolic",
                get_favorite_tasks().unwrap_or_default().len() as i32,
            ),
            List::new_mc("Archived", "folder-symbolic", 0),
        ];
        lists.append(&mut get_lists().unwrap_or_default());
        {
            // TODO: Fix COUNT trigger to remove this code.
            for (index, list) in lists.iter().enumerate() {
                let mut model = list.model_mut().unwrap();
                let id_list = &model.id_list;
                let count = match index {
                    0 => 0,
                    1 => 0,
                    2 => 0,
                    3 => get_all_tasks().unwrap_or_default().len(),
                    4 => get_favorite_tasks().unwrap_or_default().len(),
                    _ => get_tasks(id_list.to_owned()).unwrap_or_default().len(),
                };
                model.set_count(count as i32);
                drop(model);
                list.update_view().unwrap();
            }
        }
        SidebarModel {
            lists,
            selected_list: Default::default(),
        }
    }

    fn update(
        &mut self,
        msg: Self::Msg,
        components: &Self::Components,
        _sender: Sender<Self::Msg>,
        _parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            SidebarMsg::Delete(index) => {
                self.lists.remove(index);
            }
            SidebarMsg::AddList(name) => {
                let posted_list = post_list(name).unwrap();
                self.lists.push(MicroComponent::new(posted_list, ()))
            }
            SidebarMsg::ListSelected(index) => {
                let model = self.lists.index(index).model_mut().unwrap();
                self.selected_list = (index, model.id_list.clone());
                components
                    .content
                    .send(ContentMsg::UpdateWidgetData(index, model.id_list.clone()))
                    .unwrap();
                drop(model);
                self.lists.index(index).update_view().unwrap();
            }
            SidebarMsg::Rename(index, name) => {
                let mut model = self.lists.index(index).model_mut().unwrap();
                model.display_name = name;
                drop(model);
                self.lists.index(index).update_view().unwrap();
            }
            SidebarMsg::UpdateCounters => {
                for (index, list) in self.lists.iter().enumerate() {
                    let mut model = list.model_mut().unwrap();
                    let id_list = &model.id_list;
                    let count = match index {
                        0 => 0,
                        1 => 0,
                        2 => 0,
                        3 => get_all_tasks().unwrap().len(),
                        4 => get_favorite_tasks().unwrap().len(),
                        _ => get_tasks(id_list.to_owned()).unwrap().len(),
                    };
                    model.set_count(count as i32);
                    drop(model);
                    list.update_view().unwrap();
                }
            }
        }
    }
}

#[relm4_macros::widget(pub)]
impl Widgets<SidebarModel, AppModel> for SidebarWidgets {
    view! {
        leaflet = &adw::Leaflet {
            set_can_navigate_back: true,
            append = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_width_request: 270,
                append: list_container = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    append: scroll_window = &gtk::ScrolledWindow {
                        set_child: list = Some(&gtk::ListBox) {
                            set_selection_mode: gtk::SelectionMode::Single,
                            set_vexpand: true,
                            set_margin_all: 2,
                            set_css_classes: &["navigation-sidebar"],
                            connect_row_activated(sender) => move |listbox, _| {
                                let index = listbox.selected_row().unwrap().index() as usize;
                                send!(sender, SidebarMsg::ListSelected(index))
                            },
                            append: iterate! {
                                model.lists.iter().map(|list| {
                                    list.root_widget() as &gtk::Box
                                }).collect::<Vec<&gtk::Box>>()
                            }
                        },
                    },
                }
            },
            append: &gtk::Separator::default(),
            append: content_box = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,
                set_vexpand: true,
                append: &components.content.widgets().unwrap().task_container,
            }
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
