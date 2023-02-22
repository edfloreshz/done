use relm4::{
	factory::AsyncFactoryVecDeque,
	gtk::{
		self,
		traits::{BoxExt, OrientableExt},
	},
	ComponentParts, SimpleComponent,
};

use crate::widgets::smart_lists::sidebar::model::{
	SmartList, SmartSidebarListModel,
};

use super::message::{SmartSidebarListInput, SmartSidebarListOutput};

#[relm4::component(pub)]
impl SimpleComponent for SmartSidebarListModel {
	type Input = SmartSidebarListInput;
	type Output = SmartSidebarListOutput;
	type Init = ();

	view! {
		#[root]
		gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			#[local_ref]
			smart_list_container -> gtk::Box {
				set_spacing: 12,
				set_orientation: gtk::Orientation::Vertical,
			}
		}
	}

	fn init(
		_init: Self::Init,
		root: &Self::Root,
		sender: relm4::ComponentSender<Self>,
	) -> relm4::ComponentParts<Self> {
		let mut model = SmartSidebarListModel {
			smart_list_controller: AsyncFactoryVecDeque::new(
				gtk::Box::default(),
				sender.input_sender(),
			),
		};
		let smart_list_container = model.smart_list_controller.widget();
		let widgets = view_output!();
		for smart_list in SmartList::list() {
			model.smart_list_controller.guard().push_back(smart_list);
		}
		ComponentParts { model, widgets }
	}

	fn update(
		&mut self,
		message: Self::Input,
		sender: relm4::ComponentSender<Self>,
	) {
		match message {
			SmartSidebarListInput::SelectSmartList(list) => sender
				.output(SmartSidebarListOutput::SelectSmartList(list))
				.unwrap(),
			SmartSidebarListInput::Forward => {
				sender.output(SmartSidebarListOutput::Forward).unwrap()
			},
		}
	}
}
