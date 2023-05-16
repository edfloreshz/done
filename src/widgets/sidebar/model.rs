use done_local_storage::models::List;
use relm4::factory::AsyncFactoryVecDeque;
use relm4_icons::icon_name;

use crate::{factories::task_list::model::TaskListFactoryModel, fl};

#[derive(Debug)]
pub struct SidebarComponentModel {
	pub list_factory: AsyncFactoryVecDeque<TaskListFactoryModel>,
	pub extended: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SidebarList {
	All,
	Today,
	Starred,
	Next7Days,
	Custom(List),
}

impl SidebarList {
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

	pub fn _description(&self) -> String {
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

	pub fn icon(&self) -> &str {
		match self {
			SidebarList::All => icon_name::CLIPBOARD,
			SidebarList::Today => icon_name::IMAGE_ADJUST_BRIGHTNESS,
			SidebarList::Starred => icon_name::STAR_FILLED_ROUNDED,
			SidebarList::Next7Days => icon_name::WORK_WEEK,
			SidebarList::Custom(list) => list.icon.as_ref().unwrap().as_str(),
		}
	}
}
