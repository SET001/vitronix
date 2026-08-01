use core::config::{Config, WindowConfig};
use core::runner::run;

fn main() {
	run(Config {
		window: WindowConfig {
			title: "Vitronix basic example".to_string(),
			..Default::default()
		},
		..Default::default()
	});
}
