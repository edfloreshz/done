use relm4::gtk;

#[derive(Debug, Clone)]
pub struct TaskListEntryComponent {
	pub name: gtk::EntryBuffer,
	pub mode: TaskListEntryMode,
	pub label: String,
}

#[derive(Debug, Clone)]
pub enum TaskListEntryMode {
	New,
	Edit,
}
