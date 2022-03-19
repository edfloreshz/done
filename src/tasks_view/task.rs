use gtk::prelude::{ BoxExt, CheckButtonExt };
use relm4::factory::{FactoryPrototype, FactoryVec};
use relm4::{gtk, send, AppUpdate, Sender, WidgetPlus};
use cascade::cascade;

use crate::AppModel;

pub enum TaskMsg {
    SetCompleted((usize, bool)),
    AddEntry(String),
}

pub struct Task {
    name: String,
    completed: bool,
}

#[derive(Debug)]
pub struct TaskWidgets {
    label: gtk::Label,
    hbox: gtk::Box,
}

impl FactoryPrototype for Task {
    type View = gtk::ListBox;
    type Msg = TaskMsg;
    type Factory = FactoryVec<Task>;
    type Widgets = TaskWidgets;
    type Root = gtk::Box;

    fn init_view(&self, key: &usize, sender: Sender<Self::Msg>) -> Self::Widgets {
        let index = *key;
        
        let checkbox = cascade! {
            gtk::CheckButton::builder().active(false).build();
            ..set_margin_all(12);
            ..connect_toggled(move |checkbox| {
                send!(sender, TaskMsg::SetCompleted((index, checkbox.is_active())));
            });
        };
        let label = cascade! {
            gtk::Label::new(Some(&self.name));
            ..set_margin_all(12);
        };
        
        assert!(!self.completed);
        
        let hbox = cascade! {
            gtk::Box::builder().orientation(gtk::Orientation::Horizontal).build();
            ..append(&checkbox);
            ..append(&label);
        };

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

impl AppUpdate for AppModel {
    fn update(&mut self, msg: TaskMsg, _components: &(), _sender: Sender<TaskMsg>) -> bool {
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
        true
    }
}