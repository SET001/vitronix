use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use dioxus_dnd_kit::prelude::{ConnectedLists, DragLayout, DraggableHandler, DraggableView, Droppable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
	resource::Resource,
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

	pub fn set_panels(&mut self, container_id: &str, panels: Vec<PanelParams>) {
		let Some(container) = self.containers.get_mut(container_id) else {
			warn!("Cannot update panels for unknown container {}", container_id);
			return;
		};
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

pub struct PanelTemplateRegistry {
	pub templates: HashMap<String, PanelParams>,
}
impl PanelTemplateRegistry {
	pub fn new() -> Self {
		Self {
			templates: HashMap::new(),
		}
	}

	pub fn register(&mut self, template_name: &str, params: PanelParams) -> Result<(), String> {
		if self.templates.contains_key(template_name) {
			Err(format!("Panel template {} is already registered", template_name))
		} else {
			self.templates.insert(template_name.to_string(), params);
			Ok(())
		}
	}

	pub fn get(&self, template_name: &str) -> Option<&PanelParams> {
		self.templates.get(template_name)
	}
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct PanelParams {
	pub name: String,
	pub class: String,
	pub dnd_enabled: bool,
	// id: ResourceId,
	// pub display_mode: DisplayMode,
	pub is_collapsed: Option<bool>,
	// pub layout_direction: LayoutDirection,
	// Sizing along the parent branch's layout axis.
	// Sizing across the parent branch's layout axis.
	// pub views: Vec<ViewDescriptor>,
	pub views: Vec<ViewDescriptor>,
}

impl Resource for PanelParams {}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct ContainerParams {
	pub class: String,
	pub panels: Vec<PanelParams>,
	// pub layout_direction: LayoutDirection,
	pub dnd_enabled: bool,
	// pub resizeable: Option<ResizeAxis>,
	// Sizing along this branch's own layout axis, from its parent's perspective.
	// pub main_sizing: PanelSizing,
	// Sizing across this branch's own layout axis, from its parent's perspective.
	// pub cross_sizing: CrossSizing,
	pub size: Option<(f32, f32)>,
}
impl Resource for ContainerParams {}

#[derive(Clone, PartialEq)]
struct ViewDndItem {
	id: String,
	descriptor: ViewDescriptor,
}

#[component]
pub fn AppDndRoot(children: Element) -> Element {
	rsx! {
		DraggableView::<PanelParams> {
			class: "flex flex-col flex-1 min-h-0 min-w-0",
			key_gen: |panel: PanelParams| panel.name,
			render: |panel: PanelParams| rsx! { Panel { panel, panel_draggable: true } },
			overlay_render: |panel: PanelParams| rsx! {
				div { class: "panel panel-drag-overlay", "Panel {panel.name}" }
			},
			ConnectedLists::<PanelParams> {
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
	let panels = use_signal(|| {
		registration
			.as_ref()
			.ok()
			.map(|container| container.panels.iter().map(|panel| panel.clone()).collect::<Vec<_>>())
			.unwrap_or_default()
	});

	if registration.is_ok() {
		let container_id = id.clone();
		use_effect(move || {
			layout_manager.write().set_panels(&container_id, panels());
		});
	}

	match registration {
		Ok(container) if container.dnd_enabled => rsx! {
			div { class: "container-shell flex flex-col flex-1 min-h-0 min-w-0",
				div { class: "container-title", "{id}" }
				Droppable::<PanelParams> {
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
				for panel in container.panels.iter() {
					Panel { panel: panel.clone(), panel_draggable: false }
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
fn Panel(panel: PanelParams, panel_draggable: bool) -> Element {
	let view_items: Signal<Vec<ViewDndItem>> = use_signal(|| {
		panel
			.views
			.iter()
			.cloned()
			.map(|descriptor| ViewDndItem {
				id: Uuid::new_v4().to_string(),
				descriptor,
			})
			.collect()
	});

	rsx! {
		div { class: "{panel.class} panel panel-content flex flex-col min-h-0 min-w-0",
			if panel_draggable {
				div { class: "panel-drag-bar",
					DraggableHandler::<PanelParams> { class: "panel-drag-handle cursor-grab select-none", item: panel.clone(), "⠿" }
				}
			}
			if panel.dnd_enabled {
				Droppable::<ViewDndItem> {
					id: format!("panel-views-{}", panel.name),
					class: "panel-views flex flex-col flex-1 min-h-12 min-w-0 gap-2",
					items: view_items,
					layout: DragLayout::Relative,
				}
			} else {
				for descriptor in panel.views.iter() {
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
					panels: vec![test_panel("panel-a"), test_panel("panel-b")],
					..Default::default()
				},
			)
			.unwrap();

		layout.set_panels("left", vec![test_panel("panel-b"), test_panel("panel-a")]);

		let ids: Vec<_> = layout
			.get("left")
			.unwrap()
			.panels
			.iter()
			.map(|panel| panel.name.as_str())
			.collect();
		assert_eq!(ids, ["panel-b", "panel-a"]);
	}

	fn test_panel(name: &str) -> PanelParams {
		PanelParams {
			name: name.into(),
			class: String::new(),
			dnd_enabled: true,
			is_collapsed: None,
			views: Vec::new(),
		}
	}
}
