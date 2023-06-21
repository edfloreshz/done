use core_done::service::Service;
use libset::{project::Project, format::FileFormat};
use relm4::{
	component::{AsyncComponent, AsyncComponentParts, AsyncController, AsyncComponentController},
	gtk::{self, prelude::{OrientableExt, ButtonExt}, traits::{WidgetExt, BoxExt, GtkWindowExt}}, AsyncComponentSender, factory::AsyncFactoryVecDeque,
    adw, RelmWidgetExt, new_action_group, new_stateless_action
};
use relm4_icons::icon_name;

use crate::{app::{factories::service::ServiceFactoryModel, config::preferences::{Preferences}, components::preferences::PreferencesComponentOutput}, fl};

use super::preferences::PreferencesComponentModel;

pub struct ServicesSidebarModel {
    services_factory: AsyncFactoryVecDeque<ServiceFactoryModel>,
    preferences: AsyncController<PreferencesComponentModel>,
    extended: bool
}

#[derive(Debug)]
pub enum ServicesSidebarInput {
    ServiceSelected(Service),
    ToggleExtended(bool),
    ReloadSidebar,
    OpenPreferences
}

#[derive(Debug)]
pub enum ServicesSidebarOutput {
    ServiceSelected(Service),
}

new_action_group!(pub(super) WindowActionGroup, "win");
new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
new_stateless_action!(AboutAction, WindowActionGroup, "about");
new_stateless_action!(QuitAction, WindowActionGroup, "quit");

#[relm4::component(pub async)]
impl AsyncComponent for ServicesSidebarModel {
	type CommandOutput = ();
	type Input = ServicesSidebarInput;
	type Output = ServicesSidebarOutput;
	type Init = ();

    menu! {
		primary_menu: {
			section! {
				keyboard_shortcuts => ShortcutsAction,
				about_done => AboutAction,
				quit => QuitAction,
			}
		}
	}

	view! {
		#[root]
        gtk::Box {
            set_css_classes: &["view"],
            set_orientation: gtk::Orientation::Vertical,
            #[watch]
            set_width_request: if model.extended { 200 } else { 50 },
            #[name = "services_sidebar_header"]
            adw::HeaderBar {
                #[watch]
                set_visible: model.extended,
                set_css_classes: &["flat"],
                set_show_end_title_buttons: false,
                set_show_start_title_buttons: false,
                pack_start = &gtk::MenuButton {
                    set_tooltip: fl!("menu"),
                    set_valign: gtk::Align::Center,
                    set_css_classes: &["flat"],
                    set_icon_name: icon_name::MENU,
                    set_menu_model: Some(&primary_menu),
                },
                #[wrap(Some)]
                set_title_widget = &gtk::Label {
                    set_hexpand: true,
                    set_text: fl!("done"),
                },
            },
            gtk::CenterBox {
                #[watch]
                set_visible: !model.extended,
                set_height_request: 46,
                set_margin_top: 8,
                set_margin_bottom: 8,
                #[wrap(Some)]
                set_center_widget = &gtk::Box {
                    set_spacing: 5,
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::MenuButton {
                        set_width_request: 42,
                        set_valign: gtk::Align::Center,
                        set_css_classes: &["flat"],
                        set_icon_name: icon_name::MENU,
                        set_menu_model: Some(&primary_menu),
                    },
                },
            },
            gtk::ScrolledWindow {
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_vexpand: true,
                    #[local_ref]
                    services_list -> gtk::ListBox {
                        set_css_classes: &["navigation-sidebar"],
                        connect_row_selected => move |_, listbox_row| {
                            if let Some(row) = listbox_row {
                                row.activate();
                            }
                        },
                    },
                    gtk::CenterBox {
                        set_vexpand: true,
                        set_valign: gtk::Align::End,
                        set_css_classes: &["navigation-sidebar"],
                        set_has_tooltip: true,
                        set_tooltip_text: Some("Preferences"),
                        #[wrap(Some)]
                        set_center_widget = &gtk::Button {
                            set_css_classes: &["flat"],
                            gtk::CenterBox {
                                #[wrap(Some)]
                                set_center_widget = &gtk::Image {
                                    set_icon_name: Some("controls")
                                },
                            },
                            connect_clicked => ServicesSidebarInput::OpenPreferences
                        },
                    }
                }
            }
        }
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
        let keyboard_shortcuts: &str = fl!("keyboard-shortcuts");
		let about_done: &str = fl!("about-done");
		let quit: &str = fl!("quit");
        
        let mut services_factory = AsyncFactoryVecDeque::new(
            gtk::ListBox::default(),
            sender.input_sender(),
        );

        {
			let mut guard = services_factory.guard();
            
			for service in Service::list() {
                guard.push_back(service);
            }
		}

        let current_preferences =
        if let Ok(project) = Project::open("dev", "edfloreshz", "done") {
            project
                .get_file_as::<Preferences>("preferences", FileFormat::JSON)
                .unwrap_or(Preferences::new().await)
        } else {
            Preferences::new().await
        };

        let model = ServicesSidebarModel {
            services_factory,
            preferences: PreferencesComponentModel::builder().launch(()).forward(sender.input_sender(), |message| match message {
                PreferencesComponentOutput::ToggleExtended(extended) => ServicesSidebarInput::ToggleExtended(extended),
            }),
            extended: current_preferences.extended
        };
        
        let services_list = model.services_factory.widget();
		let widgets = view_output!();
		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
            ServicesSidebarInput::ReloadSidebar => {

            }
            ServicesSidebarInput::ToggleExtended(extended) => {
                self.extended = extended;
            }
            ServicesSidebarInput::ServiceSelected(service) => {
                sender
				.output(ServicesSidebarOutput::ServiceSelected(service))
				.unwrap();
            },
            ServicesSidebarInput::OpenPreferences => self.preferences.widget().present(),
        }
	}
}
