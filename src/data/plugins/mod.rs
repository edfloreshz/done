use crate::data::plugins::all::AllProvider;
use crate::data::plugins::local::LocalProvider;
use crate::data::plugins::next7days::Next7DaysProvider;
use crate::data::plugins::starred::StarredProvider;
use crate::data::plugins::today::TodayProvider;
use crate::data::traits::provider::ReflectProvider;
use crate::Provider;
use bevy_reflect::{Reflect, Struct, TypeRegistry};

pub mod all;
pub mod local;
pub mod next7days;
pub mod starred;
pub mod today;

#[derive(Debug, Default, Reflect)]
pub struct Plugins {
	pub(crate) all: AllProvider,
	pub(crate) today: TodayProvider,
	pub(crate) starred: StarredProvider,
	pub(crate) next: Next7DaysProvider,
	pub(crate) local: LocalProvider,
}

impl Plugins {
	pub fn get_providers<'a>(&'a self) -> Vec<&'a dyn Provider> {
		let mut providers: Vec<&'a dyn Provider> = vec![];
		let mut type_registry = TypeRegistry::default();
		type_registry.register::<LocalProvider>();
		type_registry.register::<AllProvider>();
		type_registry.register::<TodayProvider>();
		type_registry.register::<StarredProvider>();
		type_registry.register::<Next7DaysProvider>();
		for value in self.iter_fields() {
			let ref_value: &'a dyn Reflect = value;
			let data = type_registry
				.get_type_data::<ReflectProvider>(ref_value.type_id())
				.unwrap();
			let provider: &'a dyn Provider = data.get(&*ref_value).unwrap();
			providers.push(provider)
		}
		providers
	}
	pub fn get_provider<'a>(&'a self, id: &str) -> &'a dyn Provider {
		let providers = self.get_providers();
		let provider = *providers
			.iter()
			.find(|provider| provider.get_id() == id)
			.unwrap();
		provider
	}
}
