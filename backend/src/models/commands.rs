use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct SetupReturnFlags {
	pub start_auth: bool,
	pub re_run: bool,
}
