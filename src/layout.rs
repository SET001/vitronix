use dioxus::prelude::*;

use crate::config::Config;

#[component]
pub fn Layout() -> Element {
	debug!("rendering Layout component");

	let config = use_context::<Config>();

	rsx! {
		div { "vitronix app" }
	}
}
