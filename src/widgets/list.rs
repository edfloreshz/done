use crate::models::list::QueryableList;
use crate::widgets::sidebar::SidebarMsg;
use glib::Sender;
use gtk4 as gtk;
use gtk4::prelude::{BoxExt, OrientableExt, WidgetExt};
use relm4::{MicroModel, MicroWidgets};
use uuid::Uuid;

#[tracker::track]
#[derive(Default)]
pub struct List {
    pub id_list: String,
    pub display_name: String,
    pub is_owner: bool,
    pub count: i32,
    pub icon_name: String,
}

impl List {
    pub fn new(display_name: &str, icon_name: &str, count: i32) -> Self {
        Self {
            id_list: Uuid::new_v4().to_string(),
            display_name: display_name.to_string(),
            is_owner: true,
            count,
            icon_name: icon_name.to_string(),
            tracker: 0,
        }
    }
}

impl From<QueryableList> for List {
    fn from(list: QueryableList) -> Self {
        Self {
            id_list: list.id_list,
            display_name: list.display_name,
            is_owner: list.is_owner,
            count: list.count,
            icon_name: list.icon_name,
            tracker: 0,
        }
    }
}

impl From<&QueryableList> for List {
    fn from(list: &QueryableList) -> Self {
        Self {
            id_list: list.id_list.clone(),
            display_name: list.display_name.clone(),
            is_owner: list.is_owner,
            count: list.count,
            icon_name: list.icon_name.clone(),
            tracker: 0,
        }
    }
}

impl MicroModel for List {
    type Msg = SidebarMsg;
    type Widgets = ListWidgets;
    type Data = ();

    fn update(&mut self, msg: Self::Msg, _data: &Self::Data, _sender: Sender<Self::Msg>) {
        self.reset();
        match msg {
            SidebarMsg::Delete(_) => {}
            SidebarMsg::AddList(_) => {}
            SidebarMsg::ListSelected(_) => {
                println!("MicroComponentUpdate")
            }
            SidebarMsg::Rename(_, _) => {}
            SidebarMsg::IncreaseCounter(_) => {}
            SidebarMsg::DecreaseCounter(_) => {}
        }
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
                set_text: track!(model.changed(List::count()), model.count.to_string().as_str()),
                set_margin_top: 10,
                set_margin_bottom: 10,
                set_margin_start: 15,
                set_margin_end: 15,
            },
        }
    }
}
