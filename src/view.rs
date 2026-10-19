use dioxus::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct View {
	pub id: Uuid,
	pub title: String,
	pub element: Element,
}

#[derive(Default, Clone)]
pub struct ViewManager {
	pub views: Signal<HashMap<Uuid, View>>,
}

impl ViewManager {
	pub fn open(&mut self, view_element: Element, title: String) {
		let view = View {
			id: Uuid::new_v4(),
			title: title,
			element: view_element,
		};
		self.views.write().insert(view.id, view);
	}

	pub fn close(&mut self, view_id: Uuid) {
		self.views.write().remove(&view_id);
	}
}
