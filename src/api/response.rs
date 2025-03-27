use serde::Deserialize;

pub mod accounts;
pub mod equipment;
pub mod site;
pub mod version;

#[derive(Debug, Deserialize)]
pub struct List<T> {
	#[serde(alias = "total", alias = "batteryCount")]
	pub count: Option<usize>,
	#[serde(
		alias = "data",
		alias = "site",
		alias = "siteEnergyList",
		alias = "timeFrameEnergyList",
		alias = "telemetries",
		alias = "batteries"
	)]
	pub list: Vec<T>,
}
