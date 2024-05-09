use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct SetupReturnFlags {
	pub is_re_run: bool,
	pub resume_auth: bool,
}
