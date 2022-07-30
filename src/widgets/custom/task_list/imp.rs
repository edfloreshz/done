use relm4::adw::subclass::prelude::ExpanderRowImpl;
use relm4::gtk::glib;
use relm4::gtk::subclass::prelude::*;
use crate::adw::subclass::prelude::PreferencesRowImpl;
use crate::gtk;

#[derive(Default)]
pub struct Expandible {
    expanded: bool
}