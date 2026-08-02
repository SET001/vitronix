use vitronix::plugin::{Plugin, PluginDescriptor};
use vitronix::runner::{RunConfig, WindowConfig, run};

fn main() {
	run(RunConfig {
		window: WindowConfig::default().add_title_part("Basic example"),
		plugins: vec![PluginDescriptor {
			builder: plugin,
			required: false,
		}],
		..Default::default()
	});
}

pub fn plugin() -> Plugin {
	Plugin {
		name: "basic_plugin",
		description: Some("A basic plugin example"),
		..Default::default()
	}
}
