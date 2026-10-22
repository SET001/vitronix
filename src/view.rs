use crate::resource::Resource;
use dioxus::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct View {
	pub title: String,
	pub element: Element,
}

impl Resource for View {}

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
