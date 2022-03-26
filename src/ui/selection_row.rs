use gtk4::{
    glib::{self, prelude::*, Object, Value},
    subclass::prelude::*,
};
use std::cell::RefCell;

glib::wrapper! {
	pub struct ListBoxSelectionRow(ObjectSubclass<ListBoxSelectionRowImp>)
		@extends gtk4::ListBoxRow, gtk4::Widget,
		@implements gtk4::Accessible, gtk4::Actionable;
}

impl ListBoxSelectionRow {
    pub fn new(row_id: String) -> Self {
        Object::new(&[("row-id", &row_id), ("subsection", &false)])
            .expect("Failed to create `ListBoxSelectionRow`.")
    }

    pub fn row_id(&self) -> String {
        self.property::<String>("row-id")
    }

    pub fn subsection(&self) -> bool {
        self.property::<bool>("subsection")
    }

    pub fn set_subsection(&self, subsection: bool) {
        self.set_property("subsection", subsection);
    }
}

#[derive(Debug, Default)]
pub struct ListBoxSelectionRowImp {
    row_id: RefCell<String>,
    subsection: RefCell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for ListBoxSelectionRowImp {
    const NAME: &'static str = "ListBoxSelectionRow";
    type Type = ListBoxSelectionRow;
    type ParentType = gtk4::ListBoxRow;
    type Interfaces = ();
    type Instance = glib::subclass::basic::InstanceStruct<Self>;
    type Class = glib::subclass::basic::ClassStruct<Self>;

    fn class_init(klass: &mut Self::Class) {
        // The layout manager determines how child widgets are laid out.
        klass.set_layout_manager_type::<gtk4::BinLayout>();
    }
}

impl ObjectImpl for ListBoxSelectionRowImp {
    fn properties() -> &'static [glib::ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecString::new(
                    "row-id",
                    "row id",
                    "row id",
                    None,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "subsection",
                    "subsection",
                    "subsection",
                    false,
                    glib::ParamFlags::READWRITE,
                ),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "row-id" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.row_id.replace(input_value);
            }
            "subsection" => {
                let input_value = value.get().expect("The value needs to be of type `bool`.");
                self.subsection.replace(input_value);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> Value {
        match pspec.name() {
            "row-id" => self.row_id.borrow().to_value(),
            "subsection" => self.subsection.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for ListBoxSelectionRowImp {}

impl gtk4::subclass::list_box_row::ListBoxRowImpl for ListBoxSelectionRowImp {}
