use gtk4 as gtk;
use gtk4::prelude::{BoxExt, EntryBufferExtManual, EntryExt, OrientableExt, WidgetExt};
use relm4::factory::{DynamicIndex, FactoryVecDeque};
use relm4::{send, ComponentUpdate, Model, Sender, WidgetPlus, Widgets};
use std::collections::VecDeque;

use crate::core::local::tasks::{
    delete_task, get_all_tasks, get_favorite_tasks, get_tasks, patch_task, post_task,
};
use crate::widgets::sidebar::{SidebarModel, SidebarMsg};
use crate::widgets::task::{Task, TaskStatus};
use tracker::track;

#[track]
#[derive(Debug)]
pub struct ContentModel {
    pub id_list: String,
    pub index: usize,
    #[no_eq]
    pub tasks: FactoryVecDeque<Task>,
}

impl Default for ContentModel {
    fn default() -> Self {
        Self {
            id_list: "".to_string(),
            index: 0,
            tasks: FactoryVecDeque::from_vec_deque(VecDeque::new()),
            tracker: 0,
        }
    }
}

pub enum ContentMsg {
    AddTaskEntry(String),
    RemoveTask(DynamicIndex),
    SetTaskCompleted(DynamicIndex, bool),
    SetTaskFavorite(DynamicIndex, bool),
    ModifyTaskTitle(DynamicIndex, String),
    UpdateWidgetData(usize, String),
}

impl Model for ContentModel {
    type Msg = ContentMsg;
    type Widgets = ContentWidgets;
    type Components = ();
}

impl ComponentUpdate<SidebarModel> for ContentModel {
    fn init_model(parent_model: &SidebarModel) -> Self {
        let (index, id_list) = parent_model.selected_list.clone();
        Self {
            id_list: id_list.clone(),
            index,
            tasks: FactoryVecDeque::from_vec_deque(
                get_tasks(id_list)
                    .unwrap_or_default()
                    .iter()
                    .map(|task| task.to_owned())
                    .collect(),
            ),
            tracker: 0,
        }
    }

    fn update(
        &mut self,
        msg: Self::Msg,
        _components: &Self::Components,
        _sender: Sender<Self::Msg>,
        parent_sender: Sender<SidebarMsg>,
    ) {
        self.reset();
        let id = &self.id_list;
        match msg {
            ContentMsg::AddTaskEntry(title) => {
                post_task(id.to_owned(), title.clone()).expect("Failed to post task.");
                self.tasks.push_back(Task::new(title, id.to_owned()));
                send!(parent_sender, SidebarMsg::UpdateCounters)
            }
            ContentMsg::RemoveTask(index) => {
                let index = index.current_index();
                if let Some(task) = self.tasks.get(index) {
                    delete_task(&task.id_task).expect("Failed to update task.");
                    self.tasks.remove(index); // TODO: Fix warning: Gtk-CRITICAL **: 16:15:04.865: gtk_list_box_row_grab_focus: assertion 'box != NULL' failed
                    send!(parent_sender, SidebarMsg::UpdateCounters)
                }
            }
            ContentMsg::SetTaskCompleted(index, completed) => {
                let index = index.current_index();
                if let Some(task) = self.tasks.get_mut(index) {
                    task.status = if completed {
                        TaskStatus::Completed
                    } else {
                        TaskStatus::NotStarted
                    };
                    patch_task(task.into()).expect("Failed to update task.");
                }
            }
            ContentMsg::UpdateWidgetData(index, id_list) => {
                self.set_id_list(id_list.clone());
                self.set_index(index);
                let tasks = match index {
                    0 => vec![],
                    1 => vec![],
                    2 => vec![],
                    3 => get_all_tasks().unwrap_or_default(),
                    4 => get_favorite_tasks().unwrap_or_default(),
                    _ => get_tasks(id_list).unwrap_or_default(),
                };

                loop {
                    let task = self.tasks.pop_front();
                    if task.is_none() {
                        break;
                    }
                }
                for (i, _) in tasks.iter().enumerate() {
                    self.tasks.push_back(tasks[i].clone())
                }
            }
            ContentMsg::ModifyTaskTitle(index, title) => {
                let index = index.current_index();
                if let Some(task) = self.tasks.get_mut(index) {
                    task.set_title(title);
                    patch_task(task.into()).expect("Failed to update task.");
                }
            }
            ContentMsg::SetTaskFavorite(index, favorite) => {
                let index = index.current_index();
                if let Some(task) = self.tasks.get_mut(index) {
                    task.set_favorite(favorite);
                    patch_task(task.into()).expect("Failed to update task.");
                    if self.index == 4 {
                        self.tasks.remove(index); // TODO: Fix warning: Gtk-CRITICAL **: 16:15:04.865: gtk_list_box_row_grab_focus: assertion 'box != NULL' failed
                    }
                    send!(parent_sender, SidebarMsg::UpdateCounters)
                }
            }
        }
    }
}

#[relm4_macros::widget(pub)]
impl Widgets<ContentModel, SidebarModel> for ContentWidgets {
    view! {
        task_container = &gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            append = &gtk::Box {
                append: main_stack = &gtk::Stack {
                    add_child = &gtk::ScrolledWindow {
                        set_vexpand: true,
                        set_hexpand: true,
                        set_child: list_box = Some(&gtk::Box) {
                            set_orientation: gtk::Orientation::Vertical,
                            factory!(model.tasks)
                        }
                    },
                },
            },
            append = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_margin_all: 12,
                append: entry = &gtk::Entry {
                    set_hexpand: true,
                    set_visible: track!(model.changed(ContentModel::index()), model.index > 5),
                    set_icon_from_icon_name: args!(gtk::EntryIconPosition::Primary, Some("value-increase-symbolic")),
                    set_placeholder_text: Some("New task..."),
                    set_height_request: 42,
                    connect_activate(sender) => move |entry| {
                        let buffer = entry.buffer();
                        send!(sender, ContentMsg::AddTaskEntry(buffer.text()));
                        buffer.delete_text(0, None);
                    }
                }
            }
        }
    }
}
