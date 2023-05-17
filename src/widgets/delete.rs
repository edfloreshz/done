use relm4::{
	adw,
	gtk::{
		self,
		traits::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt},
	},
	Component, ComponentParts, RelmWidgetExt,
};
use relm4_icons::icon_name;

#[derive(Debug)]
pub struct DeleteComponent {
	delete_warning: String,
	warning: String,
}

#[derive(Debug)]
pub enum DeleteInput {
	Delete,
	Cancel,
}

#[derive(Debug)]
pub enum DeleteOutput {
	Delete,
}

pub struct DeleteInit {
	pub warning: String,
	pub delete_warning: String,
}

#[relm4::component(pub)]
impl Component for DeleteComponent {
	type CommandOutput = ();
	type Input = DeleteInput;
	type Output = DeleteOutput;
	type Init = DeleteInit;

	view! {
		#[root]
		adw::Window {
			set_hide_on_close: true,
			set_default_width: 320,
			set_resizable: false,
			set_modal: true,

			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,

				adw::HeaderBar {
					set_show_end_title_buttons: true,
					set_css_classes: &["flat"],
					set_title_widget: Some(&gtk::Box::default())
				},
				gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					set_margin_all: 20,
					set_spacing: 10,
										gtk::Image {
												set_icon_size: gtk::IconSize::Large,
												set_icon_name: Some(icon_name::WARNING),
										},
						gtk::Label {
						set_css_classes: &["title-4"],
						set_label: model.warning.as_str(),
					},
						gtk::Label {
												set_label: model.delete_warning.as_str(),
					},
					gtk::Button {
						set_label: "Cancel",
						connect_clicked => DeleteInput::Cancel,
					},
					gtk::Button {
						set_css_classes: &["destructive-action"],
						set_label: "Delete",
						connect_clicked => DeleteInput::Delete,
					},
				}
			}
		}
	}

	fn update(
		&mut self,
		message: Self::Input,
		sender: relm4::ComponentSender<Self>,
		root: &Self::Root,
	) {
		match message {
			DeleteInput::Cancel => root.close(),
			DeleteInput::Delete => {
				sender.output(DeleteOutput::Delete).unwrap_or_default();
			},
		}
		root.close()
	}

	fn init(
		init: Self::Init,
		root: &Self::Root,
		_sender: relm4::ComponentSender<Self>,
	) -> relm4::ComponentParts<Self> {
		let model = DeleteComponent {
			delete_warning: init.delete_warning,
			warning: init.warning,
		};
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}
}
