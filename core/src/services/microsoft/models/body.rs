use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemBody {
	pub content: String,
	pub content_type: BodyType,
}

#[derive(
	Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "camelCase")]
pub enum BodyType {
	#[default]
	Text,
	Html,
}
