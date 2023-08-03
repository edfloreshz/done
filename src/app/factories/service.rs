use core_done::service::Service;
use relm4::{
	factory::{AsyncFactoryComponent, FactoryView},
	gtk::{self, prelude::ListBoxRowExt, traits::WidgetExt},
	loading_widgets::LoadingWidgets,
	prelude::DynamicIndex,
	AsyncFactorySender,
};

use crate::app::components::services_sidebar::ServicesSidebarInput;

pub struct ServiceFactoryModel {
	service: Service,
}

#[derive(Debug)]
pub enum ServiceFactoryInput {
	Selected,
}

#[derive(Debug)]
pub enum ServiceFactoryOutput {
	ServiceSelected(Service),
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for ServiceFactoryModel {
	type ParentWidget = gtk::ListBox;
	type ParentInput = ServicesSidebarInput;
	type Input = ServiceFactoryInput;
	type Output = ServiceFactoryOutput;
	type Init = Service;
	type CommandOutput = ();

	view! {
			#[root]
	gtk::ListBoxRow {
		set_has_tooltip: true,
		set_tooltip_text: Some(&self.service.to_string()),
		gtk::CenterBox {
			set_css_classes: &["plugin"],
			#[wrap(Some)]
			set_center_widget = &gtk::Image {
				set_icon_name: Some(self.service.icon())
			}
		},
		connect_activate => ServiceFactoryInput::Selected
	}
	}

	fn init_loading_widgets(root: &mut Self::Root) -> Option<LoadingWidgets> {
		relm4::view! {
				#[local_ref]
				root {
						#[name(spinner)]
						gtk::Spinner {
								start: ()
						}
				}
		}
		Some(LoadingWidgets::new(root, spinner))
	}

	async fn init_model(
		init: Self::Init,
		_index: &DynamicIndex,
		_sender: AsyncFactorySender<Self>,
	) -> Self {
		Self { service: init }
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
			ServiceFactoryInput::Selected => {
				sender
					.output(ServiceFactoryOutput::ServiceSelected(self.service.clone()));
				tracing::info!("Service selected: {}", self.service.to_string());
			},
		}
	}

	fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			ServiceFactoryOutput::ServiceSelected(service) => {
				ServicesSidebarInput::ServiceSelected(service)
			},
		};
		Some(output)
	}
}
