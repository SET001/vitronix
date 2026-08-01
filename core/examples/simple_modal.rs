use vitronix::runner::{RunConfig, WindowConfig, run};
use vitronix::window::WindowType;

fn main() {
	run(RunConfig {
		window: WindowConfig {
			window_type: WindowType::Sized {
				width: 800.0,
				height: 600.0,
				position: None,
				resizable: true,
			},
			..Default::default()
		}
		.add_title_part("Simple modal example"),
		..Default::default()
	});
}
