use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
	resource::{Resource, ResourceId},
	view::{View, ViewDescriptor, ViewsRegistry},
};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Child {
	parent: u32,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct LayoutManager {
	containers: HashMap<String, ContainerParams>,
	#[serde(skip_serializing)]
	declared_ids: HashSet<String>,
}

impl LayoutManager {
	pub fn get(&self, container_id: &str) -> Option<&ContainerParams> {
		self.containers.get(container_id)
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
pub fn Container(id: String, params: ContainerParams) -> Element {
	let mut layout_manager = use_context::<Signal<LayoutManager>>();
	let resources = use_context::<Signal<crate::resource::Resources>>();
	match use_hook(|| layout_manager.write().get_or_register(id.clone(), params).cloned()) {
		Ok(container) => {
			let resources = resources.read();
			let panels: Vec<PanelParams> = container
				.panels
				.iter()
				.filter_map(|panel_id| {
					let panel = resources.get(panel_id).cloned();
					if panel.is_none() {
						warn!("Panel resource {} not found for container {}", panel_id.id, id);
					}
					panel
				})
				.collect();
			debug!("Rendering container {} with panels: {:?}", id, container.panels);
			rsx! {
				div{
					class: container.class.clone() + " panel",
					{id}
					for panel in panels.iter() {
						Panel{params: panel.clone()}
					}
				}
			}
		}
		Err(err) => rsx! {
			div {
				class: "panel",
				"{id}: Error: {err}"
			}
		},
	}
}

#[component]
pub fn Panel(params: PanelParams) -> Element {
	debug!("Rendering panel with params: {:?}", params);
	let views_registry = use_context::<Signal<ViewsRegistry>>();
	let views: Vec<View> = params
		.views
		.iter()
		.filter_map(|desc| {
			let factory = views_registry.read().get(desc.template.as_str()).copied();
			factory.map(|create_view| create_view(desc.params.clone())).or_else(|| {
				warn!("View resource {} not found for panel", desc.template);
				None
			})
		})
		.collect();
	rsx! {
		div {
			class: "panel",
			for view in views.iter() {
				div {
					{view.element.clone()}
				}
			}
		}
	}
}
