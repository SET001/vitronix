use dioxus::prelude::*;
use vitronix::{
	container::{Container, ContainerParams, PanelParams},
	resource::Resources,
	runner::{RunConfig, WindowConfig, run},
	view::View,
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

pub fn App() -> Element {
	let mut resources = use_context::<Signal<Resources>>();

	let panels = use_hook(|| {
		let view1_id = resources.write().put_named(
			View {
				title: "Description".into(),
				closable: false,
				element: rsx! { "view 1"},
			},
			"exemple.views.view1".to_string(),
		);
		let view2_id = resources.write().put_named(
			View {
				title: "Description".into(),
				closable: false,
				element: rsx! { "view 2"},
			},
			"exemple.views.view2".to_string(),
		);
		let view3_id = resources.write().put_named(
			View {
				title: "Description".into(),
				closable: false,
				element: rsx! { "view 3"},
			},
			"exemple.views.view3".to_string(),
		);
		let view4_id = resources.write().put_named(
			View {
				title: "Description".into(),
				closable: false,
				element: rsx! { "view 4"},
			},
			"exemple.views.view4".to_string(),
		);

		let panel1 = resources.write().put_named(
			PanelParams {
				class: "panel-class".to_string(),
				is_collapsed: Some(false),
				views: vec![view1_id.clone(), view2_id],
			},
			"panel1".to_string(),
		);

		let panel2 = resources.write().put_named(
			PanelParams {
				class: "panel-class".to_string(),
				is_collapsed: Some(false),
				views: vec![view1_id, view3_id, view4_id],
			},
			"panel2".to_string(),
		);
		vec![panel1, panel2]
	});
	rsx! {
		div {
			class: "flex flex-row gap-4 p-4 w-full",
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
