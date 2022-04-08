use gtk4 as gtk;
use gtk4::prelude::{BoxExt, OrientableExt, WidgetExt, EntryExt, EntryBufferExtManual};
use relm4::{send, Sender, MicroComponent, WidgetPlus, MicroModel, MicroWidgets};
use crate::models::task::Task;
use crate::services::local::tasks::post_task;

#[derive(Debug)]
pub struct ContentModel {
    pub(crate) list_id: String,
    pub(crate) tasks: Vec<MicroComponent<Task>>
}

pub enum ContentMsg {
    AddTaskEntry(String)
}

impl MicroModel for ContentModel {
    type Msg = ContentMsg;
    type Widgets = ContentWidgets;
    type Data = ();

    fn update(&mut self, msg: Self::Msg, _data: &Self::Data, _sender: Sender<Self::Msg>) {
        let id = &self.list_id.to_owned();
        match msg {
            ContentMsg::AddTaskEntry(title) => {
                post_task(id.to_owned(), title.clone()).expect("Failed to post task.");
                self.tasks.push(MicroComponent::new(Task::new(title, id.to_owned()), ()))
            }
        }
    }
}

#[relm4::micro_widget(pub)]
#[derive(Debug)]
impl MicroWidgets<ContentModel> for ContentWidgets {
    view! {
        task_container = &gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            append = &gtk::Box {
                append: main_stack = &gtk::Stack {
                    add_child = &gtk::ScrolledWindow {
                        set_vexpand: true,
                        set_hexpand: true,
                        set_child: list_box = Some(&gtk::Box) {
                            set_vexpand: true,
                            set_hexpand: true,
                            append: task_list = &gtk::ListBox {
                                set_hexpand: true,
                                append: iterate! {
                                    model.tasks.iter().map(|task| {
                                        task.root_widget() as &gtk::Box
                                    }).collect::<Vec<&gtk::Box>>()
                                }
                            },
                        }
                    },
                },
            },
            append = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_margin_all: 12,
                append: entry = &gtk::Entry {
                    set_hexpand: true,
                    set_icon_from_icon_name: args!(gtk::EntryIconPosition::Primary, Some("list-add-symbolic")),
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
    fn post_view() {
        for task in &model.tasks {
            if !task.is_connected() {
                self.task_list.append(task.root_widget())
            }
        }
    }
}