use done_local_storage::service::Service;
use relm4::gtk;

#[derive(Debug)]
pub struct ListDialogComponent {
	pub selected_service: Option<Service>,
	pub name: gtk::EntryBuffer,
	pub mode: ListDialogMode,
	pub label: String,
}

#[derive(Debug, Clone)]
pub enum ListDialogMode {
	New,
	Edit,
}
