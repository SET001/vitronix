use crate::resource::{Resource, ResourceId};
use dioxus::prelude::*;
use dioxus_sdk_storage::{LocalStorage, StorageBacking, StorageEntryTrait, get_from_storage};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{collections::HashMap, sync::Arc};
use toml::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct View {
	// pub params: T,
	pub element: Element,
	// pub closable: bool,
}
impl Resource for View {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewDescriptor {
	pub template: String,
	// pub instance_id: Option<ResourceId<View>>,
	pub params: Value,
}

pub type ViewFactory = Arc<dyn Fn(Value) -> Result<View, String>>;

impl ViewDescriptor {
	pub fn with_params(mut self, params: Value) -> Self {
		self.params = params;
		self
	}
}

#[derive(Clone)]
pub struct ViewsRegistry {
	registry: HashMap<String, ViewFactory>,
}

impl ViewsRegistry {
	pub fn new() -> Self {
		Self {
			registry: HashMap::new(),
		}
	}

	pub fn register<T: Default + Serialize + DeserializeOwned + 'static>(
		&mut self,
		view_name: &str,
		view: fn(T) -> View,
	) -> ViewDescriptor {
		debug!("Registering view {} in ViewsRegistry", view_name);
		let factory = move |value: Value| -> Result<View, String> {
			let params: T = value.try_into().map_err(|e| e.to_string())?;
			Ok(view(params))
		};
		if let None = self.registry.get(view_name) {
			self.registry.insert(view_name.into(), Arc::new(factory));
		}
		ViewDescriptor {
			template: view_name.into(),
			params: Value::try_from(T::default()).unwrap(),
		}
	}

	pub fn get(&self, view_name: &str) -> Option<ViewFactory> {
		self.registry.get(view_name).cloned()
	}
}

/// A `StorageEntry` whose backing `Signal` is explicitly bound to a chosen `owner` scope
/// instead of "whatever scope is currently executing" (which is what `dioxus_sdk_storage`'s
/// own `new_persistent`/`new_storage_entry` do internally). Views are often created from a
/// handler defined in one tab (e.g. a button inside `Description`) but consumed by a
/// sibling tab (e.g. `Editor`) that isn't a descendant of it. If the originating tab is later
/// hidden/unmounted, a signal tied to its scope is dropped out from under the sibling still
/// using it, causing a panic. Binding to a long-lived scope (e.g. `ScopeId::APP`) avoids that.
#[derive(Clone)]
struct ScopedStorageEntry<T> {
	key: String,
	data: Signal<T>,
}

impl<T> StorageEntryTrait<LocalStorage, T> for ScopedStorageEntry<T>
where
	T: Serialize + DeserializeOwned + Clone + Send + Sync + PartialEq + 'static,
{
	fn save(&self) {
		LocalStorage::set(self.key.clone(), &*self.data.read());
	}

	fn update(&mut self) {
		if let Some(value) = LocalStorage::get(&self.key) {
			self.data.set(value);
		}
	}

	fn key(&self) -> &String {
		&self.key
	}

	fn data(&self) -> &Signal<T> {
		&self.data
	}
}

/// Creates a signal backed by file storage (see `dioxus_sdk_storage::set_dir!`) that is owned
/// by `owner` rather than the scope that happens to call this function. Use `ScopeId::APP` for
/// view state so it survives regardless of which tab's handler created the view.
pub fn persistent_in_scope<T>(key: String, owner: ScopeId, init: impl FnOnce() -> T) -> Signal<T>
where
	T: Serialize + DeserializeOwned + Clone + Send + Sync + PartialEq + 'static,
{
	let initial = get_from_storage::<LocalStorage, T>(key.clone(), init);
	let entry = ScopedStorageEntry {
		key,
		data: Signal::new_in_scope(initial, owner),
	};
	entry.save_to_storage_on_change();
	entry.data
}

// #[derive(Default, Clone)]
// pub struct ViewManager {
// 	pub views: Signal<Vec<View>>,
// }

// impl ViewManager {
// 	pub fn add(&mut self, view_element: Element, title: String) {
// 		let view = View {
// 			id: Uuid::new_v4(),
// 			title,
// 			element: view_element,
// 		};
// 		self.views.write().push(view);
// 	}

// 	pub fn close(&mut self, view_id: Uuid) {
// 		self.views.write().retain(|view| view.id != view_id);
// 	}
// }
