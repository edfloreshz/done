use gtk::prelude::{
    BoxExt,
    ToggleButtonExt,
    WidgetExt,
};
use relm4::{ComponentParts, ComponentSender, gtk, SimpleComponent};
use relm4::adw::ffi::{
    ADW_COLOR_SCHEME_DEFAULT as Default,
    ADW_COLOR_SCHEME_FORCE_DARK as ForceDark, ADW_COLOR_SCHEME_FORCE_LIGHT as ForceLight,
    adw_style_manager_get_default as get_style_manager, adw_style_manager_set_color_scheme as set_color_scheme,
};

pub struct MainMenuModel;

pub enum MainMenuInput {
    ForceLight,
    ForceDark,
    FollowSystem,
}

#[relm4::component(pub)]
impl SimpleComponent for MainMenuModel {
    type Input = MainMenuInput;
    type Output = ();
    type InitParams = ();
    type Widgets = MenuWidgets;

    view! {
        #[root]
        gtk::Popover {
            #[name = "theme_selector"]
            gtk::Box {
                add_css_class: "theme-container",
                set_spacing: 12,
                append: follow = &gtk::ToggleButton {
                    add_css_class: "follow",
                    add_css_class: "theme-selector",
                    connect_toggled[sender] => move |_| {
                        sender.input(MainMenuInput::FollowSystem)
                    }
                },
                append: light = &gtk::ToggleButton {
                    add_css_class: "light",
                    add_css_class: "theme-selector",
                    set_group: Some(&follow),
                    connect_toggled[sender] => move |_| {
                        sender.input(MainMenuInput::ForceLight)
                    }
                },
                append: dark = &gtk::ToggleButton {
                    add_css_class: "dark",
                    add_css_class: "theme-selector",
                    set_group: Some(&follow),
                    connect_toggled[sender] => move |_| {
                        sender.input(MainMenuInput::ForceDark)
                    }
                }
            },
            // TODO: Figure out a way to include these options in the menu.
            // gio::Menu {
            //     append: (Some("About"), Some("app.about")),
            //     append: (Some("Quit"), Some("app.quit"))
            // }
        }
    }

    fn init(
        _params: Self::InitParams,
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();
        let model = MainMenuModel;
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: &ComponentSender<Self>) {
        unsafe {
            let style_manager = get_style_manager();
            match message {
                MainMenuInput::ForceLight => {
                    set_color_scheme(style_manager, ForceLight);
                }
                MainMenuInput::ForceDark => {
                    set_color_scheme(style_manager, ForceDark);
                }
                MainMenuInput::FollowSystem => {
                    set_color_scheme(style_manager, Default);
                }
            }
        }
    }
}
