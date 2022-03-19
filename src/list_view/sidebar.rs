use relm4::factory::FactoryVec;

use super::{list::List, group::Group};

struct Sidebar {
    lists: FactoryVec<List>,
    groups: FactoryVec<Group>
}

