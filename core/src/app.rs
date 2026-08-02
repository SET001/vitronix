#![allow(unpredictable_function_pointer_comparisons)]

use crate::state::PersistentState;
use crate::{
	layout::Layout,
	plugin::{Plugin, PluginState, use_plugins},
	runner::RunConfig,
	state::{AppPersistentState, load_app_state, save_app_state},
	window::{TitleBar, Window, WindowContent},
};
use dioxus::{desktop::use_window, prelude::*};
use indexmap::IndexMap;

pub type PluginMap = IndexMap<String, (Plugin, PluginState)>;

#[component]
fn PluginHost(entry: fn() -> Element) -> Element {
	entry()
}

#[derive(Clone)]
pub struct CustomStartupFinished(pub Signal<bool>);

impl std::ops::Deref for CustomStartupFinished {
	type Target = Signal<bool>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[component]
pub fn App() -> Element {
	debug!("rendering App component");
	let config: RunConfig = use_context::<RunConfig>();
	let app_state = use_hook(|| load_app_state(&config.app_name));
	let plugins = use_plugins(&config, &app_state);
	use_context_provider(|| plugins);

	use_effect(move || {
		let state = plugins.read();
		save_app_state(
			&config.app_name,
			&AppPersistentState {
				plugins: state.iter().map(|(n, (_, ps))| (n.clone(), ps.to_stored())).collect(),
			},
		);
	});

	let custom_startup_finished = use_context_provider(|| CustomStartupFinished(Signal::new(config.startup.is_none())));
	let ctx = use_window();
	use_effect(move || {
		let _win = &ctx.window;

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
				for plugin in plugins.read().iter() {
					PluginHost { entry: plugin.1 .0.entry.unwrap() }
				}
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
