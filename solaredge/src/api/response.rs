use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;

use super::{
	DateSerde,
	DateTimeSerde,
	DateTimeSerdeOpt,
	enums::{InverterMode, MeterType, OperationMode, SiteStatus, TimeUnit},
};

#[derive(Debug, Deserialize)]
pub struct VersionSpec {
	pub release: String,
}

#[derive(Debug, Deserialize)]
pub struct VersionCurrentTop {
	pub version: VersionSpec,
}

#[derive(Debug, Deserialize)]
pub struct VersionSupportedTop {
	pub supported: Vec<VersionSpec>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
	pub country: String,
	pub city: String,
	pub address: String,
	pub address2: String,
	pub zip: String,
	pub time_zone: String,
	pub country_code: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Module {
	pub manufacturer_name: String,
	pub model_name: String,
	pub maximum_power: f64,
	pub temperature_coef: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct SiteUris {
	pub details: String,
	pub data_period: String,
	pub overview: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicSettings {
	pub name: Option<String>,
	pub is_public: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
	pub id: u64,
	pub name: String,
	pub account_id: u64,
	pub status: SiteStatus,
	pub peak_power: f64,
	#[serde(with = "DateTimeSerde")]
	pub last_update_time: NaiveDateTime,
	pub currency: Option<String>,
	#[serde(with = "DateTimeSerde")]
	pub installation_date: NaiveDateTime,
	#[serde(with = "DateTimeSerdeOpt")]
	pub pto_date: Option<NaiveDateTime>,
	pub notes: String,
	#[serde(rename = "type")]
	pub typ: String,
	pub location: Location,
	pub primary_module: Module,
	pub alert_quantity: Option<u32>,
	pub alert_severity: Option<String>,
	pub uris: SiteUris,
	pub public_settings: PublicSettings,
}

#[derive(Debug, Deserialize)]
pub struct SitesListSites {
	pub count: usize,
	pub site: Vec<Site>,
}

#[derive(Debug, Deserialize)]
pub struct SitesListTop {
	pub sites: SitesListSites,
}

#[derive(Debug, Deserialize)]
pub struct SiteDetailsTop {
	pub details: Site,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPeriod {
	#[serde(with = "DateTimeSerdeOpt")]
	pub start_date: Option<NaiveDateTime>,
	#[serde(with = "DateTimeSerdeOpt")]
	pub end_date: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteDataPeriodTop {
	pub data_period: DataPeriod,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteDateValue {
	#[serde(with = "DateTimeSerde")]
	pub date: NaiveDateTime,
	pub value: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteEnergy {
	pub time_unit: TimeUnit,
	pub unit: String,
	pub values: Vec<SiteDateValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteEnergyTop {
	pub energy: SiteEnergy,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteLifetimeEnergy {
	#[serde(with = "DateSerde")]
	pub date: NaiveDate,
	pub energy: f64,
	pub unit: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteTimeframeEnergy {
	pub energy: f64,
	pub unit: String,
	pub measured_by: String,
	pub start_lifetime_energy: SiteLifetimeEnergy,
	pub end_lifetime_energy: SiteLifetimeEnergy,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteTimeframeEnergyTop {
	#[serde(rename = "timeFrameEnergy")]
	pub timeframe_energy: SiteTimeframeEnergy,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SitePower {
	pub time_unit: TimeUnit,
	pub unit: String,
	pub values: Vec<SiteDateValue>,
}

#[derive(Debug, Deserialize)]
pub struct SitePowerTop {
	pub power: SitePower,
}

#[derive(Debug, Deserialize)]
pub struct SiteEnergyData {
	pub energy: f64,
	pub revenue: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct SitePowerData {
	pub power: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteOverview {
	#[serde(with = "DateTimeSerde")]
	pub last_update_time: NaiveDateTime,
	#[serde(rename = "lifeTimeData")]
	pub lifetime_data: SiteEnergyData,
	pub last_year_data: SiteEnergyData,
	pub last_month_data: SiteEnergyData,
	pub last_day_data: SiteEnergyData,
	pub current_power: SitePowerData,
	pub measured_by: String,
}

#[derive(Debug, Deserialize)]
pub struct SiteOverviewTop {
	pub overview: SiteOverview,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMeterValue {
	#[serde(rename = "type")]
	pub typ: String,
	pub values: Vec<SiteDateValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMetersDetails {
	pub time_unit: TimeUnit,
	pub unit: String,
	pub meters: Vec<SiteMeterValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SitePowerDetailsTop {
	pub power_details: SiteMetersDetails,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteEnergyDetailsTop {
	pub energy_details: SiteMetersDetails,
}

#[derive(Debug, Deserialize)]
pub struct PowerConnection {
	pub from: String,
	pub to: String,
}

#[derive(Debug, Deserialize)]
pub struct SiteCurrentPowerFlow {
	pub unit: Option<String>,
	pub connections: Option<Vec<PowerConnection>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteCurrentPowerFlowTop {
	pub site_current_power_flow: SiteCurrentPowerFlow,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryTelemetry {
	#[serde(rename = "timeStamp", with = "DateTimeSerde")]
	pub timestamp: NaiveDateTime,
	pub power: u32,
	pub battery_state: u32,
	#[serde(rename = "lifeTimeEnergyCharged")]
	pub lifetime_energy_charged: u32,
	#[serde(rename = "lifeTimeEnergyDischarged")]
	pub lifetime_energy_discharged: u32,
	pub full_pack_energy_available: u32,
	pub internal_temp: u32,
	#[serde(rename = "ACGridCharging")]
	pub ac_grid_charging: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageBattery {
	pub nameplate: String,
	pub serial_number: String,
	pub model_number: String,
	pub telemetry_count: usize,
	pub telemetries: Vec<BatteryTelemetry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteStorageData {
	pub battery_count: usize,
	pub batteries: Vec<StorageBattery>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteStorageDataTop {
	pub storage_data: SiteStorageData,
}

#[derive(Debug, Deserialize)]
pub struct GasEmissionsSaved {
	pub units: String,
	pub co2: f64,
	pub so2: f64,
	pub nox: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteEnvBenefits {
	pub gas_emission_saved: GasEmissionsSaved,
	pub trees_planted: f64,
	pub light_bulbs: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteEnvBenefitsTop {
	pub env_benefits: SiteEnvBenefits,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meter {
	pub name: String,
	pub manufacturer: String,
	pub model: String,
	pub firmware_version: String,
	#[serde(rename = "connectedSolaredgeDeviceSN")]
	pub connected_solaredge_device_sn: String,
	#[serde(rename = "type")]
	pub typ: String,
	pub form: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sensor {
	#[serde(rename = "connectedSolaredgeDeviceSN")]
	pub connected_solaredge_device_sn: String,
	pub id: String,
	pub connected_to: String,
	pub category: String,
	#[serde(rename = "type")]
	pub typ: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gateway {
	pub name: String,
	pub firmware_version: String,
	#[serde(rename = "SN")]
	pub sn: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battery {
	pub name: String,
	pub manufacturer: String,
	pub model: String,
	pub firmware_version: String,
	pub connected_inverter_sn: String,
	pub nameplate_capacity: f64,
	#[serde(rename = "SN")]
	pub sn: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inverter {
	pub name: String,
	pub manufacturer: String,
	pub model: String,
	pub communication_method: String,
	#[serde(rename = "SN")]
	pub sn: String,
	pub connected_optimizers: u32,
}

#[derive(Debug, Deserialize)]
pub struct SiteInventory {
	pub meters: Vec<Meter>,
	pub sensors: Vec<Sensor>,
	pub gateways: Vec<Gateway>,
	pub batteries: Vec<Battery>,
	pub inverters: Vec<Inverter>,
}

#[derive(Debug, Deserialize)]
pub struct SiteInventoryTop {
	#[serde(rename = "Inventory")]
	pub inventory: SiteInventory,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMeterValueExt {
	pub meter_serial_number: String,
	#[serde(rename = "connectedSolaredgeDeviceSN")]
	pub connected_solaredge_device_sn: String,
	pub model: String,
	pub meter_type: MeterType,
	pub values: Vec<SiteDateValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMeters {
	pub time_unit: TimeUnit,
	pub unit: String,
	pub meters: Vec<SiteMeterValueExt>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMetersTop {
	pub meter_energy_details: SiteMeters,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Equipment {
	pub name: String,
	pub manufacturer: String,
	pub model: String,
	pub serial_number: String,
	#[serde(rename = "kWpDC")]
	pub kw_p_dc: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct EquipmentReporters {
	pub count: usize,
	pub list: Vec<Equipment>,
}

#[derive(Debug, Deserialize)]
pub struct EquipmentListTop {
	pub reporters: EquipmentReporters,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LData {
	pub ac_current: f64,
	pub ac_voltage: f64,
	pub ac_frequency: f64,
	/// VA
	pub apparent_power: f64,
	/// VA
	pub active_power: f64,
	/// VAR
	pub reactive_power: f64,
	pub cos_phi: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentTelemetry {
	#[serde(with = "DateTimeSerde")]
	pub date: NaiveDateTime,
	pub total_active_power: f64,
	pub dc_voltage: Option<f64>,
	pub ground_fault_resistance: Option<f64>,
	pub power_limit: f64,
	pub total_energy: f64,
	/// Celsius
	pub temperature: f64,
	pub inverter_mode: InverterMode,
	pub operation_mode: OperationMode,
	#[serde(rename = "L1Data")]
	pub l1_data: LData,
	#[serde(rename = "vL1To2")]
	pub v_l1_to_2: Option<f64>,
	#[serde(rename = "vL2To3")]
	pub v_l2_to_3: Option<f64>,
	#[serde(rename = "vL3To1")]
	pub v_l3_to_1: Option<f64>,
	#[serde(rename = "L2Data")]
	pub l2_data: Option<LData>,
	#[serde(rename = "L3Data")]
	pub l3_data: Option<LData>,
}

#[derive(Debug, Deserialize)]
pub struct EquipmentData {
	pub count: usize,
	pub telemetries: Vec<EquipmentTelemetry>,
}

#[derive(Debug, Deserialize)]
pub struct EquipmentDataTop {
	pub data: EquipmentData,
}
