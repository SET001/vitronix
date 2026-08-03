use dioxus::prelude::*;

use crate::{
	panel::{DisplayMode, LayoutDirection, Panel, ResizeAxis},
	plugin::PluginMap,
	widget::Widget,
};

#[component]
pub fn Layout() -> Element {
	debug!("rendering Layout component");
	let plugins = use_context::<Signal<PluginMap>>();
	let panel = plugins
		.read()
		.iter()
		.find_map(
			|(_, (plugin, state))| {
				if state.is_enabled { plugin.layout.clone() } else { None }
			},
		)
		.unwrap();
	// let config = use_context::<RunConfig>();

	rsx! {
		{RenderPanel(panel)}
	}
}

pub fn RenderPanel(panel: Panel) -> Element {
	match panel {
		Panel::Layout {
			id,
			layout_direction,
			drop_target,
			children,
			resizeable,
			size,
		} => {
			rsx! {
				PanelLayout {
					layout_direction: layout_direction,
					drop_target: drop_target,
					items: children,
					resizeable: resizeable,
				}
			}
		}
		Panel::WidgetGroup {
			id,
			display_mode,
			is_collapsed,
			draggable,
			layout_direction,
			tools,
		} => {
			rsx! {
				PanelWidgetGroup {
					display_mode: display_mode,
					is_collapsed: is_collapsed,
					draggable: draggable,
					layout_direction: layout_direction,
					widgets: tools,
				}
			}
		}
	}
}

#[component]
pub fn PanelLayout(layout_direction: LayoutDirection, drop_target: bool, items: Vec<Panel>, resizeable: ResizeAxis) -> Element {
	debug!("rendering PanelLayout component");
	let flex_class = match layout_direction {
		LayoutDirection::Right => "flex flex-row",
		LayoutDirection::Left => "flex flex-row-reverse",
		LayoutDirection::Down => "flex flex-col",
		LayoutDirection::Up => "flex flex-col-reverse",
	};
	rsx! {
		div { class: "{flex_class} flex-1 min-h-0 min-w-0",
			for item in items {
				{RenderPanel(item)}
			}
		}
	}
}

#[component]
pub fn PanelWidgetGroup(
	display_mode: DisplayMode,
	is_collapsed: Option<bool>,
	draggable: bool,
	layout_direction: LayoutDirection,
	widgets: Vec<Widget>,
) -> Element {
	debug!("rendering PanelWidgetGroup component");
	let flex_class = match layout_direction {
		LayoutDirection::Right => "flex flex-row",
		LayoutDirection::Left => "flex flex-row-reverse",
		LayoutDirection::Down => "flex flex-col",
		LayoutDirection::Up => "flex flex-col-reverse",
	};
	rsx! {
		div { class: "{flex_class} flex-1 min-h-0 min-w-0",
			for widget in widgets {
				{(widget.component)()}
			}
		}
	}
}
