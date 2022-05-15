use adw::subclass::prelude::*;
use gtk::{gio, glib, glib::object_subclass};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use relm4::adw;
use relm4::gtk;
use relm4::gtk::glib::clone;

use crate::app::constants::VERSION;
use crate::app::window::DoneWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct DoneApplication {}

    #[object_subclass]
    impl ObjectSubclass for DoneApplication {
        const NAME: &'static str = "DoneApplication";
        type Type = super::DoneApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for DoneApplication {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
            obj.set_accels_for_action("app.about", &["<primary>I"]);
        }
    }

    impl ApplicationImpl for DoneApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self, application: &Self::Type) {
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = DoneWindow::new(application);
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for DoneApplication {}

    impl AdwApplicationImpl for DoneApplication {}
}

glib::wrapper! {
    pub struct DoneApplication(ObjectSubclass<imp::DoneApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DoneApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::new(&[("application-id", &application_id), ("flags", flags)])
            .expect("Failed to create DoneApplication")
    }

    fn setup_gactions(&self) {
        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.quit();
        }));
        self.add_action(&quit_action);

        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about();
        }));
        self.add_action(&about_action);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let dialog = gtk::AboutDialog::builder()
            .comments("Comments")
            .icon_name("dev.edfloreshz.Done")
            .transient_for(&window)
            .modal(true)
            .program_name("Done")
            .version(VERSION)
            .website_label("Website")
            .website("https://done.edfloreshz.dev/")
            .authors(vec!["Eduardo Flores".into()])
            .build();

        dialog.present();
    }
}
