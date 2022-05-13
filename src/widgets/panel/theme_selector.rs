use glib::Sender;
use relm4::adw::ffi::{
    adw_style_manager_get_default as get_style_manager,
    adw_style_manager_set_color_scheme as set_color_scheme, ADW_COLOR_SCHEME_DEFAULT as Default,
    ADW_COLOR_SCHEME_FORCE_DARK as ForceDark, ADW_COLOR_SCHEME_FORCE_LIGHT as ForceLight,
};
use relm4::{
    gtk,
    gtk::prelude::{BoxExt, ToggleButtonExt, WidgetExt},
    send, ComponentUpdate, Model, Widgets,
};

use crate::widgets::panel::sidebar::{SidebarModel, SidebarMsg};

pub struct ThemeSelector;

impl Model for ThemeSelector {
    type Msg = ThemeSelectorMsg;
    type Widgets = ThemeSelectorWidgets;
    type Components = ();
}

pub enum ThemeSelectorMsg {
    ForceLight,
    ForceDark,
    FollowSystem,
}

impl ComponentUpdate<SidebarModel> for ThemeSelector {
    fn init_model(_parent_model: &SidebarModel) -> Self {
        Self
    }

    fn update(
        &mut self,
        msg: Self::Msg,
        _components: &Self::Components,
        _sender: Sender<Self::Msg>,
        _parent_sender: Sender<SidebarMsg>,
    ) {
        unsafe {
            let style_manager = get_style_manager();
            match msg {
                ThemeSelectorMsg::ForceLight => {
                    set_color_scheme(style_manager, ForceLight);
                }
                ThemeSelectorMsg::ForceDark => {
                    set_color_scheme(style_manager, ForceDark);
                }
                ThemeSelectorMsg::FollowSystem => {
                    set_color_scheme(style_manager, Default);
                }
            }
        }
    }
}

#[relm4_macros::widget(pub)]
impl Widgets<ThemeSelector, SidebarModel> for ThemeSelectorWidgets {
    view! {
        theme_selector = &gtk::Box {
            add_css_class: "theme-container",
            set_spacing: 12,
            append: follow = &gtk::ToggleButton {
                add_css_class: "follow",
                add_css_class: "theme-selector",
                connect_toggled(sender) => move |_| {
                    send!(sender, ThemeSelectorMsg::FollowSystem)
                }
            },
            append: light = &gtk::ToggleButton {
                add_css_class: "light",
                add_css_class: "theme-selector",
                set_group: Some(&follow),
                connect_toggled(sender) => move |_| {
                    send!(sender, ThemeSelectorMsg::ForceLight)
                }
            },
            append: dark = &gtk::ToggleButton {
                add_css_class: "dark",
                add_css_class: "theme-selector",
                set_group: Some(&follow),
                connect_toggled(sender) => move |_| {
                    send!(sender, ThemeSelectorMsg::ForceDark)
                }
            }
        }
    }
}
