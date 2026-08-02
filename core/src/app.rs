#![allow(unpredictable_function_pointer_comparisons)]

use std::collections::HashSet;

use crate::state::PersistentState;
use crate::{
	layout::Layout,
	plugin::{Plugin, PluginState, load_dyn_plugins, merge_plugins},
	runner::RunConfig,
	state::{load_app_state, save_app_state},
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

	let plugins: Signal<PluginMap> = use_hook(|| {
		let app_state = load_app_state(&config.app_name);
		let required_names: HashSet<&str> = config
			.plugins
			.iter()
			.filter(|d| d.required)
			.map(|d| (d.builder)().name)
			.collect();
		let mut all_plugins: Vec<Plugin> = config.plugins.iter().map(|d| (d.builder)()).collect();
		if let Some(ref path) = config.plugins_path {
			all_plugins.extend(load_dyn_plugins(path));
		}
		let merged = merge_plugins(all_plugins, &required_names, &app_state);
		// persist initial state so newly discovered plugins are written immediately
		let initial_state = crate::state::AppPersistentState {
			plugins: merged.iter().map(|(name, (_, ps))| (name.clone(), ps.to_stored())).collect(),
		};
		save_app_state(&config.app_name, &initial_state);
		Signal::new(merged)
	});

	use_context_provider(|| plugins);

	// persist any newly discovered plugins and react to state changes
	use_effect(move || {
		let state = plugins.read();
		let app_state = crate::state::AppPersistentState {
			plugins: state.iter().map(|(name, (_, ps))| (name.clone(), ps.to_stored())).collect(),
		};
		save_app_state(&config.app_name, &app_state);
	});

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
