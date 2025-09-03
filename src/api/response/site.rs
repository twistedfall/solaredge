use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;

use super::List;
use crate::api::enums::{
	BatteryState, EnergyUnit, EquipmentCommunicationMethod, GasEmissionUnit, Measurer, MeterForm, MeterType, PowerFlowElement,
	PowerFlowElementStatus, PowerUnit, SensorType, SiteStatus, TimeUnit,
};
use crate::api::{DateSerde, DateTimeSerde, DateTimeSerdeOpt};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
	pub country: String,
	pub city: String,
	pub address: String,
	pub address2: Option<String>,
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
	pub temperature_coef: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Uris {
	pub details: String,
	pub data_period: String,
	pub overview: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicSettings {
	pub name: Option<String>,
	pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
	/// the site ID
	pub id: u64,
	/// the site name
	pub name: String,
	/// the account this site belongs to
	pub account_id: u64,
	/// the site status
	pub status: SiteStatus,
	/// site peak power
	pub peak_power: f64,
	#[serde(with = "DateTimeSerdeOpt")]
	pub last_update_time: Option<NaiveDateTime>,
	pub currency: Option<String>,
	/// site installation date
	#[serde(with = "DateTimeSerde")]
	pub installation_date: NaiveDateTime,
	/// permission to operate date
	#[serde(with = "DateTimeSerdeOpt")]
	pub pto_date: Option<NaiveDateTime>,
	pub notes: Option<String>,
	/// site type
	#[serde(rename = "type")]
	pub site_type: String,
	/// includes country, state, city, address, secondary address, time zone and zip
	pub location: Location,
	pub primary_module: Module,
	/// number of open alerts in this site
	pub alert_quantity: Option<u32>,
	/// the highest alert severity in this site
	pub alert_severity: Option<String>,
	pub uris: Uris,
	/// includes if this site is public and its public name
	pub public_settings: PublicSettings,
}

#[derive(Debug, Deserialize)]
pub struct ListTop {
	pub sites: List<Details>,
}

#[derive(Debug, Deserialize)]
pub struct DetailsTop {
	pub details: Details,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPeriod {
	/// In case the site is not transmitting, the value is `None`
	#[serde(with = "DateTimeSerdeOpt")]
	pub start_date: Option<NaiveDateTime>,
	/// In case the site is not transmitting, the value is `None`
	#[serde(with = "DateTimeSerdeOpt")]
	pub end_date: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPeriodTop {
	pub data_period: DataPeriod,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteDataPeriod {
	pub site_id: u64,
	pub data_period: DataPeriod,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPeriodBulkTop {
	pub date_period_list: List<SiteDataPeriod>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateValue {
	/// The date is calculated based on the time zone of the site.
	#[serde(with = "DateTimeSerde")]
	pub date: NaiveDateTime,
	/// `None` means there is no data for that time.
	pub value: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Energy {
	pub time_unit: TimeUnit,
	/// Units of measurement (e.g. `Wh`)
	pub unit: EnergyUnit,
	pub values: Vec<DateValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyTop {
	pub energy: Energy,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyValues {
	pub values: Vec<DateValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteEnergyValues {
	pub site_id: u64,
	pub energy_values: EnergyValues,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyBulkList {
	pub time_unit: TimeUnit,
	pub unit: EnergyUnit,
	pub count: usize,
	pub site_energy_list: Vec<SiteEnergyValues>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyBulkTop {
	pub sites_energy: EnergyBulkList,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifetimeEnergy {
	#[serde(with = "DateSerde")]
	pub date: NaiveDate,
	pub energy: Option<f64>,
	pub unit: EnergyUnit,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeframeEnergy {
	pub energy: Option<f64>,
	pub unit: EnergyUnit,
	pub measured_by: Option<Measurer>,
	pub start_lifetime_energy: LifetimeEnergy,
	pub end_lifetime_energy: LifetimeEnergy,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeframeEnergyTop {
	#[serde(rename = "timeFrameEnergy")]
	pub timeframe_energy: TimeframeEnergy,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteTimeframeEnergy {
	pub site_id: u64,
	#[serde(rename = "timeFrameEnergy")]
	pub timeframe_energy: TimeframeEnergy,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeframeEnergyBulkTop {
	#[serde(rename = "timeFrameEnergyList")]
	pub timeframe_energy_list: List<SiteTimeframeEnergy>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Power {
	pub time_unit: TimeUnit,
	pub unit: PowerUnit,
	pub values: Vec<DateValue>,
}

#[derive(Debug, Deserialize)]
pub struct PowerTop {
	pub power: Power,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SitePowerEnergyValues {
	pub site_id: u64,
	pub power_data_value_series: EnergyValues,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerValueList {
	pub time_unit: TimeUnit,
	pub unit: PowerUnit,
	pub count: usize,
	pub site_energy_list: Vec<SitePowerEnergyValues>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerBulkTop {
	pub power_date_values_list: PowerValueList,
}

#[derive(Debug, Deserialize)]
pub struct LifetimeData {
	pub energy: f64,
	pub revenue: f64,
}

#[derive(Debug, Deserialize)]
pub struct EnergyData {
	pub energy: f64,
}

#[derive(Debug, Deserialize)]
pub struct PowerData {
	pub power: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overview {
	#[serde(with = "DateTimeSerde")]
	pub last_update_time: NaiveDateTime,
	#[serde(rename = "lifeTimeData")]
	pub lifetime_data: LifetimeData,
	pub last_year_data: EnergyData,
	pub last_month_data: EnergyData,
	pub last_day_data: EnergyData,
	pub current_power: PowerData,
	pub measured_by: Option<Measurer>,
}

#[derive(Debug, Deserialize)]
pub struct OverviewTop {
	pub overview: Overview,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteOverview {
	pub site_id: u64,
	pub site_overview: Overview,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverviewBulkTop {
	pub sites_overviews: List<SiteOverview>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterValues {
	/// The meter type
	#[serde(rename = "type")]
	pub meter_type: MeterType,
	pub values: Vec<DateValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerDetails {
	/// The time unit of the data
	pub time_unit: TimeUnit,
	/// Power measurement units (e.g. Watt)
	pub unit: PowerUnit,
	/// List of meters
	pub meters: Vec<MeterValues>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerDetailsTop {
	pub power_details: PowerDetails,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyDetails {
	/// the requested time unit
	pub time_unit: TimeUnit,
	/// The measurement units (e.g. Wh)
	pub unit: EnergyUnit,
	/// List of meters.
	pub meters: Vec<MeterValues>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyDetailsTop {
	pub energy_details: EnergyDetails,
}

#[derive(Debug, Deserialize)]
pub struct PowerConnection {
	/// The element providing power
	pub from: PowerFlowElement,
	/// The element consuming power
	pub to: PowerFlowElement,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerFlowEntry {
	/// The current status of the element
	pub status: PowerFlowElementStatus,
	/// The current power of the element. All numbers are positive; power direction is determined by the
	/// "connections" section above:
	/// * Check the "connection" section for the direction. From grid to load = import (purchase), from load to
	///   grid = export (feed-in).
	pub current_power: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoragePowerFlowEntry {
	/// The current status of the element
	pub status: PowerFlowElementStatus,
	/// The current power of the element. All numbers are positive; power direction is determined by the
	/// "connections" section above:
	/// * Check the "connection" section for the direction. From storage to load = discharge. From PV to storage or from
	///   load to storage = charge.
	pub current_power: Option<f64>,
	/// The accumulated state of energy (% of charge) for all batteries
	pub charge_level: u8,
	/// If the accumulated storage charge level drops below a configurable level (currently 10%), this flag is returned
	pub critical: bool,
	/// In Backup mode (GRID is Disabled), this property is returned to specify the time left before the storage energy
	/// runs out (estimated according to current load level).
	pub time_left: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CurrentPowerFlow {
	/// The measurement units (e.g. Watt)
	pub unit: PowerUnit,
	/// A table including all the relationships between the elements, and the power flow directions (producing element
	/// and consuming element)
	pub connections: Vec<PowerConnection>,
	/// always included in response
	#[serde(rename = "GRID")]
	pub grid: PowerFlowEntry,
	/// always included in response
	#[serde(rename = "LOAD")]
	pub load: PowerFlowEntry,
	/// included if the site has a PV array (measurement of PV produced power)
	#[serde(rename = "PV")]
	pub pv: Option<PowerFlowEntry>,
	/// included if the site has storage installed and enabled
	#[serde(rename = "STORAGE")]
	pub storage: Option<StoragePowerFlowEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentPowerFlowTop {
	pub site_current_power_flow: CurrentPowerFlow,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryTelemetry {
	/// Telemetry timestamp
	#[serde(rename = "timeStamp", with = "DateTimeSerde")]
	pub timestamp: NaiveDateTime,
	/// Positive power indicates the battery is charging, negative is discharging.
	pub power: i32,
	pub battery_state: BatteryState,
	/// The energy Charged from the battery in Wh, during battery lifetime.
	#[serde(rename = "lifeTimeEnergyCharged")]
	pub lifetime_energy_charged: u32,
	/// The energy discharged from the battery in Wh, during battery lifetime.
	#[serde(rename = "lifeTimeEnergyDischarged")]
	pub lifetime_energy_discharged: u32,
	/// The maximum energy (Wh) that can currently be stored in the battery. Note that the battery state of health (SoH)
	/// can be calculated from this value. SoH is defined as Full Pack Energy available today/Full Pack Energy available
	/// on day one. Full pack energy available on day one can be extracted from the battery nameplate value or battery
	/// model information. Both the battery name plate value and model number are provided by the storageData method.
	pub full_pack_energy_available: u32,
	/// Battery internal temperature in Celsius.
	pub internal_temp: u32,
	/// Amount of AC energy used to charge the battery from grid within a specified date range in Wh.
	#[serde(rename = "ACGridCharging")]
	pub ac_grid_charging: u32,
	/// The battery state of charge as percentage of the available capacity. Values are in the range of 0 to 100.
	pub state_of_charge: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageBattery {
	/// The battery serial number
	pub serial_number: String,
	/// The nameplate (nominal) capacity of the battery
	pub nameplate: u32,
	/// Battery model number
	pub model_number: String,
	/// The number of telemetries for this battery in the response
	pub telemetry_count: usize,
	/// A list of storage data telemetries.
	pub telemetries: Vec<BatteryTelemetry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageDataTop {
	pub storage_data: List<StorageBattery>,
}

#[derive(Debug, Deserialize)]
pub struct GasEmissionsSaved {
	pub units: GasEmissionUnit,
	pub co2: f64,
	pub so2: f64,
	pub nox: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvBenefits {
	/// quantity of CO2 emissions that would have been generated by an equivalent fossil fuel system
	pub gas_emission_saved: GasEmissionsSaved,
	/// equivalent planting of new trees for reducing CO2 levels
	pub trees_planted: f64,
	/// number of light bulbs that could have been powered by the site for a day
	pub light_bulbs: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvBenefitsTop {
	pub env_benefits: EnvBenefits,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inverter {
	/// the inverter name e.g. Inverter 1
	pub name: String,
	/// manufacturer name (SolarEdge)
	pub manufacturer: String,
	/// model name e.g. SE16K
	pub model: String,
	/// CPU Firmware version e.g. 2.52.311
	pub cpu_version: String,
	/// DSP 1 Firmware version
	pub dsp1_version: Option<String>,
	/// DSP 2 Firmware version
	pub dsp2_version: Option<String>,
	/// the communication interface used to connect to server. E.g. Ethernet.
	pub communication_method: EquipmentCommunicationMethod,
	/// the equipment serial number e.g. 7F123456-00
	#[serde(rename = "SN")]
	pub serial_number: String,
	/// number of optimizers connected to the inverter
	pub connected_optimizers: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meter {
	/// the inverter name e.g. "Feed In Meter"
	pub name: String,
	/// e.g. "WattNode"
	pub manufacturer: Option<String>,
	/// meter model number
	pub model: Option<String>,
	/// serial number (if applicable)
	#[serde(rename = "SN")]
	pub serial_number: Option<String>,
	#[serde(rename = "type")]
	pub meter_type: MeterType,
	/// FirmwareVersion (if applicable)
	pub firmware_version: Option<String>,
	/// Name of SolarEdge device the meter is connected to
	pub connected_to: Option<String>,
	/// serial number of the inverter / gateway the meter is connected to
	#[serde(rename = "connectedSolaredgeDeviceSN")]
	pub connected_solaredge_device_sn: Option<String>,
	pub form: MeterForm,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sensor {
	/// the S/N of the device it is connected to e.g. 12345678-00
	#[serde(rename = "connectedSolaredgeDeviceSN")]
	pub connected_solaredge_device_sn: String,
	/// e.g. "SensorDirectIrradiance"
	pub id: String,
	/// name of the device it is connected to e.g. "Gateway 1"
	pub connected_to: String,
	pub category: SensorType,
	/// e.g. "Plane of array irradiance"
	#[serde(rename = "type")]
	pub sensor_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gateway {
	/// the inverter name e.g. Inverter 1
	pub name: String,
	/// the equipment serial number e.g. 7F123456-00
	#[serde(rename = "SN")]
	pub serial_number: String,
	/// Firmware version
	pub firmware_version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battery {
	pub name: String,
	/// Serial Number
	#[serde(rename = "SN")]
	pub serial_number: String,
	/// the battery manufacturer name
	pub manufacturer: String,
	/// the battery model name
	pub model: String,
	/// the nameplate capacity of the battery as provided by the manufacturer
	pub nameplate_capacity: f64,
	/// Firmware version
	pub firmware_version: String,
	/// Name of SolarEdge device the battery is connected to
	pub connected_to: String,
	/// serial number of the inverter / gateway the battery is connected to
	pub connected_inverter_sn: String,
}

#[derive(Debug, Deserialize)]
pub struct Inventory {
	pub inverters: Vec<Inverter>,
	pub meters: Vec<Meter>,
	pub sensors: Vec<Sensor>,
	pub gateways: Vec<Gateway>,
	pub batteries: Vec<Battery>,
}

#[derive(Debug, Deserialize)]
pub struct InventoryTop {
	#[serde(rename = "Inventory")]
	pub inventory: Inventory,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterDetail {
	pub meter_serial_number: String,
	#[serde(rename = "connectedSolaredgeDeviceSN")]
	pub connected_solaredge_device_sn: String,
	pub model: String,
	pub meter_type: MeterType,
	pub values: Vec<DateValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meters {
	pub time_unit: TimeUnit,
	pub unit: EnergyUnit,
	pub meters: Vec<MeterDetail>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetersTop {
	pub meter_energy_details: Meters,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Measurements of the sensors are numerical values in metric system
pub struct SensorTelemetry {
	/// timestamp of the telemetries
	#[serde(with = "DateTimeSerde")]
	pub date: NaiveDateTime,
	pub ambient_temperature: Option<f64>,
	pub module_temperature: Option<f64>,
	pub wind_speed: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SensorData {
	/// name of the gateway the sensor is connected to
	pub connected_to: String,
	/// the number of telemetries
	pub count: usize,
	pub telemetries: Vec<SensorTelemetry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SensorDataTop {
	pub site_sensors: List<SensorData>,
}
