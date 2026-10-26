use dioxus::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vitronix::panel::{Panel, RenderPanel};
use vitronix::resource::{ResourceId, Resources};
use vitronix::runner::{RunConfig, WindowConfig, run};
use vitronix::view::{View, persistent_in_scope};
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
		let editor_view_id = ResourceId::from_string(Uuid::new_v4().to_string());
		let editor_params = persistent_in_scope(format!("editor:{}", editor_view_id.id), ScopeId::APP, EditorParams::random);
		resources.write().put_named(
			View {
				title: "Editor View".into(),
				element: rsx! { Editor { params: editor_params } },
			},
			editor_view_id.id.clone(),
		);

		let desc_view_id = resources.write().put(View {
			title: "Description".into(),
			element: rsx! { Description {} },
		});

		resources.write().put_named(
			Panel {
				tools: vec![editor_view_id, desc_view_id],
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
					let editor_view_id = ResourceId::from_string(Uuid::new_v4().to_string());
					let editor_params =
						persistent_in_scope(format!("editor:{}", editor_view_id.id), ScopeId::APP, EditorParams::random);

					resources.write().put_named(
						View {
							title: "Editor View".into(),
							element: rsx! { Editor { params: editor_params } },
						},
						editor_view_id.id.clone(),
					);
					if let Some(panel) = resources.write().get_mut(&panel_id) {
						panel.tools.push(editor_view_id);
					}
				},
				"Open editor view",
			}
		 }
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum City {
	NewYork,
	London,
	Paris,
	Tokyo,
}

impl City {
	const ALL: [City; 4] = [City::NewYork, City::London, City::Paris, City::Tokyo];

	fn label(&self) -> &'static str {
		match self {
			City::NewYork => "New York",
			City::London => "London",
			City::Paris => "Paris",
			City::Tokyo => "Tokyo",
		}
	}

	fn from_label(label: &str) -> Option<City> {
		City::ALL.into_iter().find(|city| city.label() == label)
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Foo {
	name: String,
	age: u32,
	size: f32,
	city: City,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditorParams {
	items: Vec<Foo>,
}

impl EditorParams {
	pub fn random() -> Self {
		const NAMES: [&str; 6] = ["Alice", "Bob", "Carol", "Dave", "Eve", "Frank"];
		let mut rng = rand::rng();
		let count = rng.random_range(3..=8);
		Self {
			items: (0..count)
				.map(|_| Foo {
					name: NAMES[rng.random_range(0..NAMES.len())].to_string(),
					age: rng.random_range(18..=90),
					size: rng.random_range(0.5..=2.5),
					city: City::ALL[rng.random_range(0..City::ALL.len())],
				})
				.collect(),
		}
	}
}

#[component]
pub fn Editor(params: Signal<EditorParams>) -> Element {
	let mut selected = use_signal(|| None::<usize>);
	let selected_item = selected().and_then(|i| params().items.get(i).cloned());

	rsx! {
		div {
			class: "flex flex-row flex-1 w-full h-full gap-7",
			div {
				class: "flex flex-col panel w-64 shrink-0",
				ItemsList{
					items: params().items,
					selected: selected(),
					on_select: move |i| selected.set(Some(i)),
				}
			}
			div {
				class: "flex flex-col panel w-128 shrink-0",
				FooDetails{
					item: selected_item,
					on_change: move |updated: Foo| {
						if let Some(i) = selected() {
							if let Some(slot) = params.write().items.get_mut(i) {
								*slot = updated;
							}
						}
					},
				}
			}
			div {
				class: "flex flex-col panel flex-1",
				"some content goes here"
			}
		}
		div{
			class: "flex flex-row panel",
			"status bar."
		}
	}
}

#[component]
pub fn ItemsList(items: Vec<Foo>, selected: Option<usize>, on_select: EventHandler<usize>) -> Element {
	rsx! {
		div {
			class: "flex flex-col flex-1 w-full h-full",
			for (i, item) in items.iter().enumerate() {
				div {
					key: "{i}",
					class: "cursor-pointer",
					class: if selected == Some(i) { "tab-active" },
					onclick: move |_| on_select.call(i),
					"{item.name}"
				}
			}
		}
	}
}

#[component]
pub fn FooDetails(item: Option<Foo>, on_change: EventHandler<Foo>) -> Element {
	rsx! {
		div {
			class: "flex flex-col flex-1 w-full h-full gap-5",
			if let Some(item) = item {
				label {
					class: "flex flex-col",
					"Name"
					input {
						r#type: "text",
						value: "{item.name}",
						oninput: {
							let item = item.clone();
							move |evt| {
								let mut updated = item.clone();
								updated.name = evt.value();
								on_change.call(updated);
							}
						},
					}
				}
				label {
					class: "flex flex-col",
					"Age"
					input {
						r#type: "number",
						min: "0",
						value: "{item.age}",
						oninput: {
							let item = item.clone();
							move |evt| {
								if let Ok(age) = evt.value().parse::<u32>() {
									let mut updated = item.clone();
									updated.age = age;
									on_change.call(updated);
								}
							}
						},
					}
				}
				label {
					class: "flex flex-col",
					"Size"
					input {
						r#type: "number",
						step: "0.1",
						value: "{item.size}",
						oninput: {
							let item = item.clone();
							move |evt| {
								if let Ok(size) = evt.value().parse::<f32>() {
									let mut updated = item.clone();
									updated.size = size;
									on_change.call(updated);
								}
							}
						},
					}
				}
				label {
					class: "flex flex-col",
					"City"
					select {
						value: "{item.city.label()}",
						onchange: move |evt| {
							if let Some(city) = City::from_label(&evt.value()) {
								let mut updated = item.clone();
								updated.city = city;
								on_change.call(updated);
							}
						},
						for city in City::ALL {
							option { key: "{city.label()}", value: "{city.label()}", "{city.label()}" }
						}
					}
				}
			} else {
				div { "Select an item from the list." }
			}
		}
	}
}
