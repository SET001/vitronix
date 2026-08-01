use crate::plugin::PluginPersistentState;
use indexmap::IndexMap;

pub trait PersistentState {
	type Stored;
	fn to_stored(&self) -> Self::Stored;
	fn from_stored(stored: Self::Stored) -> Self;
}

pub struct AppPersistentState {
	plugins: IndexMap<String, PluginPersistentState>,
}

pub fn get_config_path(app_name: &str) -> String {
	let config_dir = dirs::config_dir().expect("Failed to get config directory");
	let config_path = config_dir.join(app_name);
	config_path.to_str().unwrap().to_string()
}
pub struct PersistenceManager {}
impl PersistenceManager {
	fn load_app_state() -> AppPersistentState {
		let plugins = IndexMap::new();
		AppPersistentState { plugins }
	}
}
