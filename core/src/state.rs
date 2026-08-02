use crate::{APP_STATE_FILE, plugin::PluginPersistentState};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

pub trait PersistentState {
	type Stored;
	fn to_stored(&self) -> Self::Stored;
	fn from_stored(stored: Self::Stored) -> Self;
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AppPersistentState {
	pub plugins: IndexMap<String, PluginPersistentState>,
}

pub fn get_config_path(app_name: &str) -> std::path::PathBuf {
	dirs::config_dir().expect("failed to get config directory").join(app_name)
}

pub fn load_app_state(app_name: &str) -> AppPersistentState {
	let path = get_config_path(app_name).join(APP_STATE_FILE);
	let Ok(contents) = std::fs::read_to_string(&path) else {
		return AppPersistentState::default();
	};
	toml::from_str(&contents).unwrap_or_default()
}

pub fn save_app_state(app_name: &str, state: &AppPersistentState) {
	let dir = get_config_path(app_name);
	std::fs::create_dir_all(&dir).expect("failed to create config directory");
	let contents = toml::to_string(state).expect("failed to serialize app state");
	std::fs::write(dir.join(APP_STATE_FILE), contents).expect("failed to write app state");
}
