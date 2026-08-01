use dioxus::prelude::*;
use indexmap::IndexMap;

use crate::state::PersistentState;

pub type PluginComponent = fn() -> Element;
pub type PluginBuilder = fn() -> Plugin;

#[derive(Default)]
pub struct Plugin {
	pub name: &'static str,
	pub description: Option<&'static str>,
	pub entry: Option<PluginComponent>,
	pub load_config: Option<fn(data: &[u8])>,
	pub save_config: Option<fn() -> Vec<u8>>,
}

pub struct PluginState {
	pub is_enabled: bool,
	pub is_initialized: bool,
}

pub struct PluginPersistentState {
	pub is_enabled: bool,
}

impl PersistentState for PluginState {
	type Stored = PluginPersistentState;

	fn to_stored(&self) -> Self::Stored {
		PluginPersistentState {
			is_enabled: self.is_enabled,
		}
	}

	fn from_stored(stored: Self::Stored) -> Self {
		Self {
			is_enabled: stored.is_enabled,
			is_initialized: false,
		}
	}
}
pub struct PluginManager {
	pub plugins: IndexMap<&'static str, (Plugin, PluginState)>,
}
