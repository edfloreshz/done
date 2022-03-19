use gtk::prelude::{ BoxExt };
use relm4::factory::{FactoryVec, FactoryPrototype};
use relm4::{gtk, WidgetPlus};
use cascade::cascade;

pub enum ListMsg {
    Delete(usize),
    Create(String),
    Select(usize),
    Rename(usize, String),
}

pub struct List {
    name: String,
}

#[derive(Debug)]
pub struct ListWidgets {
    name: gtk::Label,
    hbox: gtk::Box,
}

impl FactoryPrototype for List {
    type Root = gtk::ListBox;
    type Msg = ListMsg;
    type Factory = FactoryVec<List>;
    type Widgets = ListWidgets;
    type View = gtk::Box;

    fn init_view(
        &self,
        key: &usize,
        sender: relm4::Sender<Self::Msg>,
    ) -> Self::Widgets {
        let name = cascade! {
            gtk::Label::new(Some(&self.name));
            ..set_margin_all(12);
        };

        let hbox = cascade! {
            gtk::Box::builder().orientation(gtk::Orientation::Horizontal).build();
            ..append(&name);
        };
        
        ListWidgets { name, hbox }
    }

    fn position(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
    ) -> <Self::View as relm4::factory::FactoryView<Self::Root>>::Position {
        todo!();
    }

    fn view(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
        widgets: &Self::Widgets,
    ) {
        todo!();
    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        todo!()
    }
}