use gtk4::glib::Type;
use gtk4::prelude::TreeViewExt;
use gtk::prelude::{
    BoxExt
};
use relm4::{gtk, Model, WidgetPlus, Widgets, AppUpdate, ComponentUpdate, RelmComponent, Components};
use crate::{AppModel, AppMsg, Sender};
use crate::models::task::{Task, TaskModel};

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

pub struct ListModel {
    pub lists: Vec<List>
}

pub struct ListWidgets {
    tree_view: gtk::TreeView
}

impl Model for ListModel {
    type Msg = ListMsg;
    type Widgets = ListWidgets;
    type Components = ListComponents;
}

pub struct ListComponents {
    tasks: RelmComponent<TaskModel, ListModel>
}

impl Components<ListModel> for ListComponents {
    fn init_components(parent_model: &ListModel, parent_sender: Sender<ListMsg>) -> Self {
        ListComponents { tasks: RelmComponent::new(parent_model, parent_sender) }
    }

    fn connect_parent(&mut self, _parent_widgets: &ListWidgets) {
        todo!()
    }
}

impl ComponentUpdate<AppModel> for ListModel {
    fn init_model(parent_model: &AppModel) -> Self {
        ListModel { lists: parent_model.lists.clone()}
    }

    fn update(&mut self, msg: Self::Msg, components: &Self::Components, sender: Sender<Self::Msg>, parent_sender: Sender<AppMsg>) {
        match msg {
            ListMsg::Delete(index) => {}
            ListMsg::Create(name) => {}
            ListMsg::Select(index) => {},
            ListMsg::Rename(index, name) => {}
        }
    }
}

impl Widgets<ListModel, AppModel> for ListWidgets {
    type Root = gtk::TreeView;

    fn init_view(model: &ListModel, components: &ListComponents, sender: Sender<ListMsg>) -> Self {
        let tree_view = gtk::TreeView::builder()
            .width_request(200)
            .headers_visible(false)
            .level_indentation(12)
            .can_focus(true)
            .visible(true)
            .show_expanders(true)
            .build();

        let column = gtk::TreeViewColumn::builder().title("List").build();
        tree_view.append_column(&column);
        let list_store = gtk::TreeStore::new(&[Type::STRING]);
        tree_view.set_model(Some(&list_store));
        append_text_column(&tree_view);

        for list in model.lists.iter() {
            list_store.insert_with_values(None, Some(0), &[(0, &list.name)]);
        }

        ListWidgets { tree_view }
    }

    fn root_widget(&self) -> Self::Root {
        self.tree_view.clone()
    }

    fn view(&mut self, model: &ListModel, sender: Sender<ListMsg>) {
        todo!()
    }
}

fn append_text_column(tree: &gtk::TreeView) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}