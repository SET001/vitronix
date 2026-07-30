use dioxus::desktop::use_window;
/**
 * This example shows how to create a custom flow before the main app is rendered. In this example, we will show a splash screen before the main app is rendered.
 */
use dioxus::prelude::*;
use vitronix::CustomStartupFinished;

#[cfg(target_os = "linux")]
use vitronix::window::set_floatable;
use vitronix::window::{align_center, use_drag_window};
fn main() {
	vitronix::runner::run(vitronix::config::Config {
		window: vitronix::config::WindowConfig {
			title: "Vitronix splash screen example".to_string(),
			window_type: vitronix::config::WindowType::Sized {
				width: 800.0,
				height: 600.0,
				position: None,
				resizable: true,
			},
			..Default::default()
		},
		startup: Some(App),
		..Default::default()
	});
}

#[component]
pub fn App() -> Element {
	debug!("rendering App component");
	let done = use_context::<CustomStartupFinished>();
	let window = use_window();
	use_effect(move || {
		#[cfg(target_os = "linux")]
		set_floatable(&window.window);
		align_center(&window.window, 200., 100.);
		window.window.set_title("asdsad");
	});

	let mut done_done = done.clone();
	rsx! {
		div {
			onmousedown: use_drag_window(),
			class: "w-screen h-screen",
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
