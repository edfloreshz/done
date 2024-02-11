use core_done::service::Service;
use relm4::{
	adw::prelude::{ActionableExt, ActionableExtManual},
	factory::{AsyncFactoryComponent, FactoryView},
	gtk::{self, prelude::ListBoxRowExt, traits::WidgetExt},
	loading_widgets::LoadingWidgets,
	prelude::DynamicIndex,
	AsyncFactorySender, RelmWidgetExt,
};

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
	type Input = ServiceFactoryInput;
	type Output = ServiceFactoryOutput;
	type Init = Service;
	type CommandOutput = ();

	view! {
		#[root]
		gtk::ListBoxRow {
			set_tooltip: &self.service.to_string(),
			gtk::Box {
				set_css_classes: &["plugin"],
				gtk::Image {
					set_icon_name: Some(self.service.icon()),
					set_margin_end: 10,
				},
				gtk::Label {
					set_text: &self.service.to_string()
				}
			},
			set_action_name: Some("navigation.push"),
			set_action_target: Some("lists-page"),
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
					.output(ServiceFactoryOutput::ServiceSelected(self.service))
					.unwrap_or_default();
				tracing::info!("Service selected: {}", self.service.to_string());
			},
		}
	}
}
