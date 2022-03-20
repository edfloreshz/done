use gtk::prelude::{
    BoxExt, CheckButtonExt, EntryBufferExtManual, EntryExt, OrientableExt, WidgetExt,
};
use relm4::factory::{FactoryVec, FactoryPrototype};
use relm4::{ComponentUpdate, gtk, Model, WidgetPlus, Widgets};
use cascade::cascade;
use relm4_macros::view;
use crate::{AppModel, AppMsg, Sender};

pub enum ListMsg {
    Delete(usize),
    Create(String),
    Select(usize),
    Rename(usize, String),
}

pub struct List {
    pub name: String,
}

#[derive(Debug)]
pub struct ListWidgets {
    name: gtk::Label,
    hbox: gtk::Box,
}

impl FactoryPrototype for List {
    type Factory = FactoryVec<List>;
    type Widgets = ListWidgets;
    type Root = gtk::Box;
    type View = gtk::ListBox;
    type Msg = ListMsg;

    fn init_view(
        &self,
        key: &usize,
        sender: relm4::Sender<Self::Msg>,
    ) -> Self::Widgets {
        let name = cascade! {
            gtk::Label::new(Some(&self.name));
            ..set_margin_all(12);
        };

        let hbox = cascade! {
            gtk::Box::builder().orientation(gtk::Orientation::Horizontal).build();
            ..append(&name);
        };
        
        ListWidgets { name, hbox }
    }

    fn position(&self, _key: &usize) {}

    fn view(&self, _key: &usize, widgets: &Self::Widgets) {

    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.hbox
    }
}

pub struct ListsModel {
    list: FactoryVec<List>
}

impl Model for ListsModel {
    type Msg = ListMsg;
    type Widgets = ListsWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for ListsModel {
    fn init_model(parent_model: &AppModel) -> Self {
        ListsModel {
            list: FactoryVec::from_vec(vec![
                List {
                    name: "Shopping 🛍️".into()
                },
                List {
                    name: "Projects 🖥️".into()
                },
                List {
                    name: "Work 💼".into()
                }
            ])
        }
    }

    fn update(&mut self, msg: Self::Msg, components: &Self::Components, sender: Sender<Self::Msg>, parent_sender: Sender<AppMsg>) {
        match msg {
            ListMsg::Delete(index) => {}
            ListMsg::Create(name) => {}
            ListMsg::Select(index) => {}
            ListMsg::Rename(index, name) => {}
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<ListsModel, AppModel> for ListsWidgets {
    view! {
        vbox = Some(&gtk::Box) {

            append = &gtk::ScrolledWindow {
                set_hscrollbar_policy: gtk::PolicyType::Never,
                set_min_content_height: 360,
                set_vexpand: true,
                set_child = Some(&gtk::ListBox) {
                    factory!(model.list),
                }
            },
        }
    }
}