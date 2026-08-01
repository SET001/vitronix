use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
	debug!("rendering Layout component");

	// let config = use_context::<RunConfig>();

	rsx! {
		div { "vitronix app" }
	}
}
