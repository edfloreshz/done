use gtk::traits::{ButtonExt, WidgetExt};
use relm4::{
	factory::FactoryView,
	gtk,
	prelude::{DynamicIndex, FactoryComponent},
	FactorySender, RelmWidgetExt,
};
use relm4_icons::icon_name;

use crate::fl;

pub struct TagModel {
	pub title: String,
	pub index: DynamicIndex,
}

#[derive(Debug)]
pub enum TagInput {
	RemoveTag(DynamicIndex),
}

#[derive(Debug)]
pub enum TagOutput {
	RemoveTag(DynamicIndex),
}

#[derive(derive_new::new)]
pub struct TagInit {
	pub title: String,
}

#[relm4::factory(pub)]
impl FactoryComponent for TagModel {
	type ParentWidget = gtk::FlowBox;
	type Input = TagInput;
	type Output = TagOutput;
	type Init = TagInit;
	type CommandOutput = ();

	view! {
		#[root]
		gtk::Box {
			set_valign: gtk::Align::Center,
			add_css_class: "linked",
			#[name(tag_label)]
			gtk::Button {
				set_label: &self.title,
			},
			#[name(close_button)]
			gtk::Button {
				set_icon_name: icon_name::SMALL_X,
				set_valign: gtk::Align::Center,
				set_tooltip: fl!("remove-tag"),
				connect_clicked[sender, index] => move |_| {
					sender.input(TagInput::RemoveTag(index.clone()))
				}
			}
		}
	}

	fn init_model(
		init: Self::Init,
		index: &DynamicIndex,
		_sender: FactorySender<Self>,
	) -> Self {
		Self {
			title: init.title,
			index: index.clone(),
		}
	}

	fn init_widgets(
		&mut self,
		index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: FactorySender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}

	fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
		match message {
			TagInput::RemoveTag(index) => sender
				.output(TagOutput::RemoveTag(index))
				.unwrap_or_default(),
		}
	}
}
