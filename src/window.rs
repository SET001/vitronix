use crate::{ResizeHandles, config::WindowType, window_utils::use_drag_window};
use dioxus::prelude::*;

#[component]
pub fn Window(title: String, window_type: WindowType, children: Element) -> Element {
	debug!("rendering Window component for type: {:?}", window_type);
	let resizeable = match window_type {
		WindowType::Maximized => false,
		WindowType::Sized { resizable, .. } => resizable,
	};

	rsx! {
		div {
			class: "window",
			if resizeable {
				ResizeHandles {}
			}
			{children}
		}
	}
}

#[component]
pub fn WindowContent(children: Element) -> Element {
	rsx! {
		div { class: "window-content", {children} }
	}
}

#[derive(Clone, PartialEq, Props, Debug)]
pub struct TitleBarProp {
	title: String,
	icon: Option<Element>,
	#[props(default = true)]
	closeable: bool,
	#[props(default = true)]
	maximizable: bool,
	#[props(default = true)]
	minimizable: bool,
}

#[component]
pub fn TitleBar(props: TitleBarProp) -> Element {
	debug!("rendering TitleBar component with props: {:?}", props);

	let close_button = props.closeable.then(|| {
		rsx! {
			button {
				onmousedown: |e| e.stop_propagation(),
				onclick: move |_: Event<MouseData>| {
					// TODO: blitz has no use_window() close API yet
				},
				class: "title-bar-button",
				"X"
			}
		}
	});

	rsx! {
		div {
			class: "title-bar",
			onmousedown: use_drag_window(),
			div {
				class: "flex items-center gap-2 min-w-0",
				{props.icon}
				span { class: "truncate leading-none", {props.title} }
			}
			div { class: "flex items-center gap-1 ml-auto", {close_button} }
		}
	}
}
