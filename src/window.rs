use relm4::gtk as gtk;
use relm4::adw as adw;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

mod imp {
    use relm4::adw::subclass::application_window::AdwApplicationWindowImpl;
    use super::*;

    #[derive(Debug, Default)]
    pub struct DoneWindow;

    #[glib::object_subclass]
    impl ObjectSubclass for DoneWindow {
        const NAME: &'static str = "DoneWindow";
        type Type = super::DoneWindow;
        type ParentType = adw::ApplicationWindow;
    }

    impl ObjectImpl for DoneWindow {}
    impl WidgetImpl for DoneWindow {}
    impl WindowImpl for DoneWindow {}
    impl ApplicationWindowImpl for DoneWindow {}
    impl AdwApplicationWindowImpl for DoneWindow {}
}

glib::wrapper! {
    pub struct DoneWindow(ObjectSubclass<imp::DoneWindow>)
        @extends gtk::Widget, gtk::Window, adw::Window, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DoneWindow {
    pub fn new<P: glib::IsA<adw::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create DoneWindow")
    }
}