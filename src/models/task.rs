use gtk::prelude::{
    BoxExt, CheckButtonExt, EntryBufferExtManual, EntryExt, OrientableExt, WidgetExt,
};
use relm4::factory::{Factory, FactoryPrototype, FactoryVec};
use relm4::{gtk, send, Model, Sender, WidgetPlus, Widgets, ComponentUpdate, MicroComponent, MicroModel, MicroWidgets};
use crate::models::list::{ListModel, ListMsg};

pub enum TaskMsg {
    SetCompleted((usize, bool)),
    AddEntry(String),
}

#[derive(Clone)]
pub struct Task {
    pub(crate) name: String,
    pub(crate) completed: bool,
}

#[derive(Debug)]
pub struct TaskWidgets {
    label: gtk::Label,
    container: gtk::Box
}

impl FactoryPrototype for Task {
    type Factory = FactoryVec<Task>;
    type Widgets = TaskWidgets;
    type Root = gtk::Box;
    type View = gtk::ListBox;
    type Msg = TaskMsg;

    fn init_view(&self, key: &<Self::Factory as Factory<Self, Self::View>>::Key, sender: Sender<Self::Msg>) -> Self::Widgets {
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        let checkbox = gtk::CheckButton::builder().active(false).build();
        let label = gtk::Label::new(Some(&self.name));

        assert!(!self.completed);

        checkbox.set_margin_all(12);
        label.set_margin_all(12);

        container.append(&checkbox);
        container.append(&label);

        let index = *key;
        checkbox.connect_toggled(move |checkbox| {
            send!(sender, TaskMsg::SetCompleted((index, checkbox.is_active())));
        });

        TaskWidgets { label, container }
    }

    fn position(&self, _index: &usize) {}

    // view! {
    //     container = gtk::Box {
    //         set_orientation: gtk::Orientation::Horizontal,
    //
    //         append = &gtk::CheckButton {
    //             set_active: false,
    //             set_margin_all: 12,
    //
    //             connect_toggled(key) => move |checkbox| {
    //                 send!(sender, AppMsg::SetCompleted((key, checkbox.is_active())));
    //             }
    //         },
    //         append = &gtk::Label {
    //             set_label: &self.name,
    //             set_margin_all: 12
    //         }
    //     }
    // }

    fn view(&self, key: &usize, widgets: &Self::Widgets) {
        let attrs = widgets.label.attributes().unwrap_or_default();
        attrs.change(gtk::pango::AttrInt::new_strikethrough(self.completed));
        widgets.label.set_attributes(Some(&attrs));
    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.container
    }
}

pub struct TaskModel {
    pub(crate) tasks: FactoryVec<Task>
}

impl MicroModel for TaskModel {
    type Msg = TaskMsg;
    type Widgets = TaskModelWidgets;
    type Data = ();

    fn update(&mut self, msg: Self::Msg, data: &Self::Data, sender: Sender<Self::Msg>) {
        match msg {
            TaskMsg::SetCompleted((index, completed)) => {
                if let Some(task) = self.tasks.get_mut(index) {
                    task.completed = completed;
                }
            }
            TaskMsg::AddEntry(name) => {
                self.tasks.push(Task {
                    name,
                    completed: false,
                });
            }
        }
    }
}

#[relm4::micro_widget(pub)]
#[derive(Debug)]
impl MicroWidgets<TaskModel> for TaskModelWidgets {
    view! {
        container = &gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_hexpand: true,
            set_margin_all: 12,
            set_spacing: 6,

            append = &gtk::ScrolledWindow {
                set_hscrollbar_policy: gtk::PolicyType::Never,
                set_min_content_height: 360,
                set_vexpand: true,
                set_child = Some(&gtk::ListBox) {
                    factory!(model.tasks)
                }
            },
            append = &gtk::Entry {
                connect_activate(sender) => move |entry| {
                    let buffer = entry.buffer();
                    send!(sender, TaskMsg::AddEntry(buffer.text()));
                    buffer.delete_text(0, None);
                }
            }
        }
    }
}