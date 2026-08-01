use dioxus::prelude::*;

/// Returns a no-op drag handler (blitz branch: native decorations handle dragging).
pub fn use_drag_window() -> impl FnMut(Event<MouseData>) {
	move |_| {}
}

/// Returns a no-op resize handler.
pub fn use_resize_window(_direction: ResizeDirection) -> impl FnMut(Event<MouseData>) {
	move |_| {}
}

#[derive(Clone, Copy, Debug)]
pub enum ResizeDirection {
	North,
	South,
	East,
	West,
	NorthEast,
	NorthWest,
	SouthEast,
	SouthWest,
}

/// Transparent resize handles — no-op in blitz branch (native decorations).
#[component]
pub fn ResizeHandles() -> Element {
	rsx! {}
}
