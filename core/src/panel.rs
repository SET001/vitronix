use dioxus::signals::default_impl;

use crate::widget::Widget;

#[derive(Clone, Debug, PartialEq)]
pub enum DisplayMode {
	Tabbed(ResourceId),
	Stacked,
}
#[derive(Clone, Debug, PartialEq)]
pub enum LayoutDirection {
	Left,
	Right,
	Up,
	Down,
}

pub enum CrossSizing {
	Stretch,
	Fixed(f32),
	Content,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PanelSizing {
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
	None,
}

type ResourceId = String;

#[derive(Clone, Debug, PartialEq)]
pub enum Panel {
	Layout {
		id: ResourceId,
		layout_direction: LayoutDirection,
		drop_target: bool,
		children: Vec<Panel>,
		resizeable: ResizeAxis,
		size: Option<(f32, f32)>,
	},
	WidgetGroup {
		id: ResourceId,
		display_mode: DisplayMode,
		is_collapsed: Option<bool>,
		draggable: bool,
		layout_direction: LayoutDirection,
		tools: Vec<Widget>,
	},
	//	TODO:
	// pub main_sizing: PanelSizing,
	// pub cross_sizing: CrossSizing,
}
