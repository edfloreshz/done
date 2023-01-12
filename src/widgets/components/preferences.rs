use relm4::ComponentParts;
use relm4::{gtk, adw};
use relm4::adw::prelude::{
    AdwWindowExt,
    PreferencesPageExt,
    PreferencesGroupExt,
    PreferencesRowExt,
    ExpanderRowExt
};
use relm4::gtk::prelude::{BoxExt, OrientableExt};
use relm4::{
    Component,
    ComponentSender
};

pub struct Preferences {
}

#[relm4::component(pub)]
impl Component for Preferences {
    type CommandOutput = ();
    type Input = ();
    type Output = ();
    type Init = ();

    view! {
        adw::PreferencesWindow {
            #[wrap(Some)]
            set_content = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                append = &adw::HeaderBar {
                    set_show_end_title_buttons: true
                },
                append = &adw::Clamp {
                    #[wrap(Some)]
                    set_child = &adw::PreferencesPage {
                        add = &adw::PreferencesGroup {
                            set_title: "Providers",
                            add = &adw::ExpanderRow {
                                set_title: "Local",
                                set_subtitle: "Local task provider",
                                set_show_enable_switch: true
                            },
                            add = &adw::ExpanderRow {
                                set_title: "Google",
                                set_subtitle: "Google Task provider",
                                set_show_enable_switch: true
                            },
                            add = &adw::ExpanderRow {
                                set_title: "Microsoft",
                                set_subtitle: "Microsoft To Do provider",
                                set_show_enable_switch: true
                            },
                            add = &adw::ExpanderRow {
                                set_title: "Nextcloud",
                                set_subtitle: "Nextcloud Tasks provider",
                                set_show_enable_switch: true
                            },
                        },
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = Preferences {};
        let widgets = view_output!();
        ComponentParts {
            model, widgets
        }
    }
}