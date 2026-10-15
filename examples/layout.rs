use dioxus::prelude::*;
use vitronix::{
	layout::Layout,
	panel::{
		Branch, DisplayMode, LayoutDirection,
		Node::{self, Leaf},
		Panel, PanelSizing,
	},
	runner::{RunConfig, WindowConfig, run},
	window::WindowWrapper,
};

fn main() {
	run(RunConfig {
		window: WindowConfig::default().add_title_part("Layout example"),
		app: App,
		..Default::default()
	});
}

#[component]
pub fn App() -> Element {
	let layout = Node::Branch(Branch {
		children: vec![
			Node::Leaf(Panel {
				main_sizing: PanelSizing::Content,
				tools: vec![rsx! {"This is simple example of how you can use Layout which is a collection of pannels."}],
				..Default::default()
			}),
			Node::Branch(Branch {
				layout_direction: LayoutDirection::Left,
				main_sizing: PanelSizing::Fill,
				children: vec![
					Node::Leaf(Panel {
						tools: vec![rsx! {"left"}],
						..Default::default()
					}),
					Node::Leaf(Panel {
						tools: vec![rsx! {"middle"}],
						..Default::default()
					}),
					Node::Leaf(Panel {
						tools: vec![rsx! {"right"}],
						..Default::default()
					}),
				],
				..Default::default()
			}),
			Node::Leaf(Panel {
				main_sizing: PanelSizing::Content,
				tools: vec![rsx! {"status bar"}],
				..Default::default()
			}),
		],
		..Default::default()
	});

	rsx! {
		WindowWrapper{
			content: rsx! {
				Layout{layout},
			}
		}
	}
}
