use core_done::service::Service;
use glib::Cast;
use libadwaita::prelude::{FlowBoxChildExt, ToggleButtonExt};
use relm4::{
	component::{AsyncComponent, AsyncComponentParts},
	factory::{AsyncFactoryVecDeque, DynamicIndex},
	gtk::{self, prelude::OrientableExt},
	AsyncComponentSender, RelmIterChildrenExt, RelmWidgetExt,
};

use crate::{
	app::factories::service::{ServiceFactoryModel, ServiceFactoryOutput},
	fl,
};

pub struct ServicesModel {
	services_factory: AsyncFactoryVecDeque<ServiceFactoryModel>,
}

#[derive(Debug)]
pub enum ServicesInput {
	ServiceSelected(DynamicIndex, Service),
	ReloadServices(Service),
}

#[derive(Debug)]
pub enum ServicesOutput {
	ServiceSelected(Service),
	ServiceDisabled(Service),
}

#[relm4::component(pub async)]
impl AsyncComponent for ServicesModel {
	type CommandOutput = ();
	type Input = ServicesInput;
	type Output = ServicesOutput;
	type Init = ();

	view! {
		#[root]
		gtk::Box {
			#[local_ref]
			flow_box -> gtk::FlowBox {
				set_margin_all: 10,
				set_column_spacing: 5,
				#[watch]
				set_orientation: if model.services_factory.len() == 1 {
					gtk::Orientation::Vertical
				} else {
					gtk::Orientation::Horizontal
				},
				set_selection_mode: gtk::SelectionMode::None,
				set_homogeneous: true,
				#[watch]
				set_max_children_per_line: if model.services_factory.len() == 1 {
					1
				} else {
					2
				},
			},
		}
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let _keyboard_shortcuts: &str = fl!("keyboard-shortcuts");
		let _about_done: &str = fl!("about-done");
		let _quit: &str = fl!("quit");

		let mut services_factory = AsyncFactoryVecDeque::builder()
			.launch(gtk::FlowBox::default())
			.forward(sender.input_sender(), |output| match output {
				ServiceFactoryOutput::ServiceSelected(index, service) => {
					ServicesInput::ServiceSelected(index, service)
				},
			});

		{
			let mut guard = services_factory.guard();

			for service in Service::list() {
				if service.get_service().available() {
					guard.push_back(service);
				}
			}
		}

		let model = ServicesModel { services_factory };

		let flow_box = model.services_factory.widget();

		let selected_child = flow_box
			.child_at_index(0)
			.unwrap()
			.child()
			.unwrap()
			.downcast::<gtk::ToggleButton>();
		if let Ok(button) = selected_child {
			button.set_active(true);
		}

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
			ServicesInput::ReloadServices(service) => {
				let mut guard = self.services_factory.guard();
				guard.clear();
				for service in Service::list() {
					if service.get_service().available() {
						guard.push_back(service);
					}
				}
				sender
					.output(ServicesOutput::ServiceDisabled(service))
					.unwrap()
			},
			ServicesInput::ServiceSelected(index, service) => {
				let flow_box = self.services_factory.widget();

				for (i, child) in flow_box.iter_children().enumerate() {
					if let Ok(button) =
						child.child().unwrap().downcast::<gtk::ToggleButton>()
					{
						if i != index.current_index() {
							button.set_active(false);
						}
					}
				}

				sender
					.output(ServicesOutput::ServiceSelected(service))
					.unwrap();
			},
		}
	}
}
