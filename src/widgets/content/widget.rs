use crate::widgets::content::messages::{ContentInput, ContentOutput};
use crate::widgets::task_input::messages::{TaskInputInput, TaskInputOutput};
use crate::widgets::task_input::model::TaskInputModel;

use relm4::component::{
	AsyncComponent, AsyncComponentParts, AsyncComponentSender,
};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::{
	adw, gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
};
use relm4::{Component, ComponentController, RelmWidgetExt};

use crate::widgets::content::model::ContentModel;

use super::helpers::{
	add_task, hide_flap, remove_task, reveal_task_details, select_task_list,
	update_task,
};

#[relm4::component(pub async)]
impl AsyncComponent for ContentModel {
	type CommandOutput = ();
	type Input = ContentInput;
	type Output = ContentOutput;
	type Init = ();

	view! {
		#[root]
		gtk::Stack {
			set_vexpand: true,
			set_transition_duration: 250,
			set_transition_type: gtk::StackTransitionType::Crossfade,
			#[name(flap)]
			adw::Flap {
				set_modal: true,
				set_locked: true,
				#[watch]
				set_reveal_flap: model.show_task_details,
				#[wrap(Some)]
				set_content = &gtk::Box {
					set_width_request: 300,
					set_margin_all: 10,
					set_orientation: gtk::Orientation::Vertical,
					gtk::Box {
						set_orientation: gtk::Orientation::Horizontal,
						gtk::Image {
							#[watch]
							set_visible: model.smart,
							#[watch]
							set_icon_name: model.icon.as_deref(),
							set_margin_start: 10,
						},
						gtk::Label {
							#[watch]
							set_visible: !model.smart,
							#[watch]
							set_text: model.icon.as_deref().unwrap_or_default(),
							set_margin_start: 10,
						},
						gtk::Label {
							set_css_classes: &["title-3"],
							set_halign: gtk::Align::Start,
							set_margin_start: 10,
							set_margin_end: 10,
							#[watch]
							set_text: model.title.as_str()
						},
					},
					gtk::Label {
						#[watch]
						set_visible: !model.description.is_empty(),
						set_css_classes: &["title-5"],
						set_halign: gtk::Align::Start,
						set_margin_bottom: 10,
						set_margin_start: 10,
						set_margin_end: 10,
						#[watch]
						set_text: model.description.as_str()
					},
					#[name(task_container)]
					gtk::Stack {
						set_transition_duration: 250,
						set_transition_type: gtk::StackTransitionType::Crossfade,
						gtk::ScrolledWindow {
							#[watch]
							set_visible: !model.task_factory.is_empty(),
							set_vexpand: true,
							set_hexpand: true,

							#[local_ref]
							list_box -> adw::PreferencesGroup {
								set_css_classes: &["boxed-list"],
								set_valign: gtk::Align::Fill,
								set_margin_all: 5,
							},
						},
						gtk::CenterBox {
							#[watch]
							set_visible: model.task_factory.is_empty(),
							set_vexpand: true,
							set_hexpand: true,
							set_orientation: gtk::Orientation::Vertical,
							set_halign: gtk::Align::Center,
							set_valign: gtk::Align::Center,
							#[wrap(Some)]
							set_center_widget = &gtk::Box {
								set_orientation: gtk::Orientation::Vertical,
								set_margin_all: 24,
								set_spacing: 24,
								gtk::Picture {
									#[watch]
									set_resource: Some(model.page_icon),
									set_margin_all: 70
								},
								gtk::Label {
									set_css_classes: &["title-3", "accent"],
									set_wrap: true,
									set_wrap_mode: gtk::pango::WrapMode::Word,
									set_justify: gtk::Justification::Center,
									#[watch]
									set_text: &model.page_title,
								},
								gtk::Label {
									set_css_classes: &["body"],
									#[watch]
									set_text: &model.page_subtitle,
									set_wrap: true,
									set_wrap_mode: gtk::pango::WrapMode::Word,
									set_justify: gtk::Justification::Center,
								},
							}
						}
					},
					append: model.task_entry.widget()
				},
				#[wrap(Some)]
				#[local_ref]
				set_flap = flap_container -> gtk::Box {
					set_width_request: 300,
					set_css_classes: &["background"],
				},
				#[wrap(Some)]
				set_separator = &gtk::Separator {
					set_orientation: gtk::Orientation::Vertical,
				},
				set_flap_position: gtk::PackType::End,
			}
		}
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let model = ContentModel {
			task_factory: AsyncFactoryVecDeque::new(
				adw::PreferencesGroup::default(),
				sender.input_sender(),
			),
			task_details_factory: AsyncFactoryVecDeque::new(
				gtk::Box::default(),
				sender.input_sender(),
			),
			task_entry: TaskInputModel::builder().launch(None).forward(
				sender.input_sender(),
				|message| match message {
					TaskInputOutput::EnterCreationMode(task) => {
						ContentInput::RevealTaskDetails(None, task)
					},
					TaskInputOutput::AddTask(task) => ContentInput::AddTask(task),
				},
			),
			parent_list: None,
			icon: None,
			title: String::new(),
			description: String::new(),
			smart: false,
			selected_task: None,
			show_task_details: false,
			page_icon: "/dev/edfloreshz/Done/icons/scalable/actions/empty.png",
			page_title: "list-empty".into(),
			page_subtitle: "instructions".into(),
		};
		let list_box = model.task_factory.widget();
		let flap_container = model.task_details_factory.widget();

		let widgets = view_output!();

		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			ContentInput::Refresh => {
				if let Some(list) = &self.parent_list {
					sender.input(ContentInput::SelectList(list.clone()))
				}
			},
			ContentInput::AddTask(mut task) => {
				if let Err(err) = add_task(self, sender, &mut task).await {
					tracing::error!("{err}");
				}
			},
			ContentInput::RemoveTask(index) => {
				if let Err(err) = remove_task(self, sender, index).await {
					tracing::error!("{err}");
				}
			},
			ContentInput::UpdateTask(task) => {
				if let Err(err) = update_task(self, sender, task).await {
					tracing::error!("{err}");
				}
			},
			ContentInput::SelectList(list) => {
				if let Err(err) = select_task_list(self, list).await {
					tracing::error!("{err}");
				}
			},
			ContentInput::RevealTaskDetails(index, task) => {
				reveal_task_details(self, index, task)
			},
			ContentInput::DisablePlugin => self.parent_list = None,
			ContentInput::CleanTaskEntry => self
				.task_entry
				.sender()
				.send(TaskInputInput::CleanTaskEntry)
				.unwrap(),
			ContentInput::HideFlap => hide_flap(self, sender),
		}
	}
}
