use relm4::gtk::glib;
use relm4::gtk::subclass::prelude::*;
use crate::gtk;

#[derive(Default)]
pub struct TaskList {
    title: String,
    icon: gtk::Image,
    delete: gtk::Button
}
