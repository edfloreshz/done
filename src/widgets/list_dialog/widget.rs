use std::str::FromStr;

use done_local_storage::service::Service;
use gtk::prelude::{
	BoxExt, ButtonExt, EntryBufferExtManual, EntryExt, WidgetExt,
};
use relm4::{
	adw,
	gtk::{
		self,
		traits::{GtkWindowExt, OrientableExt},
	},
	Component, ComponentParts, ComponentSender, RelmWidgetExt,
};
use relm4_icons::icon_name;

use crate::{
	fl,
	widgets::list_dialog::model::{ListDialogComponent, ListDialogMode},
};

use super::messages::{ListDialogInput, ListDialogOutput};

#[relm4::component(pub)]
impl Component for ListDialogComponent {
	type Input = ListDialogInput;
	type Output = ListDialogOutput;
	type Init = Option<String>;
	type CommandOutput = ();

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
							set_icon_name: Some(match model.mode {
								ListDialogMode::New => icon_name::PLUS,
								ListDialogMode::Edit => icon_name::PENCIL_AND_PAPER
							}),
					},
					gtk::Label {
						set_css_classes: &["title-4"],
						set_label: match model.mode {
							ListDialogMode::New => "You're about to add a list.",
							ListDialogMode::Edit => "You're about to rename this list."
						},
					},
					gtk::Label {
						set_label: "Pick a descriptive name.",
					},
					#[name = "new_list_entry"]
					gtk::Entry {
						set_placeholder_text: Some(fl!("list-name")),
						set_buffer: &model.name,
						connect_activate => ListDialogInput::HandleEntry,
					},
					gtk::DropDown::from_strings(&["Local", "Microsoft"]) {
						connect_activate[sender] => move |dw| {
							sender.input(ListDialogInput::UpdateService(dw.selected()))
						}
					},
					gtk::Button {
						set_css_classes: &["suggested-action"],
						set_label: model.label.as_str(),
						connect_clicked => ListDialogInput::HandleEntry,
					},
				}
			}
		}
	}

	fn init(
		init: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = if let Some(name) = init {
			ListDialogComponent {
				selected_service: None,
				name: gtk::EntryBuffer::new(Some(name)),
				mode: ListDialogMode::Edit,
				label: fl!("rename").clone(),
			}
		} else {
			ListDialogComponent {
				selected_service: None,
				name: gtk::EntryBuffer::new(Some("")),
				mode: ListDialogMode::New,
				label: fl!("add-list").clone(),
			}
		};

		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(
		&mut self,
		message: Self::Input,
		sender: ComponentSender<Self>,
		root: &Self::Root,
	) {
		match message {
			ListDialogInput::UpdateService(index) => {
				if let Some(item) = ["Local", "Microsoft"].get(index as usize) {
					self.selected_service = Service::from_str(item).ok();
				}
			},
			ListDialogInput::HandleEntry => {
				let name = self.name.text();

				match self.mode {
					ListDialogMode::New => {
						if let Some(service) = self.selected_service {
							sender
								.output(ListDialogOutput::AddTaskListToSidebar(
									name.to_string(),
									service,
								))
								.unwrap_or_default();
						}
					},
					ListDialogMode::Edit => {
						if let Some(service) = self.selected_service {
							sender
								.output(ListDialogOutput::RenameList(name.to_string(), service))
								.unwrap_or_default();
						}
					},
				}
				root.close();
			},
		}
	}
}
