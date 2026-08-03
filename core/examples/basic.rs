use dioxus::prelude::*;

use vitronix::panel::{DisplayMode, LayoutDirection, Panel, ResizeAxis};
use vitronix::plugin::{Plugin, PluginDescriptor};
use vitronix::runner::{RunConfig, WindowConfig, run};
use vitronix::widget::Widget;

fn main() {
	run(RunConfig {
		window: WindowConfig::default().add_title_part("Basic example"),
		plugins: vec![PluginDescriptor {
			builder: plugin,
			required: false,
		}],
		..Default::default()
	});
}

pub fn plugin() -> Plugin {
	let layout = Panel::Layout {
		id: "example".into(),
		layout_direction: LayoutDirection::Down,
		drop_target: false,
		children: vec![
			Panel::WidgetGroup {
				id: "panel".into(),
				display_mode: DisplayMode::Stacked,
				is_collapsed: None,
				draggable: false,
				layout_direction: LayoutDirection::Down,
				tools: vec![Widget { component: NoticeBar }],
			},
			Panel::Layout {
				id: "panel".into(),
				layout_direction: LayoutDirection::Down,
				drop_target: false,
				children: vec![Panel::WidgetGroup {
					id: "panel".into(),
					display_mode: DisplayMode::Stacked,
					is_collapsed: None,
					draggable: false,
					layout_direction: LayoutDirection::Left,
					tools: vec![Widget { component: Filters }],
				}],
				resizeable: ResizeAxis::None,
				size: None,
			},
		],
		resizeable: ResizeAxis::None,
		size: None,
	};
	Plugin {
		name: "basic_plugin",
		description: Some("A basic plugin example"),
		layout: Some(layout),
		..Default::default()
	}
}

pub fn NoticeBar() -> Element {
	rsx! {
		div { "Notice bar" }
	}
}

pub fn Filters() -> Element {
	rsx! {
		div { "Filters" }
	}
}

pub fn ItemsList() -> Element {
	rsx! {
		div { "Items list" }
	}
}
