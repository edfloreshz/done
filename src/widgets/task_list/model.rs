use proto_rust::List;

use crate::application::plugin::Plugin;

#[derive(Debug, Clone, PartialEq, derive_new::new)]
pub struct ListFactoryModel {
	pub list: List,
	pub plugin: Plugin,
}

#[derive(derive_new::new)]
pub struct ListFactoryInit {
	pub plugin: Plugin,
	pub list: List,
}
