use std::collections::HashMap;

use proto_rust::Task;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::application::plugin::Plugin;

pub struct TodayModel {
	pub tasks: HashMap<Plugin, Vec<Task>>,
}

#[derive(Debug)]
pub enum TodayInput {}

#[derive(Debug)]
pub enum TodayOutput {}

#[relm4::component(pub)]
impl SimpleComponent for TodayModel {
	type Input = TodayInput;

	type Output = TodayOutput;

	type Init = ();

	view! {
				#[root]
				gtk::Box {
						gtk::Label {
								set_text: "Today"
						}
				}
	}

	fn init(
		_init: Self::Init,
		root: &Self::Root,
		_sender: relm4::ComponentSender<Self>,
	) -> relm4::ComponentParts<Self> {
		let model = Self {
			tasks: HashMap::new(),
		};
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(
		&mut self,
		_message: Self::Input,
		_sender: relm4::ComponentSender<Self>,
	) {
	}
}
