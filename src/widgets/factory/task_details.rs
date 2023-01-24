//TODO: Create task details factory.

use std::str::FromStr;

use adw::{
	prelude::MessageDialogExtManual,
	traits::{
		ActionRowExt, EntryRowExt, MessageDialogExt, PreferencesGroupExt,
		PreferencesRowExt,
	},
};
use chrono::NaiveDateTime;
use glib::{clone, Cast};
use gtk::traits::{
	BoxExt, ButtonExt, GtkWindowExt, ListBoxRowExt, OrientableExt,
	ToggleButtonExt, WidgetExt,
};
use proto_rust::{Task, TaskImportance, TaskStatus};
use relm4::{
	adw,
	factory::{AsyncFactoryComponent, FactoryView},
	gtk,
	gtk::prelude::EditableExt,
	loading_widgets::LoadingWidgets,
	prelude::DynamicIndex,
	AsyncFactorySender, RelmWidgetExt,
};

use crate::{fl, widgets::components::content::ContentComponentInput};

pub struct TaskDetailsFactoryModel {
	original_task: Task,
	task: Task,
	task_details_index: DynamicIndex,
	update: bool,
	selected_due_date: Option<String>,
	selected_reminder_date: Option<String>,
	dirty: bool,
}

#[derive(Debug)]
pub enum TaskDetailsFactoryInput {
	SaveTask,
	SetTitle(String),
	SetBody(Option<String>),
	SetImportance(i32),
	SetFavorite(bool),
	SetStatus(bool),
	SetDueDate(Option<NaiveDateTime>),
	SetReminderDate(Option<NaiveDateTime>),
	CancelWarning,
}

#[derive(Debug)]
pub enum TaskDetailsFactoryOutput {
	SaveTask(Option<DynamicIndex>, Task, bool),
	CleanTaskEntry,
	HideFlap,
}

