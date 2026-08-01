use std::any::Any;

use crate::{
	app::App,
	config::{Config, WindowType},
};
use dioxus_native::WindowAttributes;

pub fn run(config: Config) {
	let mut attrs = WindowAttributes::default()
		.with_title(config.window.title.clone())
		.with_transparent(true)
		.with_decorations(config.window.decorations);

	attrs = match config.window.window_type {
		WindowType::Maximized => attrs.with_maximized(true),
		WindowType::Sized {
			width,
			height,
			resizable,
			..
		} => attrs
			.with_resizable(resizable)
			.with_inner_size(dioxus_native::LogicalSize::new(width as f64, height as f64)),
	};

	let native_cfg = dioxus_native::Config::new().with_window_attributes(attrs);

	dioxus_native::launch_cfg(
		App,
		vec![Box::new(move || -> Box<dyn Any> { Box::new(config.clone()) }) as Box<dyn Fn() -> Box<dyn Any> + Send + Sync>],
		vec![Box::new(native_cfg) as Box<dyn Any>],
	);
}
