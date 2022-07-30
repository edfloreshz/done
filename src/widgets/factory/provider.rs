use relm4::{gtk, Sender};
use relm4::adw;
use relm4::factory::{DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryView};
use crate::core::traits::provider::{ProviderService, TaskProvider};
use adw::prelude::PreferencesGroupExt;
use diesel::SqliteConnection;
use crate::widgets::component::sidebar::SidebarInput;
use gtk::prelude::OrientableExt;
use gtk::prelude::WidgetExt;
use adw::prelude::PreferencesRowExt;
use gtk::prelude::ListBoxRowExt;
use gtk::prelude::BoxExt;
use adw::prelude::ExpanderRowExt;
use relm4::WidgetPlus;

#[derive(Debug)]
pub enum ProviderInput {

}

#[derive(Debug)]
pub enum ProviderOutput {

}

#[relm4::factory(pub)]
impl FactoryComponent for Box<dyn ProviderService> {
    type ParentMsg = SidebarInput;
    type ParentWidget = gtk::Box;
    type CommandOutput = ();
    type Input = ProviderInput;
    type Output = ProviderOutput;
    type InitParams = Box<dyn ProviderService>;
    type Widgets = ProviderWidgets;

    view! {
        #[root]
        group = adw::PreferencesGroup {
            set_description: Some(self.get_provider().get_name()),
        }
    }

    fn init_model(params: Self::InitParams, index: &DynamicIndex, sender: &FactoryComponentSender<Self>) -> Self {
        params
    }

    fn update(&mut self, message: Self::Input, sender: &FactoryComponentSender<Self>) {
        match message {

        }
    }

    fn init_widgets(&mut self, index: &DynamicIndex, root: &Self::Root, returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget, sender: &FactoryComponentSender<Self>) -> Self::Widgets {
        let widgets = view_output!();
        for list in self.get_task_lists() {
            relm4::view! {
                #[name = "list_box"]
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    append = &gtk::ListBox {
                        append = &adw::ExpanderRow {
                            set_show_enable_switch: true,
                            set_enable_expansion: true,
                            set_title: "Local",
                            #[wrap(Some)]
                            set_child = &gtk::Box {
                                set_margin_all: 10,
                                append = &gtk::ListBox {
                                    append = &adw::ActionRow {
                                        #[wrap(Some)]
                                        set_child = &gtk::Box {
                                            append: &gtk::Label::new(Some("Testing"))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            widgets.group.add(&list_box)
        }
        widgets
    }
}