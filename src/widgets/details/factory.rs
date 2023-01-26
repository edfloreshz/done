use std::str::FromStr;

use adw::{
	prelude::MessageDialogExtManual,
	traits::{
		ActionRowExt, EntryRowExt, ExpanderRowExt, MessageDialogExt,
		PreferencesGroupExt, PreferencesRowExt,
	},
};
use chrono::{Datelike, Duration, Local, NaiveDateTime};
use glib::{clone, Cast};
use gtk::traits::{
	BoxExt, ButtonExt, GtkWindowExt, ListBoxRowExt, OrientableExt,
	ToggleButtonExt, WidgetExt,
};
use proto_rust::{TaskImportance, TaskStatus};
use relm4::{
	adw,
	factory::{AsyncFactoryComponent, FactoryView},
	gtk,
	gtk::prelude::EditableExt,
	loading_widgets::LoadingWidgets,
	prelude::DynamicIndex,
	AsyncFactorySender, RelmWidgetExt,
};

use crate::{fl, widgets::content::messages::ContentInput};

use super::messages::{TaskDetailsFactoryInput, TaskDetailsFactoryOutput};
use super::model::{
	DateDay, DateTpe, TaskDetailsFactoryInit, TaskDetailsFactoryModel,
};

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for TaskDetailsFactoryModel {
	type ParentWidget = gtk::Box;
	type ParentInput = ContentInput;
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
					adw::ExpanderRow {
						set_icon_name: Some("office-calendar-symbolic"),
						set_title: "Due date",
						set_subtitle: "Set the due date for this task",
						set_enable_expansion: true,
						#[name(due_date_label)]
						add_action = &gtk::Label {
							set_css_classes: &["accent"],
							#[watch]
							set_label: self.selected_due_date.as_deref().unwrap_or("No date set"),
							set_valign: gtk::Align::Center,
						},
						add_row = &gtk::Box {
							set_orientation: gtk::Orientation::Vertical,
							#[name(due_date_calendar)]
							gtk::Calendar {
								set_margin_all: 10,
								add_css_class: "card",
								connect_day_selected[sender] => move |calendar| {
									if let Ok(date) = calendar.date().format("%Y-%m-%dT%H:%M:%S") {
										if let Ok(date) = NaiveDateTime::from_str(date.as_str()) {
											sender.input(TaskDetailsFactoryInput::SetDueDate(Some(date)))
										}
									}
								}
							},
							gtk::Box {
								set_margin_all: 10,
								set_margin_bottom: 5,
								set_margin_top: 5,
								set_spacing: 10,
								gtk::Button {
									set_hexpand: true,
									set_label: "Today",
									connect_clicked[sender] => move |_| {
										sender.input(TaskDetailsFactoryInput::SetDate(DateTpe::DueDate, DateDay::Today));
									}
								},
								gtk::Button {
									set_hexpand: true,
									set_label: "Tomorrow",
									connect_clicked[sender] => move |_| {
										sender.input(TaskDetailsFactoryInput::SetDate(DateTpe::DueDate, DateDay::Tomorrow));
									}
								}
							},
							gtk::Button {
								set_margin_all:10,
								set_margin_top: 5,
								set_label: "None",
								connect_clicked[sender] => move |_| {
									sender.input(TaskDetailsFactoryInput::SetDate(DateTpe::DueDate, DateDay::None));
								}
							}
						}
					},
					adw::ExpanderRow {
						set_icon_name: Some("appointment-soon-symbolic"),
						set_title: "Reminder",
						set_subtitle: "Set a date to get a reminder",
						set_enable_expansion: true,
						#[name(reminder_label)]
						add_action = &gtk::Label {
							set_css_classes: &["accent"],
							#[watch]
							set_label: self.selected_reminder_date.as_deref().unwrap_or("No date set"),
							set_valign: gtk::Align::Center,
						},
						add_row = &gtk::Box {
							set_orientation: gtk::Orientation::Vertical,
							#[name(reminder_calendar)]
							gtk::Calendar {
								set_margin_all: 10,
								add_css_class: "card",
								connect_day_selected[sender] => move |calendar| {
									if let Ok(date) = calendar.date().format("%Y-%m-%dT%H:%M:%S") {
										if let Ok(date) = NaiveDateTime::from_str(date.to_string().as_str()) {
											sender.input(TaskDetailsFactoryInput::SetReminderDate(Some(date)))
										}
									}
								}
							},
							gtk::Box {
								set_margin_all: 10,
								set_margin_bottom: 5,
								set_margin_top: 5,
								set_spacing: 10,
								gtk::Button {
									set_hexpand: true,
									set_label: "Today",
									connect_clicked[sender] => move |_| {
										sender.input(TaskDetailsFactoryInput::SetDate(DateTpe::Reminder, DateDay::Today));
									}
								},
								gtk::Button {
									set_hexpand: true,
									set_label: "Tomorrow",
									connect_clicked[sender] => move |_| {
										sender.input(TaskDetailsFactoryInput::SetDate(DateTpe::Reminder, DateDay::Tomorrow));
									}
								}
							},
							gtk::Button {
								set_margin_all:10,
								set_margin_top: 5,
								set_label: "None",
								connect_clicked[sender] => move |_| {
									sender.input(TaskDetailsFactoryInput::SetDate(DateTpe::Reminder, DateDay::None));
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
			TaskDetailsFactoryInput::SetDate(calendar, date) => {
				let date = match date {
					DateDay::Today => Some(Local::now().naive_local()),
					DateDay::Tomorrow => {
						let date = Local::now()
							.checked_add_signed(Duration::days(1))
							.unwrap()
							.naive_local();
						Some(date)
					},
					DateDay::None => None,
				};
				match calendar {
					DateTpe::Reminder => {
						sender.input(TaskDetailsFactoryInput::SetReminderDate(date));
						if let Some(date) = date {
							self.task.reminder_date = Some(date.timestamp());
							self.selected_reminder_date =
								Some(date.format("%m/%d/%Y").to_string());
							widgets.reminder_calendar.set_year(date.year());
							widgets.reminder_calendar.set_month(date.month() as i32 - 1);
							widgets.reminder_calendar.set_day(date.day() as i32);
						} else {
							self.task.reminder_date = None;
							self.selected_reminder_date = None;
						}
					},
					DateTpe::DueDate => {
						sender.input(TaskDetailsFactoryInput::SetDueDate(date));
						if let Some(date) = date {
							self.task.due_date = Some(date.timestamp());
							self.selected_due_date =
								Some(date.format("%m/%d/%Y").to_string());
							widgets.due_date_calendar.set_year(date.year());
							widgets.due_date_calendar.set_month(date.month() as i32 - 1);
							widgets.due_date_calendar.set_day(date.day() as i32);
						} else {
							self.task.due_date = None;
							self.selected_due_date = None;
						}
					},
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
		}
		if self.task != self.original_task {
			self.dirty = true;
		}
		self.update_view(widgets, sender);
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			TaskDetailsFactoryOutput::CleanTaskEntry => ContentInput::CleanTaskEntry,
			TaskDetailsFactoryOutput::SaveTask(_, task, is_update) => {
				if is_update {
					ContentInput::UpdateTask(task)
				} else {
					ContentInput::AddTask(task)
				}
			},
			TaskDetailsFactoryOutput::HideFlap => ContentInput::HideFlap,
		};
		Some(output)
	}
}
