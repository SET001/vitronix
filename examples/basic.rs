use vitronix::config::{Config, WindowConfig};
use vitronix::runner::run;

fn main() {
	run(Config {
		window: WindowConfig {
			title: "Vitronix basic example".to_string(),
			..Default::default()
		},
		..Default::default()
	});
}
