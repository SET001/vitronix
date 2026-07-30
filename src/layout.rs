use dioxus::{desktop::use_window, prelude::*};

use crate::{
	config::{Config, WindowType},
	window::align_center,
};

#[component]
pub fn Layout() -> Element {
	debug!("rendering Layout component");

	let config = use_context::<Config>();

	let window = use_window();
	use_effect(move || {
		window.set_title(&config.window.title);
		match config.window.window_type {
			WindowType::Maximized => {
				if std::env::var("I3SOCK").is_ok() {
					std::process::Command::new("i3-msg").arg("floating disable").spawn().ok();
				}
				window.set_maximized(true);
			}
			WindowType::Sized {
				width,
				height,
				position,
				resizable,
			} => {
				window.set_resizable(resizable);

				if let Some((x, y)) = position {
					window.set_outer_position(dioxus::desktop::LogicalPosition::new(x, y));
				}

				window.set_inner_size(dioxus::desktop::LogicalSize::new(width, height));
				align_center(&window.window, 200., 100.);
			}
		}
	});
	rsx! {
		div { "vitronix app" }
	}
}
