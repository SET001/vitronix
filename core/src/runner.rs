use crate::{
	FRAMEWORK_NAME, WINDOW_BACKGROUND_COLOR, WINDOW_TITLE_PARTS_SEPARATOR, app::App, plugin::PluginBuilder, theme::Theme,
	window::WindowType,
};
use dioxus::core::Element;

#[derive(Clone)]
pub struct WindowConfig {
	pub title_parts: Vec<String>,
	pub decorations: bool,
	pub window_type: WindowType,
}

impl Default for WindowConfig {
	fn default() -> Self {
		let mut title = FRAMEWORK_NAME.to_string();
		title.get_mut(0..1).map(|c| c.make_ascii_uppercase());
		Self {
			title_parts: vec![title],
			decorations: false,
			window_type: WindowType::Maximized,
		}
	}
}

impl WindowConfig {
	pub fn get_title(&self) -> String {
		self.title_parts.join(WINDOW_TITLE_PARTS_SEPARATOR)
	}

	pub fn add_title_part(&mut self, part: &str) -> Self {
		self.title_parts.insert(0, part.to_string());
		self.clone()
	}
}
#[derive(Default, Clone)]
pub struct RunConfig {
	pub window: WindowConfig,
	pub startup: Option<fn() -> Element>, //	for custom startup flow
	pub initial_theme: Option<Theme>,
	pub plugins: Vec<PluginBuilder>,
}

pub fn run(config: RunConfig) {
	let window = dioxus::desktop::WindowBuilder::new()
		.with_visible(false)
		.with_transparent(true)
		.with_background_color(WINDOW_BACKGROUND_COLOR)
		.with_decorations(false)
		.with_title(&config.window.get_title());

	let window_cfg = dioxus::desktop::Config::new().with_menu(None).with_window(window);
	dioxus::LaunchBuilder::desktop()
		.with_cfg(window_cfg)
		.with_context(config)
		.launch(App);
}