#[derive(derive_new::new)]
pub struct TaskDetailsFactoryInit {
	task: Task,
	index: Option<DynamicIndex>,
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for TaskDetailsFactoryModel {
	type ParentWidget = gtk::Box;
	type ParentInput = ContentComponentInput;
	type Input = TaskDetailsFactoryInput;
	type Output = TaskDetailsFactoryOutput;
	type Init = TaskDetailsFactoryInit;
	type CommandOutput = ();

	view! {
		#[root]
		#[name(overlay)]
		adw::ToastOverlay {
			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				set_margin_all: 20,
				adw::PreferencesGroup {
					set_title: "Details",
					#[wrap(Some)]
					set_header_suffix = &gtk::Box {
						set_spacing: 5,
						gtk::Button {
							set_icon_name: "go-previous-symbolic",
							set_tooltip_text: Some(fl!("cancel")),
							connect_clicked[sender] => move |_| {
								sender.input(TaskDetailsFactoryInput::CancelWarning)
							}
						},
						gtk::Button {
							set_icon_name: "media-floppy-symbolic",
							set_tooltip_text: Some(fl!("save")),
							set_css_classes: &["suggested-action"],
							connect_clicked[sender] => move |_| {
								sender.input(TaskDetailsFactoryInput::SaveTask)
							}
						},
					},
					adw::EntryRow {
							set_title: "Title",
							set_text: self.task.title.as_str(),
							set_show_apply_button: true,
							set_enable_emoji_completion: true,
							#[name(favorite)]
							add_suffix = &gtk::ToggleButton {
								add_css_class: "opaque",
								add_css_class: "circular",
								#[watch]
								set_class_active: ("favorite", self.task.favorite),
								set_icon_name: "star-filled-rounded-symbolic",
								set_valign: gtk::Align::Center,
								connect_toggled[sender] => move |toggle| {
									sender.input(TaskDetailsFactoryInput::SetFavorite(toggle.is_active()));
								}
							},
							connect_changed[sender] => move |entry| {
								let buffer = entry.text().to_string();
								sender.input(TaskDetailsFactoryInput::SetTitle(buffer));
							},
							connect_activate[sender] => move |entry| {
								let buffer = entry.text().to_string();
								sender.input(TaskDetailsFactoryInput::SetTitle(buffer));
							},
							connect_apply[sender] => move |entry| {
								let buffer = entry.text().to_string();
								sender.input(TaskDetailsFactoryInput::SetTitle(buffer));
							},
					},
					adw::EntryRow {
						set_title: "Body",
						set_show_apply_button: true,
						set_enable_emoji_completion: true,
						set_text: self.task.body.as_deref().unwrap_or(""),
						connect_changed[sender] => move |entry| {
							let buffer = entry.text().to_string();
							if buffer.is_empty() {
								sender.input(TaskDetailsFactoryInput::SetBody(None));
							} else {
								sender.input(TaskDetailsFactoryInput::SetBody(Some(buffer)));
							}
						},
						connect_activate[sender] => move |entry| {
							let buffer = entry.text().to_string();
							if buffer.is_empty() {
								sender.input(TaskDetailsFactoryInput::SetBody(None));
							} else {
								sender.input(TaskDetailsFactoryInput::SetBody(Some(buffer)));
							}
						},
						connect_apply[sender] => move |entry| {
							let buffer = entry.text().to_string();
							if buffer.is_empty() {
								sender.input(TaskDetailsFactoryInput::SetBody(None));
							} else {
								sender.input(TaskDetailsFactoryInput::SetBody(Some(buffer)));
							}
						},
					},
					adw::ActionRow {
						set_icon_name: Some("checkbox-checked-symbolic"),
						set_title: "Completed",
						set_subtitle: "Sets wether the task is completed",
						add_suffix = &gtk::Switch {
							#[watch]
							set_active: self.task.status == 1,
							set_valign: gtk::Align::Center,
							connect_state_set[sender] => move |_, state| {
								sender.input(TaskDetailsFactoryInput::SetStatus(state));
								gtk::Inhibit(false)
							}
						}
					},
					adw::ActionRow {
						set_icon_name: Some("emblem-important-symbolic"),
						set_title: "Importance",
						set_subtitle: "Set the importance for this task",
						add_suffix = &gtk::Box {
							set_css_classes: &["linked"],
							#[name(low_importance)]
							gtk::ToggleButton {
								set_icon_name: "flag-outline-thin-symbolic",
								set_tooltip_text: Some("Low"),
								set_css_classes: &["flat", "image-button"],
								set_valign: gtk::Align::Center,
								set_active: self.task.importance == TaskImportance::Low as i32,
								connect_toggled[sender] => move |toggle| {
									if toggle.is_active() {
										sender.input(TaskDetailsFactoryInput::SetImportance(TaskImportance::Low as i32));
									}
								}
							},
							gtk::ToggleButton {
								set_icon_name: "flag-outline-thick-symbolic",
								set_tooltip_text: Some("Medium"),
								set_css_classes: &["flat", "image-button"],
								set_valign: gtk::Align::Center,
								set_group: Some(&low_importance),
								set_active: self.task.importance == TaskImportance::Normal as i32,
								connect_toggled[sender] => move |toggle| {
									if toggle.is_active() {
										sender.input(TaskDetailsFactoryInput::SetImportance(TaskImportance::Normal as i32));
									}
								}
							},
							gtk::ToggleButton {
								set_icon_name: "flag-filled-symbolic",
								set_tooltip_text: Some("High"),
								set_css_classes: &["flat", "image-button"],
								set_valign: gtk::Align::Center,
								set_group: Some(&low_importance),
								set_active: self.task.importance == TaskImportance::High as i32,
								connect_toggled[sender] => move |toggle| {
									if toggle.is_active() {
										sender.input(TaskDetailsFactoryInput::SetImportance(TaskImportance::High as i32));
									}
								}
							}
						}
					},
					adw::ActionRow {
						set_icon_name: Some("office-calendar-symbolic"),
						set_title: "Due date",
						set_subtitle: "Set the due date for this task",
						add_suffix = &gtk::MenuButton {
							#[watch]
							set_label: self.selected_due_date.as_deref().unwrap_or("No date set"),
							set_css_classes: &["flat", "image-button"],
							set_valign: gtk::Align::Center,
							set_direction: gtk::ArrowType::Down,
							#[wrap(Some)]
							set_popover = &gtk::Popover {
								gtk::Calendar {
									connect_day_selected[sender] => move |calendar| {
										if let Ok(date) = calendar.date().format("%Y-%m-%dT%H:%M:%S") {
											if let Ok(date) = NaiveDateTime::from_str(date.to_string().as_str()) {
												sender.input(TaskDetailsFactoryInput::SetDueDate(Some(date)))
											}
										}
									}
								}
							}
						}
					},
					adw::ActionRow {
						set_icon_name: Some("appointment-soon-symbolic"),
						set_title: "Reminder",
						set_subtitle: "Set a date to get a reminder",
						add_suffix = &gtk::MenuButton {
							#[watch]
							set_label: self.selected_reminder_date.as_deref().unwrap_or("No date set"),
							set_css_classes: &["flat", "image-button"],
							set_valign: gtk::Align::Center,
							set_direction: gtk::ArrowType::Down,
							#[wrap(Some)]
							set_popover = &gtk::Popover {
								gtk::Calendar {
									connect_day_selected[sender] => move |calendar| {
										if let Ok(date) = calendar.date().format("%Y-%m-%dT%H:%M:%S") {
											if let Ok(date) = NaiveDateTime::from_str(date.to_string().as_str()) {
												sender.input(TaskDetailsFactoryInput::SetReminderDate(Some(date)))
											}
										}
									}
								}
							}
						}
					}
				},

			}
		}
	}

