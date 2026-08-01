use vitronix::{config::Config, runner::run};

fn main() {
	run(Config {
		window: vitronix::config::WindowConfig {
			title: "Vitronix splash screen example".to_string(),
			window_type: vitronix::config::WindowType::Sized {
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
