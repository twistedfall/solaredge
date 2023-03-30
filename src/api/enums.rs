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

impl Display for SortOrder {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		let s = match self {
			SortOrder::Ascending => "ASC",
			SortOrder::Descending => "DESC",
		};
		f.write_str(s)
	}
}

#[derive(Copy, Clone, Debug, Serialize)]
pub enum SiteSortBy {
	Name,
	Country,
	State,
	City,
	Address,
	Zip,
	Status,
	PeakPower,
	InstallationDate,
	Amount,
	MaxSeverity,
	CreationTime,
}

impl Display for SiteSortBy {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		let s = match self {
			SiteSortBy::Name => "Name",
			SiteSortBy::Country => "Country",
			SiteSortBy::State => "State",
			SiteSortBy::City => "City",
			SiteSortBy::Address => "Address",
			SiteSortBy::Zip => "Zip",
			SiteSortBy::Status => "Status",
			SiteSortBy::PeakPower => "PeakPower",
			SiteSortBy::InstallationDate => "InstallationDate",
			SiteSortBy::Amount => "Amount",
			SiteSortBy::MaxSeverity => "MaxSeverity",
			SiteSortBy::CreationTime => "CreationTime",
		};
		f.write_str(s)
	}
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum SiteStatus {
	Active,
	Pending,
	Disabled,
	All,
}

impl Display for SiteStatus {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		let s = match self {
			SiteStatus::Active => "Active",
			SiteStatus::Pending => "Pending",
			SiteStatus::Disabled => "Disabled",
			SiteStatus::All => "All",
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

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum MeterType {
	Production,
	Consumption,
	SelfConsumption,
	FeedIn,
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
pub enum InverterMode {
	#[serde(rename = "OFF")]
	Off,
	/// night mode
	#[serde(rename = "NIGHT")]
	Night,
	/// pre-production
	#[serde(rename = "WAKE_UP")]
	WakeUp,
	#[serde(rename = "PRODUCTION")]
	Production,
	/// Forced power reduction
	#[serde(rename = "PRODUCTION_LIMIT")]
	ProductionLimit,
	/// Shutdown procedure
	#[serde(rename = "SHUTDOWN")]
	Shutdown,
	/// error mode
	#[serde(rename = "ERROR")]
	Error,
	/// maintenance
	#[serde(rename = "SETUP")]
	Setup,
	/// standby mode lock
	#[serde(rename = "LOCKED_STDBY")]
	LockedStdby,
	/// firefighters lock mode
	#[serde(rename = "LOCKED_FIRE_FIGHTERS")]
	LockedFireFighters,
	/// forced shutdown from servers
	#[serde(rename = "LOCKED_FORCE_SHUTDOWN")]
	LockedForceShutdown,
	/// communication timeout
	#[serde(rename = "LOCKED_COMM_TIMEOUT")]
	LockedCommTimeout,
	/// inverter self-lock trip
	#[serde(rename = "LOCKED_INV_TRIP")]
	LockedInvTrip,
	/// inverter self-lock on arc detection
	#[serde(rename = "LOCKED_INV_ARC_DETECTED")]
	LockedInvArcDetected,
	/// inverter lock due to DG mode enable
	#[serde(rename = "LOCKED_DG")]
	LockedDg,
	#[serde(rename = "MPPT")]
	MaximumPowerPointTracking,
	#[serde(rename = "SLEEPING")]
	Sleeping,
}

#[derive(Copy, Clone, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum OperationMode {
	OnGrid = 0,
	OffGridWithPvOrBattery = 1,
	OffGridWithGenerator = 2,
}

#[derive(Copy, Clone, Debug, Serialize)]
pub enum SystemUnits {
	Metrics,
	Imperial,
}
