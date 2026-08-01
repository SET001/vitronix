use std::ops::Deref;

use crate::{
	config::Config,
	layout::Layout,
	window::{TitleBar, Window, WindowContent},
};
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
	let ctx = use_window();
	use_effect(move || {
		let win = &ctx.window;

		// #[cfg(target_os = "linux")]
		// {
		// 	use crate::window::set_gtk_background_color;
		// 	let (r, g, b) = config.initial_theme.background_rgb();
		// 	set_gtk_background_color(r, g, b, win.clone());
		// }
	});
	let window = rsx! {
		Window{
			title: &config.window.title,
			window_type: config.window.window_type.clone(),
			TitleBar {
				icon: Some(rsx! {
					img { src: asset!("/assets/logo_sample3.png") }
				}),
				title: &config.window.title,
			}
			WindowContent {
				Layout {}
			}
		}
	};
	rsx! {
		document::Stylesheet { href: asset!("/assets/styles.css") }
		if let Some(Startup) = config.startup {
			if !*custom_startup_finished.read() {
				Startup {}
			} else {
				{window}
			}
		} else {
			{window}
		}
	}
}
