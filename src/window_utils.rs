use dioxus::{
	desktop::{
		tao::window::{ResizeDirection, Window},
		window as desktop_window,
	},
	prelude::*,
};
use std::sync::Arc;

/// Returns a closure that starts dragging the OS window when called.
/// Attach it to `onmousedown` of any element you want to use as a drag handle.
pub fn use_drag_window() -> impl FnMut(Event<MouseData>) {
	move |_| {
		desktop_window().window.drag_window().ok();
	}
}

/// Returns a closure that initiates an OS-level resize in the given direction.
/// Attach it to `onmousedown` of an edge/corner resize handle element.
pub fn use_resize_window(direction: ResizeDirection) -> impl FnMut(Event<MouseData>) {
	move |_| {
		desktop_window().window.drag_resize_window(direction).ok();
	}
}

/// Transparent edge/corner resize handles for decoration-less windows.
///
/// Renders eight invisible `position: fixed` divs that cover the window
/// borders and corners. Each one calls `drag_resize_window` on mouse-down
/// so the OS takes over the resize gesture — identical to what the native
/// title-bar frame does when decorations are enabled.
///
/// Add `<ResizeHandles />` anywhere inside your root component (it uses
/// `position: fixed` so layout position doesn't matter).
#[component]
pub fn ResizeHandles() -> Element {
	rsx! {
		style { "
			.vx-rh {{ position: fixed; z-index: 9999; user-select: none; }}
			.vx-rh-n  {{ top: 0;    left: 10px;  right: 10px;  height: 5px; cursor: n-resize;  }}
			.vx-rh-s  {{ bottom: 0; left: 10px;  right: 10px;  height: 5px; cursor: s-resize;  }}
			.vx-rh-e  {{ right: 0;  top: 10px;   bottom: 10px; width: 5px;  cursor: e-resize;  }}
			.vx-rh-w  {{ left: 0;   top: 10px;   bottom: 10px; width: 5px;  cursor: w-resize;  }}
			.vx-rh-ne {{ top: 0;    right: 0;    width: 10px;  height: 10px; cursor: ne-resize; }}
			.vx-rh-nw {{ top: 0;    left: 0;     width: 10px;  height: 10px; cursor: nw-resize; }}
			.vx-rh-se {{ bottom: 0; right: 0;    width: 10px;  height: 10px; cursor: se-resize; }}
			.vx-rh-sw {{ bottom: 0; left: 0;     width: 10px;  height: 10px; cursor: sw-resize; }}
		" }
		div { class: "vx-rh vx-rh-n",  onmousedown: use_resize_window(ResizeDirection::North) }
		div { class: "vx-rh vx-rh-s",  onmousedown: use_resize_window(ResizeDirection::South) }
		div { class: "vx-rh vx-rh-e",  onmousedown: use_resize_window(ResizeDirection::East) }
		div { class: "vx-rh vx-rh-w",  onmousedown: use_resize_window(ResizeDirection::West) }
		div { class: "vx-rh vx-rh-ne", onmousedown: use_resize_window(ResizeDirection::NorthEast) }
		div { class: "vx-rh vx-rh-nw", onmousedown: use_resize_window(ResizeDirection::NorthWest) }
		div { class: "vx-rh vx-rh-se", onmousedown: use_resize_window(ResizeDirection::SouthEast) }
		div { class: "vx-rh vx-rh-sw", onmousedown: use_resize_window(ResizeDirection::SouthWest) }
	}
}

/// Centers the window on the primary monitor given its logical size.
pub fn align_center(window: &Arc<Window>, width: f32, height: f32) {
	debug!("aligning window to center of primary monitor with size {}x{}", width, height);
	use dioxus::desktop::tao::dpi::LogicalPosition;
	if let Some(monitor) = window.current_monitor().or_else(|| window.primary_monitor()) {
		let scale = window.scale_factor();
		let m = monitor.size().to_logical::<f32>(scale);
		let m_pos = monitor.position().to_logical::<f32>(scale);
		window.set_outer_position(LogicalPosition::new(
			m_pos.x + (m.width - width) / 2.0,
			m_pos.y + (m.height - height) / 2.0,
		));
	}
}

/// Sets `_NET_WM_WINDOW_TYPE_SPLASH` on the GTK window before it is shown.
/// i3wm (and most tiling WMs) automatically float splash-type windows,
/// so no IPC commands or delays are needed.
/// Must be called before `set_visible(true)`.
#[cfg(target_os = "linux")]
pub fn set_floatable(window: &Arc<Window>) {
	use dioxus::desktop::tao::platform::unix::WindowExtUnix;
	use gtk::prelude::*;
	let gtk_win = window.gtk_window();
	gtk_win.set_type_hint(gtk::gdk::WindowTypeHint::Splashscreen);
}

// On Linux, set the GTK window background via CSS provider so that
// the X11 Expose event paints the correct color instead of white.
#[cfg(target_os = "linux")]
pub fn set_gtk_background_color(r: u8, g: u8, b: u8, window: Arc<Window>) {
	use dioxus::desktop::tao::platform::unix::WindowExtUnix;
	use gtk::prelude::*;
	let css = gtk::CssProvider::new();
	let _ = css.load_from_data(format!("window, widget {{ background: rgb({r},{g},{b}); }}").as_bytes());
	let gtk_win = window.gtk_window();
	gtk_win
		.style_context()
		.add_provider(&css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}
