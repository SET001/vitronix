use dioxus::prelude::*;
use vitronix::panel::Panel;
use vitronix::resource::{Resource, ResourceId, Resources};
use vitronix::runner::{RunConfig, WindowConfig, run};
use vitronix::view::View;
use vitronix::window::WindowWrapper;

fn main() {
	run(RunConfig {
		window: WindowConfig::default().add_title_part("Views example"),
		app: App,
		..Default::default()
	});
}

#[component]
pub fn App() -> Element {
	let mut resources: Signal<Resources> = use_context::<Signal<Resources>>();
	let panel = use_hook(|| {
		let tools = vec![
			View {
				title: "Description".into(),
				element: rsx! { Description {} },
			},
			View {
				title: "Another View".into(),
				element: rsx! {"This is another view."},
			},
		]
		.iter()
		.map(|view| resources.write().put(view.clone()))
		.collect::<Vec<_>>();
		resources.write().put_named(
			Panel {
				tools,
				..Default::default()
			},
			"tabs".into(),
		)
	});

	rsx! {
		WindowWrapper{
			content: RenderPanel(panel)
		}
	}
}

#[allow(non_snake_case)]
pub fn RenderPanel(panel_id: ResourceId<Panel>) -> Element {
	let mut resources: Signal<Resources> = use_context::<Signal<Resources>>();
	let tools: Vec<(ResourceId<View>, View)> = resources
		.read()
		.get(&panel_id)
		.map(|panel| {
			panel
				.tools
				.clone()
				.into_iter()
				.flat_map(|id| resources.read().get(&id).map(|view| (id, view.clone())))
				.collect()
		})
		.unwrap_or_default();

	let mut active_view = use_signal(|| tools.clone().first().cloned());
	rsx! {{
		active_view.read().cloned().map(|(id, view)| {
			let tabs = rsx! {
				for (key, tool) in tools.iter() {
					{
						let is_active = active_view.read().as_ref().map(|(id, _)| id) == Some(key);
						let onclick_key = key.clone();
						let remove_key = key.clone();
						let remove_panel_id = panel_id.clone();
						let fallback_view = tools.iter().find(|(id, _)| id != key).cloned();
						let onclick_tool = tool.clone();
						rsx! {
						div {
							class: "flex flex-row w-fit panel",
							class: if is_active { "tab-active" },
							onclick: move |_| active_view.set(Some((onclick_key.clone(), onclick_tool.clone()))),
							"{tool.title}"
							button {
								onclick: move |evt| {
									evt.stop_propagation();
									resources.write().remove(&remove_key);
									if let Some(panel) = resources.write().get_mut(&remove_panel_id) {
										panel.tools.retain(|id| id != &remove_key);
									}
									if active_view.read().as_ref().map(|(id, _)| id) == Some(&remove_key) {
										active_view.set(fallback_view.clone());
									}
								},
								"[X]"
							}
						}
					}}
				}
			};
			return rsx! {
				div{
					class: "flex flex-col flex-1 w-full min-h-0 gap-0",
					div {
						class: "flex flex-row gap-7 panel rounded-b-none",
						{tabs}
					}
					div {
						class: "flex flex-col flex-1 min-h-0 w-full panel rounded-t-none",
						{view.element}
					}
				}
			}
		})
	}}
}

#[component]
pub fn Description() -> Element {
	let mut resources: Signal<Resources> = use_context::<Signal<Resources>>();
	let panel_id = ResourceId::<Panel>::from_string("tabs".into());
	let panel_exists = resources.read().get(&panel_id).is_some();

	if !panel_exists {
		return rsx! { "Tabs panel not found" };
	}

	rsx! {
		div {
			class: "flex flex-row",
			"This is example of how you can use multiple Views in the app."
		}
		div {
			class: "flex flex-row",
			button { "Click me" }
		 }
		div {
			class: "flex flex-row",
			button {
				onclick: move |_| {
					info!("Opening another view");
					let id = resources.write().put(View {
						title: "Another View".into(),
						element: rsx! {"This is another view."},
					});
					if let Some(panel) = resources.write().get_mut(&panel_id) {
						panel.tools.push(id);
					}
				},
				"Open default view",
			}
		 }
	}
}
