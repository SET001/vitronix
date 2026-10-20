#![allow(non_snake_case)]

use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use vitronix::{
	layout::Layout,
	panel::{Branch, LayoutDirection, Node, Panel, PanelSizing},
	runner::{RunConfig, WindowConfig, run},
	view::ViewManager,
	window::WindowWrapper,
};

fn main() {
	run(RunConfig {
		window: WindowConfig::default().add_title_part("Views example"),
		app: App,
		..Default::default()
	});
}

#[component]
pub fn App() -> Element {
	let mut view_manager = use_context::<ViewManager>();
	let mut default_view_id = use_signal(|| 0);

	use_hook(|| {
		default_view_id.set(default_view_id() + 1);
		view_manager.open(
			DefaultView(default_view_id()),
			format!("Default view #{}", default_view_id()).into(),
		);
	});

	let layout = Node::Branch(Branch {
		children: vec![
			Node::Leaf(Panel {
				main_sizing: PanelSizing::Content,
				tools: vec![
					rsx! {"This is example of how you can use multiple Views in the app."},
					rsx! {button {onclick: {
						let view_manager = view_manager.clone();

						move |_| {
							let mut view_manager = view_manager.clone();
							spawn(async move {
								if let Some(file) = AsyncFileDialog::new()
									.set_title("Open file")
									.pick_file()
									.await
								{
									let path = file.path().to_string_lossy().into_owned();
									view_manager.open(FileView(path), "File".into());
								}
							});
						}
					}, "Open file view"}},
					rsx! {button {onclick: {
						let mut view_manager = view_manager.clone();
						move |_| {
							default_view_id.set(default_view_id() + 1);
							view_manager.open(DefaultView(default_view_id()), format!("Default view #{}", default_view_id()).into());
						}
					}, "Open default view"}},
				],
				..Default::default()
			}),
			ViewSwitcher(),
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

#[component]
fn ViewSwitcher() -> Node {
	let view_manager = use_context::<ViewManager>();
	let views = view_manager.views.cloned();
	let mut active_view_id = use_signal(|| views.values().next().map(|v| v.id.clone()).unwrap());
	let tabs = rsx! {
		for (_, view) in views.clone() {
			div {
				class: "view-tab",
				class: if active_view_id() == view.id.clone() { "tab-active" },
				onclick: move |_| {
					active_view_id.set(view.id.clone());
				},
				{view.title.clone()}
				button {
					onclick:{
						let mut view_manager = view_manager.clone();
						move |event| {
							view_manager.close(view.id.clone());
							event.stop_propagation();
						}
					},
					"[ X ]"
				}
			}
		}
	};
	Node::Branch(Branch {
		children: vec![
			Node::Leaf(Panel {
				class: "gap-7".to_string(),
				main_sizing: PanelSizing::Content,
				layout_direction: LayoutDirection::Right,
				tools: vec![tabs],
				..Default::default()
			}),
			Node::Leaf(Panel {
				main_sizing: PanelSizing::Content,
				tools: vec![rsx! { {views.get(&active_view_id.read()).map(|v| v.element.clone()) }}],
				..Default::default()
			}),
		],
		..Default::default()
	})
}

fn FileView(file: String) -> Element {
	rsx! {
		"FileView for file: {file}"
	}
}

fn DefaultView(id: u32) -> Element {
	rsx! {
		"Default view #{id} content"
	}
}
