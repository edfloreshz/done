use crate::schema::lists;
use diesel::{Insertable, Queryable};
use glib::Sender;
use gtk4 as gtk;
use gtk4::prelude::{BoxExt, OrientableExt, WidgetExt};
use relm4::{MicroModel, MicroWidgets};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[table_name = "lists"]
pub struct List {
    pub id_list: String,
    pub display_name: String,
    pub is_owner: bool,
    pub count: i32,
    pub icon_name: String,
}

impl List {
    pub fn new(display_name: &str, icon_name: &str) -> Self {
        Self {
            id_list: Uuid::new_v4().to_string(),
            display_name: display_name.to_string(),
            is_owner: true,
            count: 0,
            icon_name: icon_name.to_string(),
        }
    }
}

impl MicroModel for List {
    type Msg = ();
    type Widgets = ListWidgets;
    type Data = ();

    fn update(&mut self, _msg: Self::Msg, _data: &Self::Data, _sender: Sender<Self::Msg>) {
        todo!()
    }
}

#[relm4::micro_widget(pub)]
#[derive(Debug)]
impl MicroWidgets<List> for ListWidgets {
    view! {
        smart_list_box = &gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            append: icon = &gtk::Image {
                set_from_icon_name: Some(model.icon_name.as_str())
            },
            append: name = &gtk::Label {
                set_halign: gtk::Align::Start,
                set_hexpand: true,
                set_text: model.display_name.as_str(),
                set_margin_top: 10,
                set_margin_bottom: 10,
                set_margin_start: 15,
                set_margin_end: 15,
            },
            append: count = &gtk::Label {
                set_halign: gtk::Align::End,
                set_css_classes: &["dim-label", "caption"],
                set_text: &model.count.to_string(),
                set_margin_top: 10,
                set_margin_bottom: 10,
                set_margin_start: 15,
                set_margin_end: 15,
            },
        }
    }
}
