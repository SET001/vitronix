use std::path::Path;

use dioxus::prelude::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{PLUGIN_DIR_NAME, state::PersistentState};

pub type PluginComponent = fn() -> Element;
pub type PluginBuilder = fn() -> Plugin;
pub type PluginMap = IndexMap<String, (Plugin, PluginState)>;

#[derive(Clone)]
pub struct PluginDescriptor {
	pub builder: PluginBuilder,
	pub required: bool,
}

#[derive(Default, Clone)]
pub struct Plugin {
	pub name: &'static str,
	pub description: Option<&'static str>,
	pub entry: Option<PluginComponent>,
	pub load_config: Option<fn(data: &[u8])>,
	pub save_config: Option<fn() -> Vec<u8>>,
	pub dependencies: Vec<&'static str>,
}

#[derive(Clone)]
pub struct PluginState {
	pub is_enabled: bool,
	pub is_initialized: bool,
	pub is_required: bool,
}

#[derive(Serialize, Deserialize, Clone)]
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
			is_required: false, // set from PluginDescriptor for static plugins
		}
	}
}

pub fn plugin_dir() -> std::path::PathBuf {
	std::env::current_exe()
		.expect("failed to resolve executable path")
		.parent()
		.expect("executable has no parent directory")
		.join(PLUGIN_DIR_NAME)
}

pub fn scan_plugin_files(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
	#[cfg(target_os = "windows")]
	const DYN_LIB_EXT: &str = "dll";
	#[cfg(target_os = "macos")]
	const DYN_LIB_EXT: &str = "dylib";
	#[cfg(not(any(target_os = "windows", target_os = "macos")))]
	const DYN_LIB_EXT: &str = "so";

	std::fs::read_dir(dir)
		.map(|entries| {
			entries
				.filter_map(|e| e.ok())
				.map(|e| e.path())
				.filter(|p| p.extension().and_then(|ext| ext.to_str()) == Some(DYN_LIB_EXT))
				.collect()
		})
		.unwrap_or_default()
}

pub fn load_dyn_plugins(path: &Path) -> Vec<Plugin> {
	scan_plugin_files(path)
		.into_iter()
		.filter_map(|plugin_path| unsafe {
			match libloading::Library::new(&plugin_path) {
				Ok(lib) => {
					let plugin = {
						let create: libloading::Symbol<fn() -> Plugin> =
							lib.get(b"create_plugin").expect("create_plugin symbol not found");
						create()
					};
					std::mem::forget(lib);
					info!("loaded plugin {} from {}", plugin.name, plugin_path.display());
					Some(plugin)
				}
				Err(e) => {
					error!("failed to load {}: {e}", plugin_path.display());
					None
				}
			}
		})
		.collect()
}

pub fn use_plugins(config: &crate::runner::RunConfig, app_state: &crate::state::AppPersistentState) -> Signal<PluginMap> {
	use_hook(|| {
		let basic_plugins: Vec<(String, Plugin, PluginState)> = config
			.plugins
			.iter()
			.map(|d| {
				let plugin = (d.builder)();
				let is_enabled = app_state.plugins.get(plugin.name).map_or(true, |s| s.is_enabled);
				let state = PluginState {
					is_enabled,
					is_initialized: false,
					is_required: d.required,
				};
				(plugin.name.to_string(), plugin, state)
			})
			.collect();

		let dyn_plugins: Vec<(String, Plugin, PluginState)> = config
			.plugins_path
			.as_deref()
			.map(load_dyn_plugins)
			.unwrap_or_default()
			.into_iter()
			.map(|plugin| {
				let is_enabled = app_state.plugins.get(plugin.name).map_or(true, |s| s.is_enabled);
				let state = PluginState {
					is_enabled,
					is_initialized: false,
					is_required: false,
				};
				(plugin.name.to_string(), plugin, state)
			})
			.collect();

		let all: Vec<(String, Plugin, PluginState)> = [basic_plugins, dyn_plugins].concat();
		let lookup: PluginMap = all.into_iter().map(|(name, p, s)| (name, (p, s))).collect();

		// known plugins in app_state order first, then new ones appended
		let mut merged: PluginMap = app_state
			.plugins
			.keys()
			.filter_map(|name| lookup.get(name).map(|e| (name.clone(), e.clone())))
			.collect();
		for (name, entry) in lookup {
			merged.entry(name).or_insert(entry);
		}

		crate::state::save_app_state(
			&config.app_name,
			&crate::state::AppPersistentState {
				plugins: merged.iter().map(|(n, (_, ps))| (n.clone(), ps.to_stored())).collect(),
			},
		);
		Signal::new(merged)
	})
}
