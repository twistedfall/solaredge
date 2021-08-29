use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;

use super::{
	DateSerde,
	DateTimeSerde,
	enums::{MeterType, SiteSortBy, SiteStatus, SortOrder, TimeUnit},
	serialize_comma_slice_opt,
};

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SitesList<'r> {
	pub size: Option<u32>,
	pub start_index: Option<u32>,
	pub search_text: Option<&'r str>,
	pub sort_property: Option<SiteSortBy>,
	pub sort_order: Option<SortOrder>,
	#[serde(serialize_with = "serialize_comma_slice_opt")]
	pub status: Option<&'r [SiteStatus]>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteEnergy {
	#[serde(with = "DateSerde")]
	pub start_date: NaiveDate,
	#[serde(with = "DateSerde")]
	pub end_date: NaiveDate,
	pub time_unit: Option<TimeUnit>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteTotalEnergy {
	#[serde(with = "DateSerde")]
	pub start_date: NaiveDate,
	#[serde(with = "DateSerde")]
	pub end_date: NaiveDate,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeRange {
	#[serde(with = "DateTimeSerde")]
	pub start_time: NaiveDateTime,
	#[serde(with = "DateTimeSerde")]
	pub end_time: NaiveDateTime,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SitePowerDetails<'r> {
	#[serde(with = "DateTimeSerde")]
	pub start_time: NaiveDateTime,
	#[serde(with = "DateTimeSerde")]
	pub end_time: NaiveDateTime,
	#[serde(serialize_with = "serialize_comma_slice_opt")]
	pub meters: Option<&'r [MeterType]>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteEnergyDetails<'r> {
	#[serde(with = "DateTimeSerde")]
	pub start_time: NaiveDateTime,
	#[serde(with = "DateTimeSerde")]
	pub end_time: NaiveDateTime,
	pub time_unit: Option<TimeUnit>,
	#[serde(serialize_with = "serialize_comma_slice_opt")]
	pub meters: Option<&'r [MeterType]>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteStorageData<'r> {
	#[serde(with = "DateTimeSerde")]
	pub start_time: NaiveDateTime,
	#[serde(with = "DateTimeSerde")]
	pub end_time: NaiveDateTime,
	#[serde(serialize_with = "serialize_comma_slice_opt")]
	pub serials: Option<&'r [String]>,
}
