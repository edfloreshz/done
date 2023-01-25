use super::model::SmartList;

#[derive(Debug)]
pub enum SmartSidebarListInput {
	SelectSmartList(SmartList),
	Forward,
}

#[derive(Debug)]
pub enum SmartSidebarListOutput {
	SelectSmartList(SmartList),
	Forward,
}
