use relm4::factory::AsyncFactoryVecDeque;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::fl;

use super::factory::SmartSidebarFactoryListModel;

#[derive(Debug)]
pub struct SmartSidebarListModel {
	pub smart_list_controller: AsyncFactoryVecDeque<SmartSidebarFactoryListModel>,
}

#[derive(Debug, EnumIter, Clone, PartialEq)]
pub enum SmartList {
	All,
	Today,
	Starred,
	Next7Days,
}

impl SmartList {
	pub fn list() -> Vec<Self> {
		SmartList::iter().collect()
	}

	pub fn name(&self) -> String {
		let all: &String = fl!("all");
		let today: &String = fl!("today");
		let starred: &String = fl!("starred");
		let next_7_days: &String = fl!("next-7-days");
		match self {
			SmartList::All => all.clone(),
			SmartList::Today => today.clone(),
			SmartList::Starred => starred.clone(),
			SmartList::Next7Days => next_7_days.clone(),
		}
	}

	pub fn description(&self) -> String {
		let all_desc: &String = fl!("all-desc");
		let today_desc: &String = fl!("today-desc");
		let starred_desc: &String = fl!("starred-desc");
		let next_7_days_desc: &String = fl!("next-7-days-desc");
		match self {
			SmartList::All => all_desc.clone(),
			SmartList::Today => today_desc.clone(),
			SmartList::Starred => starred_desc.clone(),
			SmartList::Next7Days => next_7_days_desc.clone(),
		}
	}

	pub fn icon(&self) -> &str {
		match self {
			SmartList::All => "clipboard",
			SmartList::Today => "sun-alt",
			SmartList::Starred => "star-rounded",
			SmartList::Next7Days => "table",
		}
	}
}
