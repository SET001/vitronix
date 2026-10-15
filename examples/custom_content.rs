use dioxus::prelude::*;
use vitronix::{
	runner::{RunConfig, WindowConfig, run},
	window::WindowWrapper,
};
fn main() {
	run(RunConfig {
		window: WindowConfig::default().add_title_part("Basic example"),
		app: App,
		..Default::default()
	});
}

#[component]
pub fn App() -> Element {
	rsx! {
		WindowWrapper{
			content: rsx! {
				div {
					"This is simple example of how you can use any custom content within a window."
				}
			}
		}
	}
}
