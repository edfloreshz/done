use glib::Sender;
use relm4::{adw, ComponentUpdate, gtk, gtk::prelude::{BoxExt, CheckButtonExt}, Model, send, Widgets};
use relm4::adw::ColorScheme;
use relm4::adw::ffi::{
    ADW_COLOR_SCHEME_DEFAULT,
    ADW_COLOR_SCHEME_PREFER_DARK,
    adw_style_manager_get_default,
    adw_style_manager_set_color_scheme,
};

use crate::adw::ffi::ADW_COLOR_SCHEME_PREFER_LIGHT;
use crate::widgets::sidebar::{SidebarModel, SidebarMsg};

pub struct ThemeSelector {
    selected_theme: adw::ColorScheme,
}

impl Model for ThemeSelector {
    type Msg = ThemeSelectorMsg;
    type Widgets = ThemeSelectorWidgets;
    type Components = ();
}

pub enum ThemeSelectorMsg {
    SetPrefersLight,
    SetPrefersDark,
    SetFollowSystem,
}

impl ComponentUpdate<SidebarModel> for ThemeSelector {
    fn init_model(parent_model: &SidebarModel) -> Self {
        Self {
            selected_theme: ColorScheme::Default
        }
    }

    fn update(&mut self, msg: Self::Msg, components: &Self::Components, sender: Sender<Self::Msg>, parent_sender: Sender<SidebarMsg>) {
        unsafe {
            let style_manager = adw_style_manager_get_default();
            match msg {
                ThemeSelectorMsg::SetPrefersLight => {
                    adw_style_manager_set_color_scheme(style_manager, ADW_COLOR_SCHEME_PREFER_LIGHT);
                }
                ThemeSelectorMsg::SetPrefersDark => {
                    adw_style_manager_set_color_scheme(style_manager, ADW_COLOR_SCHEME_PREFER_DARK);
                }
                ThemeSelectorMsg::SetFollowSystem => {
                    adw_style_manager_set_color_scheme(style_manager, ADW_COLOR_SCHEME_DEFAULT);
                }
            }
        }
    }
}

#[relm4_macros::widget(pub)]
impl Widgets<ThemeSelector, SidebarModel> for ThemeSelectorWidgets {
    view! {
        popover = &gtk::Box {
            append = &gtk::CheckButton {
                connect_toggled(sender) => move |_| {
                    send!(sender, ThemeSelectorMsg::SetPrefersLight)
                }
            },
            append = &gtk::CheckButton {
                connect_toggled(sender) => move |_| {
                    send!(sender, ThemeSelectorMsg::SetPrefersLight)
                }
            }
        }
    }
}
