use dioxus::prelude::*;
use vitronix::plugin::Plugin;

#[global_allocator]
static ALLOC: std::alloc::System = std::alloc::System;

#[unsafe(no_mangle)]
pub fn create_plugin() -> Plugin {
	Plugin {
		entry: Some(entry),
		name: "foo",
		..Default::default()
	}
}

fn entry() -> Element {
	rsx! {
		div { "hello from foo plugin" }
	}
}
