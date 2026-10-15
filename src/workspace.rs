use crate::panel::Panel;

#[derive(Clone)]
pub struct Workspace {
	pub name: String,
	pub layout: Option<Panel>,
}
