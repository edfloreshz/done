use relm4::{
    ComponentUpdate,
    gtk::prelude::{OrientableExt, WidgetExt}, Model, Sender, view, Widgets,
};
use relm4::gtk;

use crate::AppModel;
use crate::widgets::app::AppMsg;

pub struct DetailsModel {}

pub enum DetailsMsg {}

impl Model for DetailsModel {
    type Msg = DetailsMsg;
    type Widgets = DetailsWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for DetailsModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        DetailsModel {}
    }

    fn update(
        &mut self,
        _msg: Self::Msg,
        _components: &Self::Components,
        _sender: Sender<Self::Msg>,
        _parent_sender: Sender<AppMsg>,
    ) {
        todo!()
    }
}

#[derive(Clone)]
pub struct DetailsWidgets {
    pub revealer: gtk::Revealer,
    pub navigation_box: gtk::Box,
}

impl DetailsWidgets {
    pub fn new() -> Self {
        let navigation_box = Self::create_navigation_box();
        let revealer = Self::create_revealer(&navigation_box);
        revealer.set_child(Some(&navigation_box));
        Self {
            revealer,
            navigation_box,
        }
    }
    fn create_navigation_box() -> gtk::Box {
        view! {
            navigation_box = gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_width_request: 350,
            }
        }
        navigation_box
    }
    fn create_revealer(navigation_box: &relm4::gtk::Box) -> gtk::Revealer {
        view! {
            revealer = gtk::Revealer {
                set_child: Some(navigation_box),
                set_transition_type: gtk::RevealerTransitionType::SlideLeft
            }
        }
        revealer
    }
}

impl Widgets<DetailsModel, AppModel> for DetailsWidgets {
    type Root = gtk::Revealer;

    fn init_view(_model: &DetailsModel, _components: &(), _sender: Sender<DetailsMsg>) -> Self {
        DetailsWidgets::new()
    }

    fn root_widget(&self) -> Self::Root {
        self.revealer.clone()
    }

    fn view(&mut self, _model: &DetailsModel, _sender: Sender<DetailsMsg>) {}
}
