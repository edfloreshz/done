use relm4::factory::AsyncFactoryVecDeque;

use crate::{factories::service::ServiceModel, fl};

#[derive(Debug)]
pub struct SidebarComponentModel {
	pub service_factory: AsyncFactoryVecDeque<ServiceModel>,
	pub extended: bool,
}
