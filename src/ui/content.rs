use relm4_macros::view;
use gtk4 as gtk;
use gtk::glib::clone;
use gtk::prelude::*;

#[derive(Clone)]
pub struct MainWidgets {
    pub overlay: gtk::Overlay,
    pub revealer: gtk::Revealer,
    pub stack: gtk::Stack,
    pub label: gtk::Label,
}

impl MainWidgets {
    pub fn new(child: &gtk4::Box) -> Self {
        let stack = Self::create_stack();
        let revealer = Self::create_revealer();
        let button = Self::create_button(&revealer);
        let overlay = gtk::Overlay::builder().child(child).build();
        let label = gtk::Label::builder()
            .halign(gtk::Align::Start)
            .valign(gtk::Align::Start)
            .build();
        let top_box = gtk4::Box::new(gtk::Orientation::Horizontal, 60);
        top_box.append(&label);
        top_box.append(&button);
        let internal_box = gtk4::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        internal_box.append(&top_box);
        internal_box.append(&stack);
        revealer.set_child(Some(&internal_box));
        overlay.add_overlay(&revealer);
        Self {
            stack,
            revealer,
            overlay,
            label,
        }
    }

    fn create_stack() -> gtk::Stack {
        gtk::Stack::builder()
            .margin_top(24)
            .margin_bottom(24)
            .margin_start(24)
            .margin_end(24)
            .hexpand(true)
            .vexpand(true)
            .build()
    }

    fn create_revealer() -> gtk::Revealer {
        gtk::Revealer::builder()
            .halign(gtk::Align::End)
            .valign(gtk::Align::Start)
            .transition_type(gtk::RevealerTransitionType::SlideLeft)
            .margin_end(24)
            .build()
    }

    fn create_button(revealer: &gtk::Revealer) -> gtk::Button {
        let button = gtk::Button::builder()
            .label("Close")
            .halign(gtk::Align::End)
            .valign(gtk::Align::Center)
            .css_classes(vec!["settings-popup-close".into()])
            .build();
        button.connect_clicked(clone!(@weak revealer => move |_| {
			revealer.set_reveal_child(false);
		}));
        button
    }

    pub fn pop_up(&self, name: &str) {
        self.label.set_text(name);
        self.stack.set_visible_child_name(name);
        self.revealer.set_reveal_child(true);
    }

    pub fn add_overlay<W, F>(&self, name: &str, create_overlay: F)
        where
            W: IsA<gtk::Widget>,
            F: FnOnce() -> W,
    {
        let new_overlay = create_overlay();
        self.stack.add_named(&new_overlay, Some(name));
    }
}