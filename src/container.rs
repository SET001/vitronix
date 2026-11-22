use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use dioxus_dnd_kit::prelude::{ConnectedLists, DndContext, DragLayout, DraggableHandler, DraggableView, Droppable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
	resource::{Resource, ResourceId},
	view::{ViewDescriptor, ViewsRegistry},
};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Child {
	parent: u32,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct LayoutManager {
	containers: HashMap<String, ContainerParams>,
	#[serde(skip)]
	declared_ids: HashSet<String>,
}

impl LayoutManager {
	pub fn get(&self, container_id: &str) -> Option<&ContainerParams> {
		self.containers.get(container_id)
	}

	pub fn set_panels(&mut self, container_id: &str, panel_ids: Vec<String>) {
		let Some(container) = self.containers.get_mut(container_id) else {
			warn!("Cannot update panels for unknown container {}", container_id);
			return;
		};
		let panels: Vec<ResourceId<PanelParams>> = panel_ids.into_iter().map(ResourceId::from_string).collect();
		if container.panels != panels {
			container.panels = panels;
		}
	}

	pub fn get_or_register(&mut self, container_id: String, container: ContainerParams) -> Result<&mut ContainerParams, String> {
		info!("Registering container with id {}", container_id);
		match self.declared_ids.insert(container_id.clone()) {
			true => Ok(self.containers.entry(container_id.clone()).or_insert(container)),
			false => Err(format!("Container with id {} is already registered", container_id)),
		}
	}
}

// pub fn PanelsApp() {
// 	let layou_manager = use_storage::<LocalStorage, LayoutManager>("layout_manager".to_string(), || LayoutManager {
// 		containers: HashMap::new(),
// 	});
// }

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct PanelParams {
	pub class: String,
	// id: ResourceId,
	// pub display_mode: DisplayMode,
	pub is_collapsed: Option<bool>,
	// pub layout_direction: LayoutDirection,
	/// Sizing along the parent branch's layout axis.
	/// Sizing across the parent branch's layout axis.
	// pub views: Vec<ViewDescriptor>,
	pub views: Vec<ViewDescriptor>,
}
impl Resource for PanelParams {}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct ContainerParams {
	pub class: String,

	// pub panels: Vec<PanelParams>,	//	can't contain panels list here because at the time when panels declared, the container might not be registered yet. containers came from app layout
	pub panels: Vec<ResourceId<PanelParams>>,
	// pub layout_direction: LayoutDirection,
	pub drop_target: bool, // pub children: Vec<Node>,
	// pub resizeable: Option<ResizeAxis>,
	// Sizing along this branch's own layout axis, from its parent's perspective.
	// pub main_sizing: PanelSizing,
	// Sizing across this branch's own layout axis, from its parent's perspective.
	// pub cross_sizing: CrossSizing,
	pub size: Option<(f32, f32)>,
}
impl Resource for ContainerParams {}

#[component]
pub fn Panel(params: PanelParams) -> Element {
	rsx! {
		div { class: "{params.class} panel",
			for descriptor in params.views.iter() {
				ViewBody { descriptor: descriptor.clone() }
			}
		}
	}
}

#[derive(Clone, PartialEq)]
struct ViewDndItem {
	id: String,
	descriptor: ViewDescriptor,
}

/// Wrap related containers in this component to enable panel and view drag-and-drop between them.
#[component]
pub fn WorkspaceDnd(children: Element) -> Element {
	rsx! {
		DraggableView::<String> {
			class: "flex flex-col flex-1 min-h-0 min-w-0",
			key_gen: |id: String| id,
			render: |id: String| rsx! { PanelContent { id, draggable: true } },
			overlay_render: |id: String| rsx! {
				div { class: "panel panel-drag-overlay", "Panel {id}" }
			},
			ConnectedLists::<String> {
				DraggableView::<ViewDndItem> {
					class: "flex flex-col flex-1 min-h-0 min-w-0",
					key_gen: |item: ViewDndItem| item.id,
					render: |item: ViewDndItem| rsx! { ViewDndContent { item } },
					overlay_render: |item: ViewDndItem| rsx! { ViewBody { descriptor: item.descriptor } },
					ConnectedLists::<ViewDndItem> {
						{children}
					}
				}
			}
		}
	}
}

#[component]
pub fn Container(id: String, params: ContainerParams) -> Element {
	let mut layout_manager = use_context::<Signal<LayoutManager>>();
	let registration = use_hook(|| layout_manager.write().get_or_register(id.clone(), params).cloned());
	let initial_panels = registration
		.as_ref()
		.ok()
		.map(|container| {
			container
				.panels
				.iter()
				.map(|panel_id| panel_id.id.clone())
				.collect::<Vec<_>>()
		})
		.unwrap_or_default();
	let panels = use_signal(|| initial_panels);
	let dnd_enabled = try_use_context::<DndContext<String>>().is_some();

	if registration.is_ok() {
		let container_id = id.clone();
		use_effect(move || {
			layout_manager.write().set_panels(&container_id, panels());
		});
	}

	match registration {
		Ok(container) if dnd_enabled => rsx! {
			div { class: "container-shell flex flex-col flex-1 min-h-0 min-w-0",
				div { class: "container-title", "{id}" }
				Droppable::<String> {
					id: "container-panels-{id}",
					class: "{container.class} panel container-panels flex flex-col flex-1 min-h-24 min-w-0 gap-4",
					items: panels,
					layout: DragLayout::Relative,
				}
			}
		},
		Ok(container) => rsx! {
			div { class: "{container.class} panel",
				"{id}"
				for panel_id in container.panels.iter() {
					PanelContent { key: "{panel_id.id}", id: panel_id.id.clone(), draggable: false }
				}
			}
		},
		Err(err) => rsx! {
			div {
				class: "panel",
				"{id}: Error: {err}"
			}
		},
	}
}

#[component]
fn PanelContent(id: String, draggable: bool) -> Element {
	let resources = use_context::<Signal<crate::resource::Resources>>();
	let resource_id = ResourceId::<PanelParams>::from_string(id.clone());
	let params = resources.read().get(&resource_id).cloned();
	let view_items = use_signal(|| {
		params
			.as_ref()
			.map(|params| params.views.clone())
			.unwrap_or_default()
			.into_iter()
			.map(|descriptor| ViewDndItem {
				id: Uuid::new_v4().to_string(),
				descriptor,
			})
			.collect::<Vec<_>>()
	});

	if try_use_context::<DndContext<ViewDndItem>>().is_some() {
		let mut resources = resources;
		let panel_id = resource_id.clone();
		use_effect(move || {
			let descriptors = view_items.read().iter().map(|item| item.descriptor.clone()).collect();
			if let Some(panel) = resources.write().get_mut(&panel_id) {
				panel.views = descriptors;
			}
		});
	}

	let Some(params) = params else {
		warn!("Panel resource {} not found", id);
		return rsx! { div { class: "panel", "Panel {id} is unavailable" } };
	};

	rsx! {
		div { class: "{params.class} panel panel-content flex flex-col min-h-0 min-w-0",
			if draggable {
				div { class: "panel-drag-bar",
					DraggableHandler::<String> { class: "panel-drag-handle cursor-grab select-none", item: id.clone(), "⠿" }
				}
			}
			if try_use_context::<DndContext<ViewDndItem>>().is_some() {
				Droppable::<ViewDndItem> {
					id: "panel-views-{id}",
					class: "panel-views flex flex-col flex-1 min-h-12 min-w-0 gap-2",
					items: view_items,
					layout: DragLayout::Relative,
				}
			} else {
				for descriptor in params.views.iter() {
					ViewBody { descriptor: descriptor.clone() }
				}
			}
		}
	}
}

#[component]
fn ViewDndContent(item: ViewDndItem) -> Element {
	rsx! {
		div { class: "view-entry",
			DraggableHandler::<ViewDndItem> { class: "view-drag-handle cursor-grab select-none", item: item.clone(), "⠿" }
			ViewBody { descriptor: item.descriptor }
		}
	}
}

#[component]
fn ViewBody(descriptor: ViewDescriptor) -> Element {
	let views_registry = use_context::<Signal<ViewsRegistry>>();
	let factory = views_registry.read().get(descriptor.template.as_str());
	match factory {
		Some(create_view) => match create_view(descriptor.params) {
			Ok(view) => rsx! { div { class: "view-content", {view.element} } },
			Err(err) => {
				error!("Failed to create view {}: {}", descriptor.template, err);
				rsx! { div { class: "view-error", "Failed to create view {descriptor.template}: {err}" } }
			}
		},
		None => {
			warn!("View resource {} not found for panel", descriptor.template);
			rsx! { div { class: "view-error", "View {descriptor.template} is unavailable" } }
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn set_panels_persists_the_new_order() {
		let mut layout = LayoutManager::default();
		layout
			.get_or_register(
				"left".into(),
				ContainerParams {
					panels: vec![
						ResourceId::from_string("panel-a".into()),
						ResourceId::from_string("panel-b".into()),
					],
					..Default::default()
				},
			)
			.unwrap();

		layout.set_panels("left", vec!["panel-b".into(), "panel-a".into()]);

		let ids: Vec<_> = layout
			.get("left")
			.unwrap()
			.panels
			.iter()
			.map(|panel| panel.id.as_str())
			.collect();
		assert_eq!(ids, ["panel-b", "panel-a"]);
	}
}
