use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Spec {
	pub release: String,
}

#[derive(Debug, Deserialize)]
pub struct CurrentTop {
	pub version: Spec,
}

#[derive(Debug, Deserialize)]
pub struct SupportedTop {
	pub supported: Vec<Spec>,
}
