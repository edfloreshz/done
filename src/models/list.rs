use gtk::prelude::{
    BoxExt
};
use relm4::{gtk, Model, WidgetPlus, Widgets, AppUpdate};
use crate::Sender;
use crate::models::task::Task;

pub enum ListMsg {
    Delete(usize),
    Create(String),
    Select(usize),
    Rename(usize, String),
}

#[derive(Clone)]
pub struct List {
    pub name: String,
    pub tasks: Vec<Task>
}

pub struct ListWidgets {
    view: gtk::Box
}

impl Model for List {
    type Msg = ListMsg;
    type Widgets = ListWidgets;
    type Components = ();
}

impl AppUpdate for List {
    fn update(&mut self, msg: Self::Msg, components: &Self::Components, sender: Sender<Self::Msg>) -> bool {
        match msg {
            ListMsg::Delete(index) => {}
            ListMsg::Create(name) => {}
            ListMsg::Select(index) => {},
            ListMsg::Rename(index, name) => {}
        }
        true
    }
}

impl Widgets<List, ()> for ListWidgets {
    type Root = gtk::Box;

    fn init_view(model: &List, _components: &(), sender: Sender<ListMsg>) -> Self {
        let view = gtk::Box::new(gtk::Orientation::Vertical, 6);
        for task in &model.tasks {
            let hbox = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .build();
            let checkbox = gtk::CheckButton::builder().active(false).build();
            let label = gtk::Label::new(Some(&model.name));

            assert!(!task.completed);

            checkbox.set_margin_all(12);
            label.set_margin_all(12);

            hbox.append(&checkbox);
            hbox.append(&label);
            view.append(&hbox);
        }
        ListWidgets { view }
    }

    fn root_widget(&self) -> Self::Root {
        self.view.clone()
    }

    fn view(&mut self, model: &List, sender: Sender<ListMsg>) {
        todo!()
    }
}