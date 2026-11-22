use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use toml::Value;
use vitronix::{
	container::{Container, ContainerParams, PanelParams, WorkspaceDnd},
	resource::Resources,
	runner::{RunConfig, WindowConfig, run},
	view::{View, ViewsRegistry},
	window::WindowWrapper,
};
fn main() {
	run(RunConfig {
		window: WindowConfig::default().add_title_part("Containers example"),
		app: || {
			rsx! {
				WindowWrapper{
					content: App()
				}
			}
		},
		app_name: "containers_example".to_string(),
		..Default::default()
	});
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
struct ViewParams(u32);

pub fn App() -> Element {
	let mut resources = use_context::<Signal<Resources>>();
	let mut views_registry = use_context::<Signal<ViewsRegistry>>();
	let panels = use_hook(|| {
		let desc = views_registry
			.write()
			.register::<ViewParams>("containers_example.views.some_view", |params: ViewParams| View {
				element: rsx! { "view {params.0}" },
			});
		let panel1 = resources.write().put_named(
			PanelParams {
				class: "panel-class".to_string(),
				is_collapsed: Some(false),
				views: vec![
					desc.clone().with_params(Value::try_from(ViewParams(1)).unwrap()),
					desc.clone().with_params(Value::try_from(ViewParams(2)).unwrap()),
				],
			},
			"panel1".to_string(),
		);
		let panel2 = resources.write().put_named(
			PanelParams {
				class: "panel-class".to_string(),
				is_collapsed: Some(false),
				views: vec![
					desc.clone().with_params(Value::try_from(ViewParams(3)).unwrap()),
					desc.clone().with_params(Value::try_from(ViewParams(4)).unwrap()),
				],
			},
			"panel2".to_string(),
		);
		vec![panel1, panel2]
	});

	rsx! {
		WorkspaceDnd {
			div {
				class: "flex flex-row flex-1 min-h-0 min-w-0 gap-4 p-4 w-full",
				Container{
					id: "first",
					params: ContainerParams{
						class: "flex-col flex-1 min-w-0".to_string(),
						panels: vec![panels.get(0).unwrap().clone()],
						..Default::default()
					}
				}
				Container{
					id: "second",
					params: ContainerParams{
						class: "flex-col flex-1 min-w-0".to_string(),
						panels: vec![panels.get(1).unwrap().clone()],
						..Default::default()
					}
				}
			}
		}
	}
}
