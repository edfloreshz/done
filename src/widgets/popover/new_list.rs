use glib::clone;
use gtk::prelude::{
	BoxExt, ButtonExt, EditableExt, EntryBufferExtManual, EntryExt,
	ListBoxRowExt, OrientableExt, PopoverExt, WidgetExt,
};
use relm4::factory::FactoryVecDeque;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::{fl, SERVICES};
use crate::widgets::popover::providers_list::ProvidersList;

pub struct NewListModel {
	selected_provider: Option<String>,
	providers: FactoryVecDeque<ProvidersList>,
}

#[derive(Debug)]
pub enum NewListInput {
	SelectProvider(usize),
	AddTaskList(String),
}

#[derive(Debug)]
pub enum NewListOutput {
	AddTaskListToSidebar(String, String),
}

#[relm4::component(pub)]
impl SimpleComponent for NewListModel {
	type Input = NewListInput;
	type Output = NewListOutput;
	type InitParams = Option<String>;
	type Widgets = NewListWidgets;

	view! {
		#[root]
		gtk::Popover {
			#[wrap(Some)]
			set_child = &gtk::Stack {
				add_child = &gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					set_spacing: 10,
					gtk::Label::new(Some(fl!("list-name"))),
					gtk::Box {
						set_orientation: gtk::Orientation::Horizontal,
						set_spacing: 10,
						#[name = "new_list_entry"]
						gtk::Entry {
							connect_activate[sender] => move |entry| {
								let buffer = entry.buffer();
								if !buffer.text().is_empty() {
									sender.input.send(NewListInput::AddTaskList(buffer.text()))
								}
							}
						},
						#[name = "providers_button"]
						gtk::MenuButton {
							set_visible: true,
							set_icon_name: "x-office-address-book-symbolic",
							add_css_class: "raised",
							set_has_frame: true,
							set_direction: gtk::ArrowType::Right,
							#[wrap(Some)]
							set_popover = &gtk::Popover {
								#[wrap(Some)]
								set_child = &gtk::Box {
									#[name = "providers_list"]
									append = &gtk::ListBox {
										set_width_request: 100,
										connect_row_activated[sender] => move |list, _| {
											let row = list.selected_row().unwrap().index() as usize;
											sender.input.send(NewListInput::SelectProvider(row));
										}
									}
								}
							}
						}
					},
					#[name = "add_button"]
					gtk::Button {
						set_label: fl!("create-list"),
						set_css_classes: &["suggested-action"],
						connect_clicked: clone!(@strong new_list_entry, @strong sender => move |_| {
							let buffer = new_list_entry.buffer();
							if !buffer.text().is_empty() {
								sender.input(NewListInput::AddTaskList(buffer.text()))
							}
							new_list_entry.set_text("");
						})
					},
					#[name = "cancel_button"]
					gtk::Button {
						set_label: fl!("cancel"),
						connect_clicked: clone!(@strong root, @strong new_list_entry, @strong sender => move |_| {
							new_list_entry.set_text("");
							root.popdown();
						})
					},
				}
			}
		}
	}

	fn init(
		params: Self::InitParams,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let widgets = view_output!();
		let mut model = NewListModel {
			selected_provider: params,
			providers: FactoryVecDeque::new(
				widgets.providers_list.clone(),
				&sender.output,
			),
		};
		unsafe {
			for service in &mut *SERVICES.get_mut().unwrap() {
				if service.is_enabled() {
					model.providers.guard().push_back(&**service);
				}
			}
		}
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			NewListInput::SelectProvider(index) => {
				self.selected_provider =
					Some(self.providers.get(index).unwrap().provider.get_id().to_string())
			},
			NewListInput::AddTaskList(name) => {
				sender.output.send(NewListOutput::AddTaskListToSidebar(
					self.selected_provider.clone().unwrap(),
					name,
				))
			},
		}
	}
}
