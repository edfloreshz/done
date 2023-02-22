use adw::prelude::{ActionRowExt, PreferencesGroupExt, PreferencesRowExt};
use relm4::adw;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::gtk;
use relm4::gtk::traits::WidgetExt;
use relm4::loading_widgets::LoadingWidgets;

use crate::widgets::smart_lists::sidebar::model::SmartList;

use super::message::SmartSidebarListInput;

#[derive(Debug)]
pub struct SmartSidebarFactoryListModel {
	pub name: String,
	pub description: String,
	pub icon: String,
	pub smart_list: SmartList,
}

#[derive(Debug)]
pub enum SmartSidebarFactoryListInput {
	SelectSmartList,
}

#[derive(Debug)]
pub enum SmartSidebarFactoryListOutput {
	SelectSmartList(SmartList),
	Forward,
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for SmartSidebarFactoryListModel {
	type ParentInput = SmartSidebarListInput;
	type ParentWidget = gtk::Box;
	type CommandOutput = ();
	type Input = SmartSidebarFactoryListInput;
	type Output = SmartSidebarFactoryListOutput;
	type Init = SmartList;

	view! {
		#[root]
		#[name(list_box)]
		adw::PreferencesGroup {
			#[name(expander)]
			add = &adw::ActionRow {
				#[watch]
				set_title: self.name.as_str(),
				#[watch]
				set_subtitle: self.description.as_str(),
				#[watch]
				set_icon_name: Some(self.icon.as_str()),
			},
			add_controller = gtk::GestureClick {
				connect_pressed[sender] => move |_, _, _, _| {
					sender.input(SmartSidebarFactoryListInput::SelectSmartList);
					sender.output(SmartSidebarFactoryListOutput::Forward)
				}
			}
		}
	}

	fn init_loading_widgets(
		root: &mut Self::Root,
	) -> Option<relm4::loading_widgets::LoadingWidgets> {
		relm4::view! {
			#[local_ref]
			root {
				#[name(expander)]
				add = &adw::ExpanderRow {

				}
			}
		}
		Some(LoadingWidgets::new(root, expander))
	}

	async fn init_model(
		init: Self::Init,
		_index: &DynamicIndex,
		_sender: AsyncFactorySender<Self>,
	) -> Self {
		Self {
			name: init.name(),
			description: init.description(),
			icon: String::from(init.icon()),
			smart_list: init,
		}
	}

	fn init_widgets(
		&mut self,
		_index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: AsyncFactorySender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncFactorySender<Self>,
	) {
		match message {
			SmartSidebarFactoryListInput::SelectSmartList => sender.output(
				SmartSidebarFactoryListOutput::SelectSmartList(self.smart_list.clone()),
			),
		}
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			SmartSidebarFactoryListOutput::Forward => SmartSidebarListInput::Forward,
			SmartSidebarFactoryListOutput::SelectSmartList(list) => {
				SmartSidebarListInput::SelectSmartList(list)
			},
		};
		Some(output)
	}
}
