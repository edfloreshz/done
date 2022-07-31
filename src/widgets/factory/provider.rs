use gtk::prelude::BoxExt;
use gtk::prelude::ListBoxRowExt;
use relm4::factory::{
	DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryView,
};
use relm4::gtk;

use crate::data::traits::provider::Provider;
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
		_index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		_sender: FactoryComponentSender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}
}
