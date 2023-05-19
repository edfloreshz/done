use done_local_storage::models::List;
use relm4::factory::AsyncFactoryVecDeque;
use relm4_icons::icon_name;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{factories::task_list::model::TaskListFactoryModel, fl};

#[derive(Debug)]
pub struct SidebarComponentModel {
	pub list_factory: AsyncFactoryVecDeque<TaskListFactoryModel>,
	pub extended: bool,
}

#[derive(Debug, Clone, EnumIter, PartialEq)]
pub enum SidebarList {
	All,
	Today,
	Starred,
	Next7Days,
	Custom(List),
}

impl SidebarList {
	pub fn list() -> Vec<SidebarList> {
		let mut list: Vec<SidebarList> = SidebarList::iter().collect();
		list.pop();
		list
	}

	pub fn name(&self) -> String {
		let all: &String = fl!("all");
		let today: &String = fl!("today");
		let starred: &String = fl!("starred");
		let next_7_days: &String = fl!("next-7-days");
		match self {
			SidebarList::All => all.clone(),
			SidebarList::Today => today.clone(),
			SidebarList::Starred => starred.clone(),
			SidebarList::Next7Days => next_7_days.clone(),
			SidebarList::Custom(list) => list.name.clone(),
		}
	}

	pub fn description(&self) -> String {
		let all_desc: &String = fl!("all-desc");
		let today_desc: &String = fl!("today-desc");
		let starred_desc: &String = fl!("starred-desc");
		let next_7_days_desc: &String = fl!("next-7-days-desc");
		match self {
			SidebarList::All => all_desc.clone(),
			SidebarList::Today => today_desc.clone(),
			SidebarList::Starred => starred_desc.clone(),
			SidebarList::Next7Days => next_7_days_desc.clone(),
			SidebarList::Custom(list) => list.description.clone(),
		}
	}

	pub fn icon(&self) -> Option<&str> {
		match self {
			SidebarList::All => Some(icon_name::CLIPBOARD),
			SidebarList::Today => Some(icon_name::IMAGE_ADJUST_BRIGHTNESS),
			SidebarList::Starred => Some(icon_name::STAR_FILLED_ROUNDED),
			SidebarList::Next7Days => Some(icon_name::WORK_WEEK),
			SidebarList::Custom(list) => list.icon.as_deref(),
		}
	}

	pub fn smart(&self) -> bool {
		!matches!(self, SidebarList::Custom(_))
	}
}
