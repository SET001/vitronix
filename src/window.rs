use crate::{
	ResizeHandles,
	config::WindowType,
	window_utils::{align_center, set_floatable, use_drag_window},
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
	let title_cloned = title.clone();
	use_effect(move || {
		window.set_title(&title_cloned);
		window.set_visible(true);
		match window_type {
			WindowType::Maximized => {
				window.set_resizable(false);
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

				window.set_inner_size(dioxus::desktop::LogicalSize::new(width, height));
				#[cfg(target_os = "linux")]
				set_floatable(&window.window);

				if let Some((x, y)) = position {
					window.set_outer_position(dioxus::desktop::LogicalPosition::new(x, y));
				} else {
					align_center(&window.window, width, height);
				}
			}
		}
	});

	rsx! {
		div {
			class: "text-on-surface bg-surface border-on-surface border-2 rounded-lg w-screen h-screen overflow-hidden",
			if resizeable {
				ResizeHandles{}
			}
			{children}

		}
	}
}

#[derive(Clone, PartialEq, Props, Debug)]
pub struct TitleBarProp {
	title: String,
	icon: Option<String>,
	#[props(default = true)]
	closeable: bool,
	#[props(default = true)]
	maximizable: bool,
	#[props(default = true)]
	minimizable: bool,
}

/**
 * TODO:
 * 	- maximize button
 * 	- minimize button
 */
#[component]
pub fn TitleBar(props: TitleBarProp) -> Element {
	debug!("rendering TitleBar component with props: {:?}", props);
	let window = use_window();
	let icon = props.icon.map(|icon| {
		rsx! {
			img { src: "{icon}" }
		}
	});

	let close_button = props.closeable.then(|| {
		rsx! {
			button {
				onmousedown: |e| e.stop_propagation(),
				onclick: move |_| {
					info!("Closing window");
					window.close();
				},
				class: "title-bar-button",
				"X"
			}
		}
	});

	rsx! {
		div {
			onmousedown: use_drag_window(),
			{icon}
			{props.title}
			{close_button}
		}
	}
}
