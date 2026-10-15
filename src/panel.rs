use dioxus::{core::Element, signals::default_impl};

use crate::widget::Widget;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum DisplayMode {
	Tabbed(ResourceId),
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

type ResourceId = String;

#[derive(Default, Clone, PartialEq)]
pub struct Branch {
	// id: ResourceId,
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
	// id: ResourceId,
	pub display_mode: DisplayMode,
	pub is_collapsed: Option<bool>,
	pub draggable: bool,
	pub layout_direction: LayoutDirection,
	/// Sizing along the parent branch's layout axis.
	pub main_sizing: PanelSizing,
	/// Sizing across the parent branch's layout axis.
	pub cross_sizing: CrossSizing,
	pub tools: Vec<Element>,
}

#[derive(Clone, PartialEq)]
pub enum Node {
	Branch(Branch),
	Leaf(Panel),
}
