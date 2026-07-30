use std::ops::Deref;

use crate::{config::Config, layout::Layout, window::Window};
use dioxus::{desktop::use_window, prelude::*};

#[derive(Clone)]
pub struct CustomStartupFinished(pub Signal<bool>);

impl Deref for CustomStartupFinished {
	type Target = Signal<bool>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[component]
pub fn App() -> Element {
	debug!("rendering App component");
	let config: Config = use_context::<Config>();
	let custom_startup_finished = use_context_provider(|| CustomStartupFinished(Signal::new(config.startup.is_none())));
	use_effect(move || {
		let ctx = use_window();
		let win = &ctx.window;

		// #[cfg(target_os = "linux")]
		// {
		// 	use crate::window::set_gtk_background_color;
		// 	let (r, g, b) = config.initial_theme.background_rgb();
		// 	set_gtk_background_color(r, g, b, win.clone());
		// }
	});
	rsx! {
		// style { {include_str!("../../public/main.css")} }
		if let Some(Startup) = config.startup {
			if !*custom_startup_finished.read() {
				Startup {}
			} else {
				Window{
					title: config.window.title.clone(),
					window_type: config.window.window_type.clone(),
					Layout {}
				}
			}
		} else {
			Window{
				title: config.window.title.clone(),
				window_type: config.window.window_type.clone(),
				Layout {}
			}
		}
	}
}
