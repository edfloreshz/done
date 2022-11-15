use done_core::services::provider::{List, Task};
use chrono::NaiveDateTime;
use relm4::{ComponentParts, ComponentSender, gtk, gtk::prelude::{
    EntryExt,
    OrientableExt,
    WidgetExt,
    EntryBufferExtManual,
    ButtonExt,
    BoxExt,
    EditableExt
}, RelmWidgetExt, Component};
use crate::fl;

#[derive(Debug)]
pub struct NewTask {
    task: Task,
    parent_list: Option<List>,
}

#[derive(Debug)]
pub enum NewTaskEvent {
    AddToMyDay,
    SetTitle(String),
    SetReminder(NaiveDateTime),
    SetDueDate(NaiveDateTime),
    AddNote(String),
    AddTask,
    SetParentList(Option<List>)
}

#[derive(Debug)]
pub enum NewTaskOutput {
    AddTask(Task)
}

#[relm4::component(pub)]
impl Component for NewTask {
    type CommandOutput = ();
    type Input = NewTaskEvent;
    type Output = NewTaskOutput;
    type Init = Option<List>;
    type Widgets = NewTaskWidgets;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_margin_all: 12,
            set_spacing: 5,
            #[name(entry)]
            gtk::Entry {
                set_hexpand: true,
                #[watch]
                set_visible: true,
                set_icon_from_icon_name: (gtk::EntryIconPosition::Primary, Some("value-increase-symbolic")),
                set_placeholder_text: Some(fl!("new-task")),
                set_height_request: 42,
                // #[watch]
                // set_text: &model.task.title,
                connect_changed[sender] => move |entry| {
                    let buffer = entry.buffer();
                    sender.input(NewTaskEvent::SetTitle(buffer.text()));
                }
            },
            gtk::Button {
                set_icon_name: "daytime-sunrise-symbolic",
                connect_clicked[sender] => move |_| {
                    sender.input(NewTaskEvent::AddToMyDay)
                }
            },
            gtk::Button {
                set_icon_name: "appointment-soon-symbolic",
                connect_clicked[sender] => move |_| {
                    sender.input(NewTaskEvent::SetReminder(chrono::Utc::now().naive_utc()))
                }
            },
            gtk::Button {
                set_icon_name: "office-calendar-symbolic",
                connect_clicked[sender] => move |_| {
                    sender.input(NewTaskEvent::SetDueDate(chrono::Utc::now().naive_utc()))
                }
            },
            gtk::Button {
                set_icon_name: "text-editor-symbolic",
                connect_clicked[sender] => move |_| {
                    sender.input(NewTaskEvent::AddNote(String::new()))
                }
            },
            gtk::Button {
                set_icon_name: "mail-send-symbolic",
                connect_clicked[sender] => move |_| {
                    sender.input(NewTaskEvent::AddTask)
                }
            }
        }
    }

    fn init(init: Self::Init, root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = NewTask {
            task: Task::new(String::new(), String::new()),
            parent_list: init
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update_with_view(&mut self, widgets: &mut Self::Widgets, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            NewTaskEvent::AddToMyDay => (), // TODO: Add to my day.
            NewTaskEvent::SetTitle(title) => self.task.title = title,
            NewTaskEvent::SetReminder(reminder) => {
                self.task.reminder_date = Some(reminder.timestamp());
                self.task.is_reminder_on = true;
            },
            NewTaskEvent::SetDueDate(due) => self.task.due_date = Some(due.timestamp()),
            NewTaskEvent::AddNote(note) => self.task.body = Some(note),
            NewTaskEvent::AddTask => if !self.task.title.is_empty() && self.parent_list.is_some() {
                sender.output(NewTaskOutput::AddTask(self.task.clone()));
                self.task = Task::new(String::new(), self.parent_list.as_ref().unwrap().id.clone());
                widgets.entry.set_text("")
            },
            NewTaskEvent::SetParentList(list) => self.parent_list = list
        }
    }
}