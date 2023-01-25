use std::collections::HashMap;

use relm4::component::{AsyncComponentParts, SimpleAsyncComponent};
use relm4::{gtk, AsyncComponentSender};

use super::messages::{TodayInput, TodayOutput};
use super::model::TodayModel;

#[relm4::component(pub async)]
impl SimpleAsyncComponent for TodayModel {
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

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		_sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let model = Self {
			tasks: HashMap::new(),
		};
		let widgets = view_output!();
		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		_message: Self::Input,
		_sender: AsyncComponentSender<Self>,
	) {
	}
}
