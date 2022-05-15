use glib::Sender;
use relm4::{ComponentUpdate, Model, send, WidgetPlus, Widgets};
use relm4::factory::{DynamicIndex, FactoryVecDeque};
use relm4::gtk;
use relm4::gtk::gio::File;
use relm4::gtk::prelude::{
    BoxExt, ButtonExt, EntryBufferExtManual, EntryExt, OrientableExt, WidgetExt,
};

use crate::core::local::tasks::{
    delete_task, get_all_tasks, get_favorite_tasks, get_tasks, patch_task, post_task,
};
use crate::widgets::app::{AppModel, AppMsg};
use crate::widgets::contents::task_list::{Task, TaskStatus};
use crate::widgets::panel::sidebar::SidebarMsg;

#[tracker::track]
#[derive(Debug)]
pub struct ContentModel {
    pub parent_list: Option<String>,
    pub index: usize,
    pub show_tasks: bool,
    #[no_eq]
    pub tasks: FactoryVecDeque<Task>,
}

#[derive(Debug)]
pub enum ContentMsg {
    Add(String),
    Remove(DynamicIndex),
    SetCompleted(DynamicIndex, bool),
    SetFavorite(DynamicIndex, bool),
    ModifyTitle(DynamicIndex, String),
    OnUpdate(usize, String),
    RemoveWelcomeScreen,
}

impl Model for ContentModel {
    type Msg = ContentMsg;
    type Widgets = ContentWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for ContentModel {
    fn init_model(_: &AppModel) -> Self {
        Self {
            parent_list: None,
            index: 0,
            show_tasks: false,
            tasks: FactoryVecDeque::default(),
            tracker: 0,
        }
    }

    fn update(
        &mut self,
        msg: Self::Msg,
        _components: &Self::Components,
        _sender: Sender<Self::Msg>,
        parent_sender: Sender<AppMsg>,
    ) {
        self.reset();
        let id = &self.parent_list;
        match msg {
            ContentMsg::Add(title) => {
                post_task(id.as_ref().unwrap().to_owned(), title.clone())
                    .expect("Failed to post task.");
                self.tasks
                    .push_back(Task::new(title, id.as_ref().unwrap().to_owned()));
                send!(
                    parent_sender,
                    AppMsg::UpdateSidebar(SidebarMsg::UpdateCounters)
                )
            }
            ContentMsg::Remove(index) => {
                let index = index.current_index();
                if let Some(task) = self.tasks.get(index) {
                    delete_task(&task.id_task).expect("Failed to remove task.");
                    self.tasks.remove(index); // TODO: Fix warning: Gtk-CRITICAL **: 16:15:04.865: gtk_list_box_row_grab_focus: assertion 'box != NULL' failed
                    send!(
                        parent_sender,
                        AppMsg::UpdateSidebar(SidebarMsg::UpdateCounters)
                    )
                }
            }
            ContentMsg::SetCompleted(index, completed) => {
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
            ContentMsg::SetFavorite(index, favorite) => {
                let index = index.current_index();
                if let Some(task) = self.tasks.get_mut(index) {
                    task.set_favorite(favorite);
                    patch_task(task.into()).expect("Failed to update task.");
                    if self.index == 4 {
                        self.tasks.remove(index); // TODO: Fix warning: Gtk-CRITICAL **: 16:15:04.865: gtk_list_box_row_grab_focus: assertion 'box != NULL' failed
                    }
                    send!(
                        parent_sender,
                        AppMsg::UpdateSidebar(SidebarMsg::UpdateCounters)
                    )
                }
            }
            ContentMsg::ModifyTitle(index, title) => {
                let index = index.current_index();
                if let Some(task) = self.tasks.get_mut(index) {
                    task.set_title(title);
                    patch_task(task.into()).expect("Failed to update task.");
                }
            }
            ContentMsg::OnUpdate(index, id_list) => {
                self.set_parent_list(Some(id_list.clone()));
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
                    self.tasks.push_back(tasks[i].clone());
                }
                self.set_show_tasks(!self.tasks.is_empty());
            }
            ContentMsg::RemoveWelcomeScreen => {
                self.set_show_tasks(true);
            }
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<ContentModel, AppModel> for ContentWidgets {
    view! {
        tasks = &gtk::Stack {
            set_vexpand: true,
            add_child = &gtk::CenterBox {
                set_orientation: gtk::Orientation::Vertical,
                set_visible: track!(model.changed(ContentModel::show_tasks()), !model.show_tasks),
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Center,
                set_center_widget = Some(&gtk::Box) {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 24,
                    set_spacing: 24,
                    append = &gtk::Picture {
                        set_file: Some(&File::for_uri("https://raw.githubusercontent.com/edfloreshz/done/4a5e22c118e58c6de1758c76daf164bd6ad6ce38/src/widgets/assets/all-done.svg")),
                    },
                    append = &gtk::Label {
                        add_css_class: "title",
                        set_text: "Tasks Will Appear Here"
                    },
                    append = &gtk::Button {
                        set_visible: track!(model.changed(ContentModel::index()), model.index > 5),
                        add_css_class: "suggested-action",
                        set_label: "Add Tasks...",
                        connect_clicked(sender) => move |_| {
                            send!(sender, ContentMsg::RemoveWelcomeScreen)
                        }
                    }
                }
            },
            add_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_visible: track!(model.changed(ContentModel::show_tasks()), model.show_tasks),
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
                    }
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
                            send!(sender, ContentMsg::Add(buffer.text()));
                            buffer.delete_text(0, None);
                        }
                    }
                }
            },
        }
    }
}
