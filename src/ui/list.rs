use gtk4 as gtk;
use gtk::glib::Type;
use gtk::prelude::*;

pub struct ListWidgets {
    tree_view: gtk::TreeView
}

impl ListWidgets {
    fn new() -> Self {
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

        // for list in model.lists.iter() {
        //     let container = gtk::Box::builder().hexpand(true).height_request(20).orientation(gtk::Orientation::Vertical).build();
        //     container.append(&gtk::Label::new(Some(&list.display_name)));
        //     list_store.insert_with_values(None, Some(0), &[(0, &list.display_name)]);
        // }

        let selection = tree_view.selection();

        selection.connect_changed(move |tree_view| {
            let (model, iter) = tree_view.selected().expect("Couldn't get selected");
            let path = model.path(&iter);
            // send!(sender, ListMsg::Select(path.indices()[0].try_into().unwrap()))
        });

        ListWidgets { tree_view }
    }
}

fn append_text_column(tree: &gtk::TreeView) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();
    cell.set_height(50);

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}