use std::path::PathBuf;

use dioxus::prelude::*;
use indexmap::IndexMap;

use crate::{FRAMEWORK_NAME, PLUGIN_DIR_NAME, state::PersistentState};

pub type PluginComponent = fn() -> Element;
pub type PluginBuilder = fn() -> Plugin;

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

impl PluginManager {
	pub fn list_dyn_plugins() -> Vec<String> {
		todo!()
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

pub fn use_dyn_plugins(path: Option<PathBuf>) -> Signal<Vec<(Plugin, PluginState)>> {
	let plugins = use_hook(|| match path {
		Some(path) => {
			let plugin_names = scan_plugin_files(&path);
			plugin_names.iter().for_each(|plugin_path| unsafe {
				match libloading::Library::new(&plugin_path) {
					Ok(lib) => {
						let plugin = {
							let create: libloading::Symbol<fn() -> Plugin> =
								lib.get(b"create_plugin").expect("create_plugin symbol not found");
							create()
						};
						std::mem::forget(lib);
						info!("loaded plugin {} from {}", plugin.name, plugin_path.display());
					}
					Err(e) => {
						error!("[vitronix] failed to load {}: {e}", plugin_path.display());
					}
				}
			});
			// info!(
			// 	"{} found {} plugin files in {}",
			// 	FRAMEWORK_NAME,
			// 	plugin_names.len(),
			// 	path.display()
			// );
			vec![]
		}
		None => vec![],
	});
	Signal::new(plugins)
}
