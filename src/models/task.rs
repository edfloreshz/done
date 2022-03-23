use gtk::prelude::{
    BoxExt, CheckButtonExt, EntryBufferExtManual, EntryExt, OrientableExt, WidgetExt,
};
use relm4::factory::{FactoryPrototype, FactoryVec};
use relm4::{gtk, send, Model, Sender, WidgetPlus, Widgets, ComponentUpdate};
use crate::List;
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
    hbox: gtk::Box
}

impl FactoryPrototype for Task {
    type Factory = FactoryVec<Task>;
    type Widgets = TaskWidgets;
    type Root = gtk::Box;
    type View = gtk::ListBox;
    type Msg = TaskMsg;

    fn init_view(&self, key: &usize, sender: Sender<Self::Msg>) -> Self::Widgets {
        let hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        let checkbox = gtk::CheckButton::builder().active(false).build();
        let label = gtk::Label::new(Some(&self.name));

        assert!(!self.completed);

        checkbox.set_margin_all(12);
        label.set_margin_all(12);

        hbox.append(&checkbox);
        hbox.append(&label);

        let index = *key;
        checkbox.connect_toggled(move |checkbox| {
            send!(sender, TaskMsg::SetCompleted((index, checkbox.is_active())));
        });

        TaskWidgets { label, hbox }
    }

    fn position(&self, _key: &usize) {}

    fn view(&self, _key: &usize, widgets: &Self::Widgets) {
        let attrs = widgets.label.attributes().unwrap_or_default();
        attrs.change(gtk::pango::AttrInt::new_strikethrough(self.completed));
        widgets.label.set_attributes(Some(&attrs));
    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.hbox
    }
}

pub struct TaskModel {
    tasks: FactoryVec<Task>,
}

impl Model for TaskModel {
    type Msg = TaskMsg;
    type Widgets = TaskModelWidgets;
    type Components = ();
}

impl ComponentUpdate<ListModel> for TaskModel {
    fn init_model(parent_model: &ListModel) -> Self {
        TaskModel { tasks: FactoryVec::from_vec(parent_model.lists[0].clone().tasks) }
    }

    fn update(&mut self, msg: Self::Msg, components: &Self::Components, sender: Sender<Self::Msg>, parent_sender: Sender<ListMsg>) {
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

#[relm4::widget(pub)]
impl Widgets<TaskModel, ListModel> for TaskModelWidgets {
    view! {
        vbox = Some(&gtk::Box) {
            set_orientation: gtk::Orientation::Vertical,
            set_margin_all: 12,
            set_spacing: 6,
            set_hexpand: true,

            append = &gtk::ScrolledWindow {
                set_hscrollbar_policy: gtk::PolicyType::Never,
                set_min_content_height: 360,
                set_vexpand: true,
                set_child = Some(&gtk::ListBox) {
                    factory!(model.tasks),
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