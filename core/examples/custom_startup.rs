/**
 * This example shows how to create a custom flow before the main app is rendered. In this example, we will show a splash screen before the main app is rendered.
 */
use dioxus::prelude::*;
use vitronix::runner::{RunConfig, WindowConfig};
use vitronix::{
	CustomStartupFinished,
	runner::run,
	window::{Window, WindowType},
};

#[cfg(target_os = "linux")]
use vitronix::window_utils::use_drag_window;
fn main() {
	run(RunConfig {
		window: WindowConfig::default().add_title_part("Custom startup example"),
		startup: Some(App),
		..Default::default()
	});
}

#[component]
pub fn App() -> Element {
	debug!("rendering App component");
	let done = use_context::<CustomStartupFinished>();

	let mut done_done = done.clone();
	rsx! {
		Window{
			title: "Vitronix custom startup example".to_string(),
			window_type: WindowType::Sized {
				width: 800.0,
				height: 600.0,
				position: None,
				resizable: true,
			},
			div {
				onmousedown: use_drag_window(),
				class: "w-screen h-screen p-4",
				p{
					"Example of how you can use a custom startup flow before the main app is rendered. In this example, we will show a splash screen before the main app is rendered."
				}
				button {
					onmousedown: |e| e.stop_propagation(),
					class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
					onclick: move |_| {
						//	signal that the startup is finished, and the main app can be rendered
						done_done.0.set(true);
					},
					"Finish Startup"
				}
			}
		}
	}
}
