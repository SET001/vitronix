use std::any::{Any, TypeId};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ResourceId<T> {
	pub id: String,
	pub _marker: std::marker::PhantomData<T>,
}

impl<T> ResourceId<T> {
	pub fn from_string(id: String) -> Self {
		Self {
			id,
			_marker: std::marker::PhantomData,
		}
	}
}
pub trait Resource {}

pub struct TypedResources<T: Resource> {
	pub resources: HashMap<String, T>,
}

impl<T: Resource> TypedResources<T> {
	pub fn get(&self, id: String) -> &T {
		&self.resources[&id]
	}
}

pub struct StoreEntry {
	pub type_name: &'static str,
	pub size: usize,
	pub count: usize,
	pub store: Box<dyn Any>,
	pub len: fn(&dyn Any) -> usize,
}

pub struct Resources {
	pub stores: HashMap<TypeId, StoreEntry>,
}

impl Resources {
	pub fn new() -> Self {
		Self { stores: HashMap::new() }
	}

	pub fn put<T: Resource + 'static>(&mut self, resource: T) -> ResourceId<T> {
		self.put_named(resource, Uuid::new_v4().to_string())
	}

	pub fn put_named<T: Resource + 'static>(&mut self, resource: T, name: String) -> ResourceId<T> {
		let store = self.stores.entry(TypeId::of::<T>()).or_insert_with(|| StoreEntry {
			type_name: std::any::type_name::<T>(),
			size: std::mem::size_of::<T>(),
			count: 0,
			store: Box::new(TypedResources::<T> {
				resources: HashMap::new(),
			}),
			len: |any| {
				any.downcast_ref::<TypedResources<T>>()
					.map(|store| store.resources.len())
					.unwrap_or(0)
			},
		});
		store
			.store
			.downcast_mut::<TypedResources<T>>()
			.unwrap()
			.resources
			.insert(name.clone(), resource);
		ResourceId {
			id: name,
			_marker: std::marker::PhantomData,
		}
	}

	pub fn get<T: Resource + 'static>(&self, id: &ResourceId<T>) -> Option<&T> {
		let type_id = TypeId::of::<T>();
		let boxed = self.stores.get(&type_id)?;
		boxed
			.store
			.downcast_ref::<TypedResources<T>>()
			.and_then(|store| store.resources.get(&id.id))
	}

	pub fn get_pair<T: Resource + Clone + 'static>(&self, id: &ResourceId<T>) -> Option<(ResourceId<T>, &T)> {
		self.get(id).map(|resource| ((*id).clone(), resource))
	}

	pub fn get_mut<T: Resource + 'static>(&mut self, id: &ResourceId<T>) -> Option<&mut T> {
		let type_id = TypeId::of::<T>();
		let boxed = self.stores.get_mut(&type_id)?;
		boxed
			.store
			.downcast_mut::<TypedResources<T>>()
			.and_then(|store| store.resources.get_mut(&id.id))
	}

	pub fn remove<T: Resource + 'static>(&mut self, id: &ResourceId<T>) -> Option<T> {
		let type_id = TypeId::of::<T>();
		let boxed = self.stores.get_mut(&type_id)?;
		boxed
			.store
			.downcast_mut::<TypedResources<T>>()
			.and_then(|store| store.resources.remove(&id.id))
	}
}
