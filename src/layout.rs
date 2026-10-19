use dioxus::prelude::*;

use crate::panel::{Branch, CrossSizing, LayoutDirection, Node, Panel, PanelSizing};

/// The axis along which a `Branch` arranges its children (derived from its
/// `layout_direction`). Needed to translate a child's `cross_sizing::Fixed`
/// into a concrete `height` (row parent) or `width` (column parent) style,
/// since cross-axis sizing is otherwise direction-agnostic.
#[derive(Clone, Copy, PartialEq)]
pub enum Axis {
	Row,
	Column,
}

impl From<&LayoutDirection> for Axis {
	fn from(direction: &LayoutDirection) -> Self {
		match direction {
			LayoutDirection::Left | LayoutDirection::Right => Axis::Row,
			LayoutDirection::Up | LayoutDirection::Down => Axis::Column,
		}
	}
}

#[component]
pub fn Layout(layout: Node) -> Element {
	debug!("rendering Layout component");
	// let plugins = use_context::<Signal<PluginMap>>();
	// let panel = plugins
	// 	.read()
	// 	.iter()
	// 	.find_map(
	// 		|(_, (plugin, state))| {
	// 			if state.is_enabled { plugin.layout.clone() } else { None }
	// 		},
	// 	)
	// 	.unwrap();
	// let config = use_context::<RunConfig>();

	// The root node has no parent branch to be cross-sized by, so its
	// `cross_sizing` is effectively moot (Stretch is the sane default here).
	rsx! {
		RenderNode{node: layout, parent_axis: Axis::Row}
	}
}
#[component]
fn RenderNode(node: Node, parent_axis: Axis) -> Element {
	match node {
		Node::Branch(branch) => rsx! { RenderBranch{branch, parent_axis} },
		Node::Leaf(panel) => rsx! { RenderPanel{panel, parent_axis} },
	}
}

/// Translates main-axis sizing into a tailwind class for static cases and an
/// inline style for runtime-computed proportions (tailwind can't generate
/// classes for values that only exist at runtime).
fn main_sizing_attrs(sizing: &PanelSizing) -> (&'static str, String) {
	match sizing {
		PanelSizing::Fill => ("flex-1 min-h-0 min-w-0", String::new()),
		PanelSizing::Content => ("flex-none min-h-0 min-w-0", String::new()),
		PanelSizing::Flex(weight) => ("min-h-0 min-w-0", format!("flex: {weight} {weight} 0%;")),
		PanelSizing::Fixed(px) => ("flex-none min-h-0 min-w-0", format!("flex-basis: {px}px;")),
	}
}

/// Translates cross-axis sizing into a tailwind class and an inline style.
/// `parent_axis` is the parent branch's own axis, needed only for `Fixed`
/// (height for a row parent, width for a column parent) — `Stretch`/`Content`
/// use `align-self`, which already adapts to whichever axis is "cross".
fn cross_sizing_attrs(sizing: &CrossSizing, parent_axis: Axis) -> (&'static str, String) {
	match sizing {
		CrossSizing::Stretch => ("self-stretch", String::new()),
		CrossSizing::Content => ("self-start", String::new()),
		CrossSizing::Fixed(px) => {
			let property = match parent_axis {
				Axis::Row => "height",
				Axis::Column => "width",
			};
			("flex-none self-stretch", format!("{property}: {px}px;"))
		}
	}
}

#[component]
pub fn RenderBranch(branch: Branch, parent_axis: Axis) -> Element {
	debug!("rendering PanelLayout component");
	let flex_class = match branch.layout_direction {
		LayoutDirection::Right => "flex flex-row",
		LayoutDirection::Left => "flex flex-row-reverse",
		LayoutDirection::Down => "flex flex-col",
		LayoutDirection::Up => "flex flex-col-reverse",
	};
	let own_axis = Axis::from(&branch.layout_direction);
	let (main_class, main_style) = main_sizing_attrs(&branch.main_sizing);
	let (cross_class, cross_style) = cross_sizing_attrs(&branch.cross_sizing, parent_axis);
	rsx! {
		div { class: "{flex_class} {main_class} {cross_class} branch {branch.class}", style: "{main_style} {cross_style}",
			for item in branch.children {
				RenderNode{node: item, parent_axis: own_axis}
			}
		}
	}
}

#[component]
pub fn RenderPanel(panel: Panel, parent_axis: Axis) -> Element {
	debug!("rendering PanelWidgetGroup component");
	let flex_class = match panel.layout_direction {
		LayoutDirection::Right => "flex flex-row",
		LayoutDirection::Left => "flex flex-row-reverse",
		LayoutDirection::Down => "flex flex-col",
		LayoutDirection::Up => "flex flex-col-reverse",
	};
	let (main_class, main_style) = main_sizing_attrs(&panel.main_sizing);
	let (cross_class, cross_style) = cross_sizing_attrs(&panel.cross_sizing, parent_axis);
	rsx! {
		div { class: "{flex_class} {main_class} {cross_class} panel {panel.class}", style: "{main_style} {cross_style}",
			for widget in panel.tools {
				{widget}
			}
		}
	}
}
