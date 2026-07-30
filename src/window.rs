use crate::{
	ResizeHandles,
	config::WindowType,
	window_utils::{align_center, set_floatable},
};
use dioxus::{desktop::use_window, prelude::*};

#[component]
pub fn Window(title: String, window_type: WindowType, children: Element) -> Element {
	debug!("rendering Window component for type: {:?}", window_type);
	let resizeable = match window_type {
		WindowType::Maximized => false,
		WindowType::Sized { resizable, .. } => resizable,
	};

	let window = use_window();
	use_effect(move || {
		window.set_title(&title);
		window.set_visible(true);
		match window_type {
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
				set_floatable(&window.window);
				align_center(&window.window, width, height);
			}
		}
	});

	rsx! {
		if resizeable {
			ResizeHandles{}
		}
		{children}
	}
}
