use adw::prelude::ExpanderRowExt;
use adw::prelude::PreferencesGroupExt;
use adw::prelude::PreferencesRowExt;
use diesel::SqliteConnection;
use gtk::prelude::BoxExt;
use gtk::prelude::ListBoxRowExt;
use gtk::prelude::OrientableExt;
use gtk::prelude::WidgetExt;
use relm4::{gtk, Sender};
use relm4::adw;
use relm4::factory::{DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryView};
use relm4::WidgetPlus;

use crate::data::traits::provider::{ProviderService, TaskProvider};
use crate::widgets::component::sidebar::SidebarInput;

#[derive(Debug)]
pub enum ProviderInput {}

#[derive(Debug)]
pub enum ProviderOutput {}

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
        #[name = "list_box"]
        adw::PreferencesGroup {
            #[name = "expander"]
            add = &adw::ExpanderRow {
                set_title: self.get_provider().get_name(),
                add_prefix: &self.get_provider().get_icon(),
                add_action: &gtk::Button::from_icon_name("accessories-text-editor-symbolic")
            }
        }
    }

    fn init_model(params: Self::InitParams, index: &DynamicIndex, sender: FactoryComponentSender<Self>) -> Self {
        params
    }

    fn update(&mut self, message: Self::Input, sender: FactoryComponentSender<Self>) {
        match message {}
    }

    fn init_widgets(&mut self, index: &DynamicIndex, root: &Self::Root, returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget, sender: FactoryComponentSender<Self>) -> Self::Widgets {
        let widgets = view_output!();

        for list in self.get_task_lists() {
            relm4::view! {
                #[name = "nested"]
                &adw::ActionRow {
                    set_title: &list.display_name
                }
            }
            widgets.expander.add_row(&nested)
        }
        widgets
    }
}