use dioxus::prelude::*;
use indexmap::IndexMap;

use crate::state::PersistentState;

pub type PluginComponent = fn() -> Element;
pub type PluginBuilder = fn() -> Plugin;

#[derive(Clone)]
pub struct PluginDescriptor {
	pub builder: PluginBuilder,
	pub required: bool,
}

#[derive(Default)]
pub struct Plugin {
	pub name: &'static str,
	pub description: Option<&'static str>,
	pub entry: Option<PluginComponent>,
	pub load_config: Option<fn(data: &[u8])>,
	pub save_config: Option<fn() -> Vec<u8>>,
	pub dependencies: Vec<&'static str>,
}

pub struct PluginState {
	pub is_enabled: bool,
	pub is_initialized: bool,
	pub is_required: bool,
}

pub struct PluginPersistentState {
	pub is_enabled: bool,
	pub is_required: bool,
}

impl PersistentState for PluginState {
	type Stored = PluginPersistentState;

	fn to_stored(&self) -> Self::Stored {
		PluginPersistentState {
			is_enabled: self.is_enabled,
			is_required: self.is_required,
		}
	}

	fn from_stored(stored: Self::Stored) -> Self {
		Self {
			is_enabled: stored.is_enabled,
			is_initialized: false,
			is_required: stored.is_required,
		}
	}
}
pub struct PluginManager {
	pub plugins: IndexMap<&'static str, (Plugin, PluginState)>,
}
