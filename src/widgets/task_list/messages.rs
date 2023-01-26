use relm4::prelude::DynamicIndex;

use super::model::ListFactoryModel;

#[derive(Debug)]
pub enum ListFactoryInput {
	Select,
	Delete(DynamicIndex),
	Rename(String),
	ChangeIcon(String),
}

#[derive(Debug)]
pub enum ListFactoryOutput {
	Select(Box<ListFactoryModel>),
	DeleteTaskList(DynamicIndex, String),
	Forward,
	Notify(String),
}
