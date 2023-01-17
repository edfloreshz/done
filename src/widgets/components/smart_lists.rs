
use relm4::{gtk::{self, traits::OrientableExt}, SimpleComponent, ComponentParts, ComponentSender, factory::AsyncFactoryVecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::widgets::factory::smart_list::SmartListFactory;

#[derive(Debug)]
pub struct SmartListModel {
    smart_list_controller: AsyncFactoryVecDeque<SmartListFactory>
}

#[derive(Debug)]
pub enum SmartListInput {
    SelectSmartList(SmartList),
    Forward
}

#[derive(Debug)]
pub enum SmartListOutput {
    SelectSmartList(SmartList)
}

#[derive(Debug, EnumIter)]
pub enum SmartList {
    All,
    Today,
    Starred,
    Next7Days
}

impl SmartList {
    pub fn list() -> Vec<Self> {
        SmartList::iter().collect()
    }

    pub fn name(&self) -> &str {
        match self {
            SmartList::All => "All",
            SmartList::Today => "Today",
            SmartList::Starred => "Starred",
            SmartList::Next7Days => "Next 7 Days",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            SmartList::All => "All tasks",
            SmartList::Today => "Tasks due today",
            SmartList::Starred => "Starred tasks",
            SmartList::Next7Days => "Tasks due the next 7 days",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            SmartList::All => "edit-paste-symbolic",
            SmartList::Today => "sun-alt-symbolic",
            SmartList::Starred => "star-outline-rounded-symbolic",
            SmartList::Next7Days => "org.gnome.Calendar.Devel-symbolic",
        }
    }
}

#[relm4::component(pub)]
impl SimpleComponent for SmartListModel {
    type Input = SmartListInput;

    type Output = SmartListOutput;

    type Init = ();

    view! {
        #[root]
        gtk::Box {
            #[local_ref]
            smart_list_container -> gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let mut model = SmartListModel { 
            smart_list_controller: AsyncFactoryVecDeque::new(
                gtk::Box::default(),
                sender.input_sender()
            )
        };
        let smart_list_container = model.smart_list_controller.widget();
        let widgets = view_output!();
        for smart_list in SmartList::list() {
            model.smart_list_controller.guard().push_back(smart_list);
        }
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            SmartListInput::SelectSmartList(list) => sender.output(SmartListOutput::SelectSmartList(list)).unwrap(),
            SmartListInput::Forward => todo!(),
        }
    }
}