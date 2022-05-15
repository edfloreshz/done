use relm4::{ComponentParts, ComponentSender, gtk, gtk::prelude::{BoxExt, ListBoxRowExt, OrientableExt, WidgetExt}, SimpleComponent, WidgetPlus};
use relm4::factory::{DynamicIndex, FactoryVecDeque};

use crate::core::local::lists::{get_lists, post_list};
use crate::core::local::tasks::{get_all_tasks, get_favorite_tasks};
use crate::models::list::List;
use crate::widgets::factory::list::ListType;

pub struct SidebarModel {
    lists: FactoryVecDeque<gtk::ListBox, List, SidebarInput>,
}

pub enum SidebarInput {
    AddList(String),
    RemoveList(DynamicIndex),
    ListSelected(usize),
    UpdateCounters(Vec<ListType>),
}

pub enum SidebarOutput {
    ListSelected(usize, List),
    Forward,
}

#[relm4::component(pub)]
impl SimpleComponent for SidebarModel {
    type Input = SidebarInput;
    type Output = SidebarOutput;
    type InitParams = Option<List>;
    type Widgets = SidebarWidgets;

    view! {
        sidebar = &gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            append: scroll_window = &gtk::ScrolledWindow {
                set_child: list = Some(&gtk::ListBox) {
                    set_selection_mode: gtk::SelectionMode::Single,
                    set_vexpand: true,
                    set_margin_all: 2,
                    set_css_classes: &["navigation-sidebar"],
                    connect_row_activated(sender) => move |listbox, _| {
                        let index = listbox.selected_row().unwrap().index() as usize;
                        sender.input.send(SidebarInput::ListSelected(index));
                        sender.output.send(SidebarOutput::Forward)
                    },
                },
            },
        }
    }

    fn init(
        _params: Self::InitParams,
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();
        let mut model = SidebarModel {
            lists: FactoryVecDeque::new(widgets.list.clone(), &sender.input),
        };
        let mut lists = vec![
            List::new("Inbox", "document-save-symbolic", 0),
            List::new("Today", "display-brightness-symbolic", 0),
            List::new("Next 7 Days", "x-office-calendar-symbolic", 0),
            List::new(
                "All",
                "edit-paste-symbolic",
                get_all_tasks().unwrap_or_default().len() as i32,
            ),
            List::new(
                "Starred",
                "non-starred-symbolic",
                get_favorite_tasks().unwrap_or_default().len() as i32,
            ),
            List::new("Archived", "folder-symbolic", 0),
        ];
        lists.append(&mut get_lists().unwrap_or_default());
        for list in lists {
            model.lists.push_back(list);
        }
        model.lists.render_changes();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: &ComponentSender<Self>) {
        match message {
            SidebarInput::AddList(name) => {
                let posted_list = post_list(name.clone()).unwrap();
                self.lists.push_back(posted_list)
            }
            SidebarInput::RemoveList(index) => {
                let index = index.current_index();
                self.lists.remove(index);
            }
            SidebarInput::ListSelected(index) => {
                let list = self.lists.get(index);
                sender.output.send(SidebarOutput::ListSelected(index, list.clone()));
            }
            SidebarInput::UpdateCounters(lists) => {
                for list in lists {
                    match list {
                        ListType::Inbox(i) => self.lists.get_mut(0).count += i as i32,
                        ListType::Today(i) => self.lists.get_mut(1).count += i as i32,
                        ListType::Next7Days(i) => self.lists.get_mut(2).count += i as i32,
                        ListType::All(i) => self.lists.get_mut(3).count += i as i32,
                        ListType::Starred(i) => self.lists.get_mut(4).count += i as i32,
                        ListType::Archived(i) => self.lists.get_mut(5).count += i as i32,
                        ListType::Other(index, i) => self.lists.get_mut(index).count += i as i32
                    }
                }
            }
        }
        self.lists.render_changes()
    }
}