	fn init_loading_widgets(root: &mut Self::Root) -> Option<LoadingWidgets> {
		relm4::view! {
			#[local_ref]
			root {
				#[name(spinner)]
				gtk::Spinner {
					start: ()
				}
			}
		}
		Some(LoadingWidgets::new(root, spinner))
	}

	async fn init_model(
		init: Self::Init,
		index: &DynamicIndex,
		_sender: AsyncFactorySender<Self>,
	) -> Self {
		Self {
			original_task: init.task.clone(),
			task: init.task.clone(),
			task_details_index: index.clone(),
			update: init.index.is_some(),
			selected_due_date: if let Some(date) = init.task.due_date {
				NaiveDateTime::from_timestamp_opt(date, 0)
					.map(|date| date.format("%m/%d/%Y").to_string())
			} else {
				None
			},
			selected_reminder_date: if let Some(date) = init.task.reminder_date {
				NaiveDateTime::from_timestamp_opt(date, 0)
					.map(|date| date.format("%m/%d/%Y").to_string())
			} else {
				None
			},
			dirty: false,
		}
	}

	fn init_widgets(
		&mut self,
		_index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: AsyncFactorySender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}

	async fn update_with_view(
		&mut self,
		widgets: &mut Self::Widgets,
		message: Self::Input,
		sender: AsyncFactorySender<Self>,
	) {
		match message {
			TaskDetailsFactoryInput::CancelWarning => {
				if let Some(root) = widgets.overlay.root() {
					let dialog = adw::MessageDialog::builder()
						.transient_for(&root.downcast::<gtk::Window>().unwrap())
						.heading("Discard Changes")
						.body("Your changes will be lost, are you sure?")
						.build();
					dialog.add_responses(&[("no", "No"), ("yes", "Yes")]);
					dialog.set_response_appearance(
						"yes",
						adw::ResponseAppearance::Destructive,
					);
					let dirty = self.dirty;
					dialog.connect_response(
						None,
						clone!(@strong sender => move |dialog, response| {
							if response == "yes" {
								sender.output(TaskDetailsFactoryOutput::HideFlap)
							}
							dialog.close();
						}),
					);
					if dirty {
						dialog.present();
					} else {
						sender.output(TaskDetailsFactoryOutput::HideFlap)
					}
				}
			},
			TaskDetailsFactoryInput::SaveTask => {
				if self.update {
					sender.output(TaskDetailsFactoryOutput::SaveTask(
						Some(self.task_details_index.clone()),
						self.task.clone(),
						self.update,
					));
					self.original_task = self.task.clone();
					self.dirty = false;
				} else {
					sender.output(TaskDetailsFactoryOutput::SaveTask(
						None,
						self.task.clone(),
						self.update,
					));
				}
				if !self.update {
					sender.output(TaskDetailsFactoryOutput::CleanTaskEntry)
				}
			},
			TaskDetailsFactoryInput::SetTitle(title) => {
				self.task.title = title;
			},
			TaskDetailsFactoryInput::SetBody(body) => {
				self.task.body = body;
			},
			TaskDetailsFactoryInput::SetImportance(importance) => {
				self.task.importance = importance;
			},
			TaskDetailsFactoryInput::SetFavorite(favorite) => {
				self.task.favorite = favorite;
			},
			TaskDetailsFactoryInput::SetStatus(status) => {
				if status {
					self.task.status = TaskStatus::Completed as i32;
				} else {
					self.task.status = TaskStatus::NotStarted as i32;
				}
			},
			TaskDetailsFactoryInput::SetDueDate(due_date) => {
				if let Some(date) = due_date {
					self.selected_due_date = Some(date.format("%m/%d/%Y").to_string());
					let timestamp = date.timestamp();
					self.task.due_date = Some(timestamp);
				} else {
					self.task.due_date = None;
				}
			},
			TaskDetailsFactoryInput::SetReminderDate(reminder_date) => {
				if let Some(date) = reminder_date {
					self.selected_reminder_date =
						Some(date.format("%m/%d/%Y").to_string());
					let timestamp = date.timestamp();
					self.task.reminder_date = Some(timestamp);
					self.task.is_reminder_on = true;
				} else {
					self.task.reminder_date = None;
					self.task.is_reminder_on = false;
				}
			},
		}
		if self.task != self.original_task {
			self.dirty = true;
		}
		self.update_view(widgets, sender);
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			TaskDetailsFactoryOutput::CleanTaskEntry => {
				ContentComponentInput::CleanTaskEntry
			},
			TaskDetailsFactoryOutput::SaveTask(_, task, is_update) => {
				if is_update {
					ContentComponentInput::UpdateTask(task)
				} else {
					ContentComponentInput::AddTask(task)
				}
			},
			TaskDetailsFactoryOutput::HideFlap => ContentComponentInput::HideFlap,
		};
		Some(output)
	}
}
