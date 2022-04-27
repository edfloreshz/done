use gtk4 as gtk;
use gtk4::prelude::{BoxExt, EntryBufferExtManual, EntryExt, OrientableExt, WidgetExt};
use relm4::{ComponentUpdate, Model, send, Sender, WidgetPlus, Widgets};
use relm4::factory::FactoryVec;

use crate::AppModel;
use crate::models::task::{Task, TaskStatus};
use crate::services::local::tasks::{get_tasks, patch_task, post_task};
use crate::widgets::app::AppMsg;

#[derive(Debug)]
pub struct ContentModel {
    pub(crate) list_id: String,
    pub(crate) tasks: FactoryVec<Task>,
}

impl Default for ContentModel {
    fn default() -> Self {
        Self {
            list_id: "".to_string(),
            tasks: FactoryVec::from_vec(vec![]),
        }
    }
}

pub enum ContentMsg {
    ParentUpdate(String),
    AddTaskEntry(String),
    SetCompleted((usize, bool)),
    Favorite((usize, bool)),
}

impl Model for ContentModel {
    type Msg = ContentMsg;
    type Widgets = ContentWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for ContentModel {
    fn init_model(parent_model: &AppModel) -> Self {
        Self {
            list_id: parent_model.selected_list.clone(),
            tasks: FactoryVec::from_vec(
                get_tasks(parent_model.selected_list.clone())
                    .unwrap()
                    .iter()
                    .map(|task| task.to_owned().into())
                    .collect()
            )
        }
    }

    fn update(&mut self, msg: Self::Msg, _components: &Self::Components, _sender: Sender<Self::Msg>, _parent_sender: Sender<AppMsg>) {
        let id = &self.list_id.to_owned();
        match msg {
            ContentMsg::AddTaskEntry(title) => {
                post_task(id.to_owned(), title.clone()).expect("Failed to post task.");
                self.tasks.push(Task::new(title, id.to_owned()))
            }
            ContentMsg::SetCompleted((index, completed)) => {
                if let Some(task) = self.tasks.get_mut(index) {
                    task.status = if completed {
                        TaskStatus::Completed
                    } else {
                        TaskStatus::NotStarted
                    };
                    patch_task(task).expect("Failed to update task.");
                }
            }
            ContentMsg::ParentUpdate(list_id) => {
                self.list_id = list_id.clone();
                let tasks = get_tasks(list_id)
                        .unwrap()
                        .iter()
                        .map(|task| task.to_owned().into())
                        .collect::<Vec<Task>>();
                loop {
                    let task = self.tasks.pop(); // TODO: Fix pop for ListBox
                    if task.is_none() {
                        break
                    }
                }
                for (i, _) in tasks.iter().enumerate() {
                    self.tasks.push(tasks[i].clone())
                }
            }
            ContentMsg::Favorite((index, favorite)) => {
                if let Some(task) = self.tasks.get_mut(index) {
                    task.favorite = favorite;
                    patch_task(task).expect("Failed to update task.");
                }
            }
        }
    }
}

#[relm4_macros::widget(pub)]
impl Widgets<ContentModel, AppModel> for ContentWidgets {
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