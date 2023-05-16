use done_local_storage::models::{List, Task};
use relm4::gtk;

#[derive(Debug)]
pub struct TaskEntryModel {
	pub task: Task,
	pub parent_list: Option<List>,
	pub buffer: gtk::EntryBuffer,
}
