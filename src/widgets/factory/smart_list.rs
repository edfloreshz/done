use crate::widgets::components::smart_lists::{SmartList, SmartListInput};
use adw::prelude::{ExpanderRowExt, PreferencesGroupExt, PreferencesRowExt};
use relm4::adw;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::gtk;
use relm4::gtk::traits::WidgetExt;
use relm4::loading_widgets::LoadingWidgets;

#[derive(Debug)]
pub struct SmartListFactory {
	pub name: String,
	pub description: String,
	pub icon: String,
	pub smart_list: SmartList
}

#[derive(Debug)]
pub enum SmartListFactoryInput {
	SelectSmartList
}

#[derive(Debug)]
pub enum SmartListFactoryOutput {
	SelectSmartList(SmartList),
	Forward,
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for SmartListFactory {
	type ParentInput = SmartListInput;
	type ParentWidget = gtk::Box;
	type CommandOutput = ();
	type Input = SmartListFactoryInput;
	type Output = SmartListFactoryOutput;
	type Init = SmartList;
	type Widgets = ProviderWidgets;

	view! {
		#[root]
		#[name(list_box)]
		adw::PreferencesGroup {
			#[name(expander)]
			add = &adw::ExpanderRow {
				#[watch]
				set_title: self.name.as_str(),
				#[watch]
				set_subtitle: self.description.as_str(),
				#[watch]
				set_icon_name: Some(self.icon.as_str()),
				#[watch]
				set_enable_expansion: false,
				set_expanded: false,
			},
			add_controller = &gtk::GestureClick {
				connect_pressed[sender] => move |_, _, _, _| {
					sender.input(SmartListFactoryInput::SelectSmartList);
					sender.output(SmartListFactoryOutput::Forward)
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
			name: String::from(init.name()),
			description: String::from(init.description()),
			icon: String::from(init.icon()),
			smart_list: init
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
    		SmartListFactoryInput::SelectSmartList => sender.output(SmartListFactoryOutput::SelectSmartList(self.smart_list.clone())),
		}
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			SmartListFactoryOutput::Forward => SmartListInput::Forward,
			SmartListFactoryOutput::SelectSmartList(list) => {
				SmartListInput::SelectSmartList(list)
			},
		};
		Some(output)
	}
}
