use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Collection<T> {
	pub value: Vec<T>,
}
