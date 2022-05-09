// use once_cell::sync::OnceCell;
// use tokio::runtime::Runtime;
use relm4::{
    adw,
    adw::prelude::AdwApplicationWindowExt,
    gtk,
    gtk::prelude::{GtkWindowExt, WidgetExt},
    AppUpdate, Components, Model, RelmComponent, Sender, Widgets,
};

// use crate::widgets::details::DetailsModel;
use crate::widgets::sidebar::SidebarModel;

// static RT: OnceCell<Runtime> = OnceCell::new();

pub struct AppModel;

impl AppModel {
    pub fn new() -> Self {
        Self {}
    }
}

pub enum AppMsg {}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for AppModel {
    fn update(
        &mut self,
        _msg: Self::Msg,
        _components: &Self::Components,
        _sender: Sender<Self::Msg>,
    ) -> bool {
        true
    }
}

pub struct AppComponents {
    sidebar: RelmComponent<SidebarModel, AppModel>,
    // details: RelmComponent<DetailsModel, AppModel>,
}

impl Components<AppModel> for AppComponents {
    fn init_components(parent_model: &AppModel, parent_sender: Sender<AppMsg>) -> Self {
        AppComponents {
            sidebar: RelmComponent::new(parent_model, parent_sender.clone()),
            // details: RelmComponent::new(parent_model, parent_sender),
        }
    }

    fn connect_parent(&mut self, _parent_widgets: &AppWidgets) {}
}

#[relm4_macros::widget(pub)]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        window = adw::ApplicationWindow {
            set_default_width: 800,
            set_default_height: 700,
            set_width_request: 460,
            set_height_request: 400,

            set_content: overlay = Some(&gtk::Overlay) {
                set_child: stack = Some(&gtk::Stack) {
                    set_hexpand: true,
                    set_vexpand: true,
                    set_transition_duration: 250,
                    set_transition_type: gtk::StackTransitionType::Crossfade,
                    add_child: &components.sidebar.widgets().unwrap().leaflet
                }
            },
        }
    }
}
