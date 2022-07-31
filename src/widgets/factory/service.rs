use adw::prelude::ExpanderRowExt;
use adw::prelude::PreferencesGroupExt;
use adw::prelude::PreferencesRowExt;
use diesel::SqliteConnection;
use gtk::prelude::BoxExt;
use gtk::prelude::ListBoxRowExt;
use gtk::prelude::OrientableExt;
use gtk::prelude::WidgetExt;
use relm4::adw;
use relm4::factory::{DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryVecDeque, FactoryView};
use relm4::WidgetPlus;
use relm4::{gtk, Sender};
use crate::data::models::generic::lists::GenericList;
use crate::data::models::generic::tasks::GenericTask;
use crate::data::plugins::local::LocalProvider;

use crate::data::traits::provider::{Provider, Service};
use crate::widgets::component::sidebar::SidebarInput;

#[derive(Debug)]
pub struct ServiceModel {
	pub provider: LocalProvider,
	pub lists: Option<FactoryVecDeque<GenericList>>,
	pub tasks: Option<FactoryVecDeque<GenericTask>>
}

#[derive(Debug)]
pub enum ServiceInput {
	UpdateService,
	RemoveList(DynamicIndex),
}

#[derive(Debug)]
pub enum ServiceOutput {}

#[relm4::factory(pub)]
impl FactoryComponent for ServiceModel {
	type ParentMsg = SidebarInput;
	type ParentWidget = gtk::Box;
	type CommandOutput = ();
	type Input = ServiceInput;
	type Output = ServiceOutput;
	type InitParams = ServiceModel;
	type Widgets = ProviderWidgets;

	view! {
			#[name = "list_box"]
			adw::PreferencesGroup {
					#[name = "expander"]
					add = &adw::ExpanderRow {
							set_title: self.provider.get_name(),
							add_prefix: &self.provider.get_icon(),
							add_action: &gtk::Button::from_icon_name("accessories-text-editor-symbolic")
					}
			}
	}

	fn init_model(
		params: Self::InitParams,
		index: &DynamicIndex,
		sender: FactoryComponentSender<Self>,
	) -> Self {
		params
	}

	fn update(
		&mut self,
		message: Self::Input,
		sender: FactoryComponentSender<Self>,
	) {
		match message {
			ServiceInput::UpdateService => {
				todo!("Update lists")
			}
			ServiceInput::RemoveList(_) => {}
		}
	}

	fn init_widgets(
		&mut self,
		index: &DynamicIndex,
		root: &Self::Root,
		returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: FactoryComponentSender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		let fac = FactoryVecDeque::new(
			widgets.expander.clone(),
			&sender.input,
		);
		self.lists = Some(fac);
		todo!("Iter list and create row.");
		// for list in self.lists.unwrap() {
		// 	relm4::view! {
		// 			#[name = "nested"]
		// 			&adw::ActionRow {
		// 					set_title: &list.display_name
		// 			}
		// 	}
		// 	widgets.expander.add_row(&nested)
		// }
		widgets
	}
}
