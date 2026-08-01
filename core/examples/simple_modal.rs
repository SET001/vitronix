use core::{
	config::{Config, WindowConfig, WindowType},
	runner::run,
};

fn main() {
	run(Config {
		window: WindowConfig {
			title: "Vitronix splash screen example".to_string(),
			window_type: WindowType::Sized {
				width: 800.0,
				height: 600.0,
				position: None,
				resizable: true,
			},
			..Default::default()
		},
		..Default::default()
	});
}
