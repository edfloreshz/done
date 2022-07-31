use adw::prelude::ExpanderRowExt;
use adw::prelude::PreferencesGroupExt;
use adw::prelude::PreferencesRowExt;
use diesel::SqliteConnection;
use glib::clone;
use gtk::prelude::BoxExt;
use gtk::prelude::ListBoxRowExt;
use gtk::prelude::OrientableExt;
use gtk::prelude::WidgetExt;
use relm4::adw;
use relm4::factory::{
	DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryView,
};
use relm4::gtk::prelude::PopoverExt;
use relm4::WidgetPlus;
use relm4::{gtk, Sender};

use crate::data::plugins::local::LocalProvider;
use crate::data::traits::provider::{Provider, ProviderType, Service};
use crate::gtk::prelude::Cast;
use crate::gtk::Image;
use crate::widgets::component::sidebar::SidebarInput;
use crate::widgets::popover::new_list::NewListOutput;

#[derive(Debug)]
pub enum ProviderInput {}

#[relm4::factory(pub)]
impl FactoryComponent for Box<dyn Provider> {
	type ParentMsg = NewListOutput;
	type ParentWidget = gtk::ListBox;
	type CommandOutput = ();
	type Input = ProviderInput;
	type Output = ();
	type InitParams = Box<dyn Provider>;
	type Widgets = ProviderWidgets;

	view! {
			#[name = "providers_container"]
			gtk::ListBoxRow {
				set_activatable: true,
				#[wrap(Some)]
				set_child = &gtk::Box {
					append = &gtk::Image {
						set_icon_name: Some(&self.get_icon_name())
					},
					append = &gtk::Label {
						set_label: self.get_name()
					}
				}
		}
	}

	fn init_model(
		params: Self::InitParams,
		_index: &DynamicIndex,
		_sender: FactoryComponentSender<Self>,
	) -> Self {
		params
	}

	fn update(
		&mut self,
		_message: Self::Input,
		_sender: FactoryComponentSender<Self>,
	) {
	}

	fn init_widgets(
		&mut self,
		index: &DynamicIndex,
		root: &Self::Root,
		returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: FactoryComponentSender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}
}
