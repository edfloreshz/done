use std::collections::HashMap;

use proto_rust::{provider_client::ProviderClient, Channel, Task};
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::application::plugin::Plugin;

pub struct AllModel {
	pub tasks: HashMap<Plugin, Vec<Task>>,
}

#[derive(Debug)]
pub enum AllInput {}

#[derive(Debug)]
pub enum AllOutput {}

#[relm4::component(pub)]
impl SimpleComponent for AllModel {
	type Input = AllInput;

	type Output = AllOutput;

	type Init = ();

	view! {
			#[root]
			gtk::Box {
								gtk::Label {
										set_text: "All"
								}
			}
	}

	fn init(
		init: Self::Init,
		root: &Self::Root,
		sender: relm4::ComponentSender<Self>,
	) -> relm4::ComponentParts<Self> {
		let model = Self {
			tasks: HashMap::new(),
		};
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(
		&mut self,
		message: Self::Input,
		sender: relm4::ComponentSender<Self>,
	) {
	}
}
