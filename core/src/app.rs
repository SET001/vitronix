#![allow(unpredictable_function_pointer_comparisons)]

use std::{ops::Deref, sync::OnceLock};

use crate::{
	layout::Layout,
	plugin::Plugin,
	runner::RunConfig,
	window::{TitleBar, Window, WindowContent},
};
use dioxus::{desktop::use_window, prelude::*};

static PLUGINS: OnceLock<Vec<Plugin>> = OnceLock::new();

fn load_plugins() -> Vec<Plugin> {
	let plugin_names = ["foo", "bar"];
	let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/debug");

	plugin_names
		.iter()
		.filter_map(|name| {
			let path = dir.join(libloading::library_filename(name));
			unsafe {
				match libloading::Library::new(&path) {
					Ok(lib) => {
						let plugin = {
							let create: libloading::Symbol<fn() -> Plugin> =
								lib.get(b"create_plugin").expect("create_plugin symbol not found");
							create()
						};
						std::mem::forget(lib);
						Some(plugin)
					}
					Err(e) => {
						eprintln!("[vitronix] failed to load {}: {e}", path.display());
						None
					}
				}
			}
		})
		.collect()
}

#[component]
fn PluginHost(entry: fn() -> Element) -> Element {
	entry()
}

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
	let plugins = PLUGINS.get_or_init(load_plugins);
	let config: RunConfig = use_context::<RunConfig>();
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
			title: &config.window.get_title(),
			window_type: config.window.window_type.clone(),
			TitleBar {
				icon: Some(rsx! {
					img { src: asset!("/assets/logo_sample3.png") }
				}),
				title: &config.window.get_title(),
			}
			WindowContent {
				// for plugin in plugins.iter() {
				// 	PluginHost { entry: plugin.entry }
				// }
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
