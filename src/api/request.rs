use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;

use super::enums::{MeterType, SiteSortBy, SiteStatus, SortOrder, SystemUnits, TimeUnit};
use super::{serialize_comma_slice_opt, DateSerde, DateTimeSerde};
use crate::AccountSortBy;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SitesList<'r> {
	/// The maximum number of sites returned by this call.
	///
	/// The maximum number of sites that can be returned by this call
	/// is 100. If you have more than 100 sites, just request another 100 sites with startIndex=100.This will fetch sites
	/// 100-199.
	///
	/// Default value: `100`
	pub size: Option<u32>,
	/// The first site index to be returned in the results.
	///
	/// Default value: `0`
	pub start_index: Option<u32>,
	/// Search text for this site.
	///
	/// Searchable site properties:
	/// * Name
	/// * Notes
	/// * Address
	/// * City
	/// * Zip code
	/// * Full address
	/// * Country
	pub search_text: Option<&'r str>,
	/// A sorting option for this site list, based on one of its properties.
	pub sort_property: Option<SiteSortBy>,
	/// Sort order for the sort property.
	///
	/// Default value: `ASC`
	pub sort_order: Option<SortOrder>,
	/// Select the sites to be included in the list by their status.
	///
	/// Default value: `Active,Pending`
	#[serde(serialize_with = "serialize_comma_slice_opt")]
	pub status: Option<&'r [SiteStatus]>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteEnergy {
	/// The start date to return energy measurement
	#[serde(with = "DateSerde")]
	pub start_date: NaiveDate,
	/// The end date return energy measurement
	#[serde(with = "DateSerde")]
	pub end_date: NaiveDate,
	/// Aggregation granularity.
	///
	/// Default value: `DAY`
	pub time_unit: Option<TimeUnit>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteTotalEnergy {
	/// The start date to calculate energy generation
	#[serde(with = "DateSerde")]
	pub start_date: NaiveDate,
	/// The end date to calculate energy generation
	#[serde(with = "DateSerde")]
	pub end_date: NaiveDate,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeRange {
	/// The start (date + time) to get power measurements
	#[serde(with = "DateTimeSerde")]
	pub start_time: NaiveDateTime,
	/// The end (date + time) to get power measurements
	#[serde(with = "DateTimeSerde")]
	pub end_time: NaiveDateTime,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SitePowerDetails<'r> {
	/// The power measured start time
	#[serde(with = "DateTimeSerde")]
	pub start_time: NaiveDateTime,
	/// The power measured end time
	#[serde(with = "DateTimeSerde")]
	pub end_time: NaiveDateTime,
	/// Select specific meters only. If this value is omitted, all meter readings are returned.
	#[serde(serialize_with = "serialize_comma_slice_opt")]
	pub meters: Option<&'r [MeterType]>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetersDateTimeRange<'r> {
	/// The energy measured start time
	#[serde(with = "DateTimeSerde")]
	pub start_time: NaiveDateTime,
	/// The energy measured end time
	#[serde(with = "DateTimeSerde")]
	pub end_time: NaiveDateTime,
	/// Aggregation granularity.
	///
	/// Default value: `DAY`
	pub time_unit: Option<TimeUnit>,
	/// Select specific meters only. If this value is omitted, all meter readings are returned.
	#[serde(serialize_with = "serialize_comma_slice_opt")]
	pub meters: Option<&'r [MeterType]>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SensorsDateTimeRange {
	/// The start (date + time) to get sensor data
	#[serde(with = "DateTimeSerde")]
	pub start_date: NaiveDateTime,
	/// The end (date + time) to get sensor data
	#[serde(with = "DateTimeSerde")]
	pub end_date: NaiveDateTime,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteStorageData<'r> {
	/// Storage power measured start time
	#[serde(with = "DateTimeSerde")]
	pub start_time: NaiveDateTime,
	/// Storage power measured end time
	#[serde(with = "DateTimeSerde")]
	pub end_time: NaiveDateTime,
	/// Return data only for specific battery serial numbers. If omitted, the response includes all the batteries in
	/// the site.
	#[serde(serialize_with = "serialize_comma_slice_opt")]
	pub serials: Option<&'r [String]>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteImage {
	/// The maximum width to scale this image
	pub max_width: Option<u16>,
	/// The maximum height to scale this image
	pub max_height: Option<u16>,
	/// The image hash
	pub hash: Option<u32>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteEnvBenefits {
	/// The system units used when returning gas emission savings.
	///
	/// If systemUnits are not specified, the logged in user system units are used.
	pub system_units: Option<SystemUnits>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsList<'r> {
	/// The maximum number of accounts returned by this call. If you have more than 100 sites, just request another 100 sites with
	/// startIndex=100. This will fetch sites 100-199.
	///
	/// Default value: `100`
	pub size: Option<u8>,
	/// The first account index to be returned in the results
	///
	/// Default value: `0`
	pub start_index: Option<u8>,
	/// Search text for this account. Searchable properties:
	/// * Name – the account name
	/// * Notes
	/// * Email – contact person email
	/// * Country
	/// * State
	/// * City
	/// * Zip
	/// * Full address
	pub search_text: Option<&'r str>,
	/// A sorting option for this account list, based on one of its properties.
	pub sort_property: Option<AccountSortBy>,
	///  Sort order for the sort property.
	pub sort_order: Option<SortOrder>,
}
