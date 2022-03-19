use relm4::factory::{FactoryVec, FactoryPrototype};
use relm4::gtk;

use super::list::List;

pub enum GroupMsg {
    DeleteGroup(usize),
}

pub struct Group {
    name: String,
    lists: FactoryVec<List>,
}

#[derive(Debug)]
pub struct GroupWidgets {
    name: gtk::Label,
    hbox: gtk::Box,
    lists: gtk::Label,
}

impl FactoryPrototype for Group {
    type Root = gtk::ListBox;
    type Msg = GroupMsg;
    type Factory = FactoryVec<Group>;
    type Widgets = GroupWidgets;
    type View = gtk::Box;

    fn init_view(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
        sender: relm4::Sender<Self::Msg>,
    ) -> Self::Widgets {
        todo!()
    }

    fn position(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
    ) -> <Self::View as relm4::factory::FactoryView<Self::Root>>::Position {
        todo!()
    }

    fn view(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
        widgets: &Self::Widgets,
    ) {
        todo!()
    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        todo!()
    }
}
