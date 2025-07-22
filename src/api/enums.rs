use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

#[derive(Copy, Clone, Debug, Serialize)]
pub enum SortOrder {
	#[serde(rename = "ASC")]
	Ascending,
	#[serde(rename = "DESC")]
	Descending,
}

#[derive(Copy, Clone, Debug, Serialize)]
pub enum SiteSortBy {
	/// sort by site name
	Name,
	/// sort by site country
	Country,
	/// sort by site state
	State,
	/// sort by site city
	City,
	/// sort by site address
	Address,
	/// sort by site zip code
	Zip,
	/// sort by site status
	Status,
	/// sort by peak power
	PeakPower,
	/// sort by installation date
	InstallationDate,
	/// sort by amount of alerts
	Amount,
	/// sort by alert severity
	MaxSeverity,
	/// sort by site creation time
	CreationTime,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum FilterSiteStatus {
	Active,
	Pending,
	PendingCommunication,
	Disabled,
	All,
}

impl Display for FilterSiteStatus {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		let s = match self {
			FilterSiteStatus::Active => "Active",
			FilterSiteStatus::Pending => "Pending",
			FilterSiteStatus::PendingCommunication => "PendingCommunication",
			FilterSiteStatus::Disabled => "Disabled",
			FilterSiteStatus::All => "All",
		};
		f.write_str(s)
	}
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum TimeUnit {
	#[serde(rename = "QUARTER_OF_AN_HOUR")]
	QuarterOfAnHour,
	#[serde(rename = "HOUR")]
	Hour,
	#[serde(rename = "DAY")]
	Day,
	#[serde(rename = "WEEK")]
	Week,
	#[serde(rename = "MONTH")]
	Month,
	#[serde(rename = "YEAR")]
	Year,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum MeterType {
	/// AC production power meter / inverter production AC power (fallback)
	Production,
	/// Consumption meter
	Consumption,
	/// virtual self-consumption (calculated)
	SelfConsumption,
	/// Export to GRID meter
	FeedIn,
	/// Import power from GRID meter
	Purchased,
}

impl Display for MeterType {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		let s = match self {
			MeterType::Production => "Production",
			MeterType::Consumption => "Consumption",
			MeterType::SelfConsumption => "SelfConsumption",
			MeterType::FeedIn => "FeedIn",
			MeterType::Purchased => "Purchased",
		};
		f.write_str(s)
	}
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MeterForm {
	/// for a HW meter
	Physical,
	/// if calculated by arithmetic between other meters
	Virtual,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InverterMode {
	Off,
	/// night mode
	Sleeping,
	/// pre-production
	Starting,
	/// production (Maximum Power Point Tracking)
	Mppt,
	/// Forced power reduction
	Throttled,
	/// Shutdown procedure
	ShuttingDown,
	/// error mode
	Fault,
	/// maintenance
	Standby,
	/// standby mode lock
	LockedStdby,
	/// fire-fighters lock mode
	LockedFireFighters,
	/// forced shutdown from server
	LockedForceShutdown,
	/// communication timeout
	LockedCommTimeout,
	/// inverter self-lock trip
	LockedInvTrip,
	/// inverter self-lock on arc detection
	LockedInvArcDetected,
	/// inverter lock due to DG mode enable
	LockedDg,
	/// inverter lock due to phase imbalance (1ph, Australia only)
	LockedPhaseBalancer,
	/// inverter lock due to pre-commissioning
	LockedPreCommissioning,
	/// inverter lock due to an undisclosed internal reason
	LockedInternal,
}

#[derive(Copy, Clone, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum OperationMode {
	OnGrid = 0,
	/// Operating in off-grid mode using PV or battery
	OffGridWithPvOrBattery = 1,
	/// Operating in off-grid mode with generator (e.g. diesel) is present
	OffGridWithGenerator = 2,
}

#[derive(Copy, Clone, Debug, Serialize)]
pub enum SystemUnits {
	Metrics,
	Imperial,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EnergyUnit {
	/// watt-hour
	Wh,
	#[serde(untagged)]
	Other(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PowerUnit {
	/// watt
	W,
	/// kilowatt
	#[serde(rename = "kW")]
	Kw,
	#[serde(untagged)]
	Other(String),
}

#[derive(Clone, Debug, Deserialize)]
pub enum Measurer {
	#[serde(rename = "INVERTER")]
	Inverter,
	#[serde(untagged)]
	Other(String),
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum PowerFlowElement {
	#[serde(rename = "GRID")]
	Grid,
	Load,
	#[serde(rename = "PV")]
	Pv,
	Storage,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum PowerFlowElementStatus {
	Active,
	Idle,
	Inactive,
	Disabled,
}

#[derive(Copy, Clone, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum BatteryState {
	Invalid = 0,
	Standby = 1,
	ThermalManagement = 2,
	Enabled = 3,
	Fault = 4,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GasEmissionUnit {
	Kg,
	Lb,
}

#[derive(Clone, Debug, Deserialize)]
pub enum EquipmentCommunicationMethod {
	#[serde(rename = "ETHERNET")]
	Ethernet,
	#[serde(untagged)]
	Other(String),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SensorType {
	Irradiance,
	Temperature,
	#[serde(untagged)]
	Other(String),
}

#[derive(Clone, Debug, Deserialize)]
pub enum SensorMeasurement {
	SensorGlobalHorizontalIrradiance,
	SensorDiffusedIrradiance,
	SensorAmbientTemperature,
	#[serde(untagged)]
	Other(String),
}

#[derive(Copy, Clone, Debug, Serialize)]
pub enum AccountSortBy {
	/// sort by account name
	Name,
	/// sort by account country
	Country,
	/// sort by account city
	City,
	/// sort by account address
	Address,
	/// sort by account zip code
	Zip,
	/// sort by account fax number
	Fax,
	/// sort by account phone
	Phone,
	/// sort by account notes
	Notes,
}
