use vitronix::runner::{RunConfig, WindowConfig, run};

fn main() {
	run(RunConfig {
		window: WindowConfig::default().add_title_part("Basic example"),
		..Default::default()
	});
}
