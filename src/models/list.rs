use gtk4::glib::Type;
use gtk4::prelude::{TreeModelExt, TreeViewExt};
use gtk::prelude::{
    BoxExt
};
use relm4::{gtk, Model, WidgetPlus, Widgets, AppUpdate, ComponentUpdate, RelmComponent, Components, send};
use crate::{AppModel, AppMsg, Sender};
use crate::models::task::{Task};

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
    pub(crate) lists: Vec<List>
}

impl ComponentUpdate<AppModel> for ListModel {
    fn init_model(parent_model: &AppModel) -> Self {
        ListModel { lists: parent_model.lists.clone() }
    }

    fn update(&mut self, msg: Self::Msg, components: &Self::Components, sender: Sender<Self::Msg>, parent_sender: Sender<AppMsg>) {
        match msg {
            ListMsg::Select(index) => send!(parent_sender, AppMsg::Select(index)),
            _ => {}
        }
    }
}

pub struct ListWidgets {
    tree_view: gtk::TreeView
}

impl Model for ListModel {
    type Msg = ListMsg;
    type Widgets = ListWidgets;
    type Components = ();
}

impl Widgets<ListModel, AppModel> for ListWidgets {
    type Root = gtk::TreeView;

    fn init_view(model: &ListModel, components: &(), sender: Sender<ListMsg>) -> Self {
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

        let selection = tree_view.selection();

        selection.connect_changed(move |tree_view| {
            let (model, iter) = tree_view.selected().expect("Couldn't get selected");
            let path = model.path(&iter);
            send!(sender, ListMsg::Select(path.indices()[0].try_into().unwrap()))
        });

        ListWidgets { tree_view }
    }

    fn root_widget(&self) -> Self::Root {
        self.tree_view.clone()
    }

    fn view(&mut self, model: &ListModel, sender: Sender<ListMsg>) {

    }
}

fn append_text_column(tree: &gtk::TreeView) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}