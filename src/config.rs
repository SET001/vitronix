use dioxus::core::Element;

use crate::theme::Theme;

#[derive(Debug, Clone, PartialEq)]
pub enum WindowType {
	Maximized,
	Sized {
		width: f32,
		height: f32,
		position: Option<(f32, f32)>,
		resizable: bool,
	},
}
#[derive(Clone, PartialEq)]
pub struct WindowConfig {
	pub title: String,
	pub decorations: bool,
	pub window_type: WindowType,
}

impl Default for WindowConfig {
	fn default() -> Self {
		Self {
			title: "Vitronix App".to_string(),
			decorations: false,
			window_type: WindowType::Maximized,
		}
	}
}

#[derive(Default, Clone, PartialEq)]
pub struct Config {
	pub window: WindowConfig,
	pub startup: Option<fn() -> Element>, //	for custom startup flow
	pub initial_theme: Option<Theme>,
}
