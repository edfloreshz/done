use glib::clone;
use gtk4 as gtk;
use gtk4::prelude::{BoxExt, ButtonExt};
use relm4::{ComponentUpdate, Model, Widgets};
use crate::{AppModel, AppMsg, Sender};

pub struct ContentModel {

}

pub enum ContentMsg {

}

impl Model for ContentModel {
    type Msg = ContentMsg;
    type Widgets = ContentWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for ContentModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        ContentModel {}
    }

    fn update(&mut self, msg: Self::Msg, components: &Self::Components, sender: Sender<Self::Msg>, parent_sender: Sender<AppMsg>) {
        todo!()
    }
}

pub struct ContentWidgets {
    pub revealer: gtk::Revealer,
    pub stack: gtk::Stack,
    pub label: gtk::Label,
}

impl ContentWidgets {
    pub fn new() -> Self {
        let stack = Self::create_stack();
        let revealer = Self::create_revealer();
        let button = Self::create_button(&revealer);
        // let overlay = gtk::Overlay::builder().child(child).build();
        let label = gtk::Label::builder()
            .halign(gtk::Align::Start)
            .valign(gtk::Align::Start)
            .build();
        let top_box = gtk4::Box::new(gtk::Orientation::Horizontal, 60);
        top_box.append(&label);
        top_box.append(&button);
        let internal_box = gtk4::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(vec!["main-box".to_string()])
            .build();
        internal_box.append(&top_box);
        internal_box.append(&stack);
        revealer.set_child(Some(&internal_box));
        Self {
            stack,
            revealer,
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
}

impl Widgets<ContentModel, AppModel> for ContentWidgets {
    type Root = gtk::Revealer;

    fn init_view(model: &ContentModel, _components: &(), sender: Sender<ContentMsg>) -> Self {
        ContentWidgets::new()
    }

    fn root_widget(&self) -> Self::Root {
        self.revealer.clone()
    }

    fn view(&mut self, model: &ContentModel, sender: Sender<ContentMsg>) {
        todo!()
    }
}