use std::collections::HashMap;

use relm4::component::{AsyncComponentParts, SimpleAsyncComponent};
use relm4::{gtk, AsyncComponentSender};

use super::messages::{AllInput, AllOutput};
use super::model::AllModel;

#[relm4::component(pub async)]
impl SimpleAsyncComponent for AllModel {
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
