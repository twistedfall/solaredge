use chrono::NaiveDateTime;
use serde::Deserialize;

use super::List;
use crate::api::DateTimeSerde;
use crate::{InverterMode, OperationMode};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reporter {
	/// the inverter/SMI name
	pub name: String,
	/// the equipment manufacturer e.g. SolarEdge
	pub manufacturer: String,
	/// the inverter/SMI model e.g. SE16K
	pub model: String,
	/// the equipment short serial number
	pub serial_number: String,
	#[serde(rename = "kWpDC")]
	pub kw_p_dc: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct ListTop {
	pub reporters: List<Reporter>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LData {
	pub ac_current: f64,
	pub ac_voltage: f64,
	pub ac_frequency: f64,
	/// Supported starting communication board version 2.474, VA
	pub apparent_power: Option<f64>,
	/// Supported starting communication board version 2.474, W
	pub active_power: Option<f64>,
	/// Supported starting communication board version 2.474, VAR
	pub reactive_power: Option<f64>,
	pub cos_phi: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Telemetry {
	#[serde(with = "DateTimeSerde")]
	pub date: NaiveDateTime,
	pub total_active_power: f64,
	pub dc_voltage: Option<f64>,
	pub ground_fault_resistance: Option<f64>,
	pub power_limit: f64,
	/// Supported starting communication board version 2.474
	pub lifetime_energy: Option<f64>,
	pub total_energy: f64,
	/// Celsius
	pub temperature: f64,
	pub inverter_mode: InverterMode,
	pub operation_mode: OperationMode,
	#[serde(rename = "vL1ToN")]
	pub v_l1_to_n: Option<f64>,
	#[serde(rename = "vL2ToN")]
	pub v_l2_to_n: Option<f64>,
	#[serde(rename = "vL1To2")]
	pub v_l1_to_2: Option<f64>,
	#[serde(rename = "vL2To3")]
	pub v_l2_to_3: Option<f64>,
	#[serde(rename = "vL3To1")]
	pub v_l3_to_1: Option<f64>,
	#[serde(rename = "L1Data")]
	pub l1_data: LData,
	#[serde(rename = "L2Data")]
	pub l2_data: Option<LData>,
	#[serde(rename = "L3Data")]
	pub l3_data: Option<LData>,
}

#[derive(Debug, Deserialize)]
pub struct DataTop {
	pub data: List<Telemetry>,
}
