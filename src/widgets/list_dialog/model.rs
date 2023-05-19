use relm4::gtk;

#[derive(Debug, Clone)]
pub struct ListDialogComponent {
	pub name: gtk::EntryBuffer,
	pub mode: ListDialogMode,
	pub label: String,
}

#[derive(Debug, Clone)]
pub enum ListDialogMode {
	New,
	Edit,
}
