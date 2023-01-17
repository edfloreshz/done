use crate::widgets::components::smart_lists::{SmartList, SmartListInput};
use adw::prelude::{ExpanderRowExt, PreferencesGroupExt, PreferencesRowExt};
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::gtk;
use relm4::loading_widgets::LoadingWidgets;
use relm4::adw;


#[derive(Debug)]
pub struct SmartListFactory {
    pub name: String, 
    pub description: String,
    pub icon: String,
}

#[derive(Debug)]
pub enum SmartListFactoryInput {
	Forward,
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
            icon: String::from(init.icon())
        }
	}

	fn init_widgets(
		&mut self,
		_index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		_sender: AsyncFactorySender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}

	async fn update(
		&mut self,
		message: Self::Input,
		_sender: AsyncFactorySender<Self>,
	) {
		match message {
            SmartListFactoryInput::Forward => todo!(),
		}
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			SmartListFactoryOutput::Forward => SmartListInput::Forward,
            SmartListFactoryOutput::SelectSmartList(list) => SmartListInput::SelectSmartList(list)
		};
		Some(output)
	}
}
