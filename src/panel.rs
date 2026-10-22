use dioxus::prelude::*;

use crate::{
	resource::{Resource, ResourceId, Resources},
	view::View,
};

#[derive(Clone, Debug, PartialEq, Default)]
pub enum DisplayMode {
	Tabbed,
	#[default]
	Stacked,
}
#[derive(Clone, Debug, PartialEq, Default)]
pub enum LayoutDirection {
	Left,
	Right,
	Up,
	#[default]
	Down,
}

/// How a node sizes itself across the parent `Branch`'s layout axis
/// (the opposite of its `layout_direction`) — e.g. height in a row, width in a column.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum CrossSizing {
	#[default]
	Stretch,
	Fixed(f32),
	Content,
}

/// How a node sizes itself along the parent `Branch`'s layout axis
/// (the direction `layout_direction` splits children into).
#[derive(Clone, Debug, PartialEq, Default)]
pub enum PanelSizing {
	#[default]
	Fill,
	Flex(f32),
	Fixed(f32),
	Content,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ResizeAxis {
	Horizontal,
	Vertical,
	Both,
}

#[derive(Default, Clone, PartialEq)]
pub struct Branch {
	// id: ResourceId,
	pub class: String,
	pub layout_direction: LayoutDirection,
	pub drop_target: bool,
	pub children: Vec<Node>,
	pub resizeable: Option<ResizeAxis>,
	/// Sizing along this branch's own layout axis, from its parent's perspective.
	pub main_sizing: PanelSizing,
	/// Sizing across this branch's own layout axis, from its parent's perspective.
	pub cross_sizing: CrossSizing,
	pub size: Option<(f32, f32)>,
}

#[derive(Default, Clone, PartialEq)]
pub struct Panel {
	pub class: String,
	// id: ResourceId,
	pub display_mode: DisplayMode,
	pub is_collapsed: Option<bool>,
	pub layout_direction: LayoutDirection,
	/// Sizing along the parent branch's layout axis.
	/// Sizing across the parent branch's layout axis.
	pub tools: Vec<ResourceId<View>>,
}

impl Resource for Panel {}

#[derive(Clone, PartialEq)]
pub enum Node {
	Branch(Branch),
	Leaf(Panel),
}

#[allow(non_snake_case)]
pub fn RenderPanel(panel_id: ResourceId<Panel>) -> Element {
	let mut resources: Signal<Resources> = use_context::<Signal<Resources>>();
	let tools: Vec<ResourceId<View>> = resources
		.read()
		.get(&panel_id)
		.map(|panel| panel.tools.clone())
		.unwrap_or_default();

	let mut active = use_signal(|| None::<ResourceId<View>>);
	let current = active().filter(|id| tools.contains(id)).or_else(|| tools.first().cloned());

	let content = current
		.as_ref()
		.and_then(|id| resources.read().get(id).map(|view| view.element.clone()));

	let tabs = tools.iter().cloned().map(|view_id| {
		let title = resources
			.read()
			.get(&view_id)
			.map(|view| view.title.clone())
			.unwrap_or_default();
		let is_active = current.as_ref() == Some(&view_id);
		let select_id = view_id.clone();
		let close_id = view_id.clone();
		let panel_id = panel_id.clone();
		rsx! {
			div {
				key: "{view_id.id}",
				class: "flex flex-row w-fit panel",
				class: if is_active { "tab-active" },
				onclick: move |_| active.set(Some(select_id.clone())),
				"{title}"
				button {
					onclick: move |evt| {
						evt.stop_propagation();
						resources.write().remove(&close_id);
						if let Some(panel) = resources.write().get_mut(&panel_id) {
							panel.tools.retain(|id| id != &close_id);
						}
					},
					"[X]"
				}
			}
		}
	});

	rsx! {
		div {
			class: "flex flex-col flex-1 w-full min-h-0 gap-0 h-full",
			div {
				class: "flex flex-row gap-7 panel rounded-b-none",
				{tabs}
			}
			div {
				class: "flex flex-col flex-1 min-h-0 w-full panel rounded-t-none gap-7",
				if let Some(element) = content {
					{element}
				}
			}
		}
	}
}
