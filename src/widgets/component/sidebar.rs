use relm4::{ComponentParts, ComponentSender, gtk, gtk::prelude::{BoxExt, ListBoxRowExt, OrientableExt, WidgetExt}, SimpleComponent, view, WidgetPlus};
use relm4::factory::{DynamicIndex, FactoryVecDeque};

use crate::core::local::lists::get_lists;
use crate::core::local::tasks::{get_all_tasks, get_favorite_tasks, get_tasks};
use crate::models::list::List;

pub struct SidebarModel {
    lists: FactoryVecDeque<gtk::ListBox, List, SidebarInput>,
}

pub enum SidebarInput {
    AddList(String),
    RemoveList(DynamicIndex),
    ListSelected(usize),
    UpdateCounters(usize, usize),
}

pub enum SidebarOutput {
    ListSelected(usize, List),
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
                        sender.input.send(SidebarInput::ListSelected(index))
                        // send!(sender, SidebarMsg::Forward)
                    },
                },
            },
        }
    }

    fn init(
        params: Self::InitParams,
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
                self.lists.push_front(List::new(name.as_str(), "test", 0))
            }
            SidebarInput::RemoveList(index) => {
                let index = index.current_index();
                self.lists.remove(index);
            }
            SidebarInput::ListSelected(index) => {
                let list = self.lists.get(index);
                sender.output.send(SidebarOutput::ListSelected(index, list.clone()));
            }
            SidebarInput::UpdateCounters(index, count) => {
                if count > self.lists.get_mut(index).count as usize {
                    let mut list = self.lists.get_mut(3);
                    list.count = list.count.wrapping_add(1);
                } else {
                    let mut list = self.lists.get_mut(3);
                    list.count = list.count.wrapping_sub(1);
                }
                self.lists.get_mut(index).count = count as i32;
            }
        }
        self.lists.render_changes()
    }
}
