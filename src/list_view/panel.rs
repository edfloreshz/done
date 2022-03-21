use relm4::factory::FactoryVec;

use super::{list::List, group::Group};

enum PanelMsg {

}

struct Sidebar {
    lists: FactoryVec<List>,
    groups: FactoryVec<Group>
}



