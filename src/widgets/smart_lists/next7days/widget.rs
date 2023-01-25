use std::collections::HashMap;

use relm4::component::{AsyncComponentParts, SimpleAsyncComponent};
use relm4::{gtk, AsyncComponentSender};

use super::messages::{Next7DaysInput, Next7DaysOutput};
use super::model::Next7DaysModel;

#[relm4::component(pub async)]
impl SimpleAsyncComponent for Next7DaysModel {
	type Input = Next7DaysInput;
	type Output = Next7DaysOutput;
	type Init = ();

	view! {
		#[root]
		gtk::Box {
			gtk::Label {
				set_text: "Next 7 Days"
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
