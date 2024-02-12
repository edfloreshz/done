use core_done::service::Service;
use libadwaita::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::{
	adw::prelude::{ActionableExt, ActionableExtManual},
	factory::{AsyncFactoryComponent, FactoryView},
	gtk::{self},
	loading_widgets::LoadingWidgets,
	prelude::DynamicIndex,
	AsyncFactorySender, RelmWidgetExt,
};

pub struct ServiceFactoryModel {
	service: Service,
}

#[derive(Debug)]
pub enum ServiceFactoryInput {
	Selected(DynamicIndex),
}

#[derive(Debug)]
pub enum ServiceFactoryOutput {
	ServiceSelected(DynamicIndex, Service),
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for ServiceFactoryModel {
	type ParentWidget = gtk::FlowBox;
	type Input = ServiceFactoryInput;
	type Output = ServiceFactoryOutput;
	type Init = Service;
	type CommandOutput = ();

	view! {
		#[root]
		gtk::ToggleButton {
			set_hexpand: true,
			set_tooltip: &self.service.to_string(),
			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				set_margin_all: 5,
				set_spacing: 5,
				set_halign: gtk::Align::Center,
				set_valign: gtk::Align::Center,
				gtk::Image {
					set_resource: Some(self.service.icon()),
				},
				gtk::Label {
					set_justify: gtk::Justification::Center,
					set_wrap: true,
					set_css_classes: &["caption"],
					set_text: &self.service.to_string(),
				}
			},
			set_action_target: Some("lists-page"),
			set_action_name: Some("navigation.push"),
			connect_clicked[sender, index] => move |_| {
				sender.input(ServiceFactoryInput::Selected(index.clone()));
			}
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
		index: &DynamicIndex,
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
			ServiceFactoryInput::Selected(index) => {
				sender
					.output(ServiceFactoryOutput::ServiceSelected(index, self.service))
					.unwrap_or_default();
				tracing::info!("Service selected: {}", self.service.to_string());
			},
		}
	}
}
