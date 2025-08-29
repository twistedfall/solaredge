use std::fmt;
use std::fmt::Write;

use http_adapter::http::header::CONTENT_TYPE;
use http_adapter::{HttpClientAdapter, Request, Response};
use log::trace;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use url::Url;

use crate::Error;
use crate::api::request;
use crate::response::{accounts, equipment, site, version};

/// Client for accessing SolarEdge API
///
/// To be able to use it, you'll need to request the API key from the Admin panel of your SolarEdge
/// installation. Then create it like this:
/// ```
/// # // Dummy implementation for doctests only, do not use as a reference, use crate `http-adapter-reqwest` instead
/// # mod http_adapter_reqwest {
/// #    #[derive(Default)]
/// #    pub struct ReqwestAdapter;
/// #    #[async_trait::async_trait]
/// #    impl http_adapter::HttpClientAdapter for ReqwestAdapter {
/// #       type Error = String;
/// #       async fn execute(&self, request: http_adapter::Request<Vec<u8>>) -> Result<http_adapter::Response<Vec<u8>>, Self::Error> { Ok(http_adapter::Response::new(vec![])) }
/// #    }
/// # }
/// let client = solaredge::Client::<http_adapter_reqwest::ReqwestAdapter>::new("API_KEY");
/// ```
pub struct Client<C> {
	client: C,
	base_url: Url,
	api_key: String,
}

impl<C: HttpClientAdapter> Client<C> {
	/// Construct a new client using an HTTP client implementation that has [HttpClientAdapter::default()]
	///
	/// # Example
	/// ```
	/// # // Dummy implementation for doctests only, do not use as a reference, use `http-adapter-reqwest` crate instead
	/// # mod http_adapter_reqwest {
	/// #    #[derive(Default)]
	/// #    pub struct ReqwestAdapter;
	/// #    #[async_trait::async_trait]
	/// #    impl http_adapter::HttpClientAdapter for ReqwestAdapter {
	/// #       type Error = String;
	/// #       async fn execute(&self, request: http_adapter::Request<Vec<u8>>) -> Result<http_adapter::Response<Vec<u8>>, Self::Error> { Ok(http_adapter::Response::new(vec![])) }
	/// #    }
	/// # }
	/// let client = solaredge::Client::<http_adapter_reqwest::ReqwestAdapter>::new("API_KEY");
	/// ```
	pub fn new(api_key: impl Into<String>) -> Self
	where
		C: Default,
	{
		Self::new_with_client(C::default(), api_key)
	}

	/// Construct a new client using a passed [HttpClientAdapter] implementation
	///
	/// # Example
	/// ```
	/// # // Dummy implementation for doctests only, do not use as a reference, use `http-adapter-reqwest` crate instead
	/// # mod http_adapter_reqwest {
	/// #    #[derive(Default)]
	/// #    pub struct ReqwestAdapter;
	/// #    #[async_trait::async_trait]
	/// #    impl http_adapter::HttpClientAdapter for ReqwestAdapter {
	/// #       type Error = String;
	/// #       async fn execute(&self, request: http_adapter::Request<Vec<u8>>) -> Result<http_adapter::Response<Vec<u8>>, Self::Error> { Ok(http_adapter::Response::new(vec![])) }
	/// #    }
	/// # }
	/// let client = solaredge::Client::new_with_client(http_adapter_reqwest::ReqwestAdapter::default(), "API_KEY");
	/// ```
	pub fn new_with_client(client: C, api_key: impl Into<String>) -> Self {
		Self {
			client,
			base_url: Url::parse("https://monitoringapi.solaredge.com").expect("Static URL parsing failed"),
			api_key: api_key.into(),
		}
	}

	/// Return the most updated version number in <major.minor.revision> format.
	pub async fn version_current(&self) -> Result<String, Error<C::Error>> {
		self
			.fetch_json::<version::CurrentTop>("/version/current.json", ())
			.await
			.map(|res| res.version.release)
	}

	/// Return a list of supported version numbers in <major.minor.revision> format.
	pub async fn version_supported(&self) -> Result<Vec<version::Spec>, Error<C::Error>> {
		self
			.fetch_json::<version::SupportedTop>("/version/supported.json", ())
			.await
			.map(|res| res.supported)
	}

	/// Returns a list of sites related to the given token, which is the account api_key
	pub async fn sites_list(&self, params: &request::SitesList<'_>) -> Result<Vec<site::Details>, Error<C::Error>> {
		self
			.fetch_json::<site::ListTop>("/sites/list.json", params)
			.await
			.map(|res| res.sites.list)
	}

	/// Displays the site details, such as name, location, status, etc.
	pub async fn site_details(&self, site_id: u64) -> Result<site::Details, Error<C::Error>> {
		self
			.fetch_json::<site::DetailsTop>(&format!("/site/{site_id}/details.json"), ())
			.await
			.map(|res| res.details)
	}

	/// Return the energy production start and end dates of the site.
	pub async fn site_data_period(&self, site_id: u64) -> Result<site::DataPeriod, Error<C::Error>> {
		self
			.fetch_json::<site::DataPeriodTop>(&format!("/site/{site_id}/dataPeriod.json"), ())
			.await
			.map(|res| res.data_period)
	}

	/// Return the energy production start and end dates of the multiple sites.
	///
	/// Note that if the list contains site IDs for which the user has no permission to view, the system will generate a
	/// 403 Forbidden error with a proper description.
	pub async fn site_data_period_bulk(&self, site_ids: &[u64]) -> Result<Vec<site::SiteDataPeriod>, Error<C::Error>> {
		let site_ids_str = Self::join_site_ids(site_ids);
		self
			.fetch_json::<site::DataPeriodBulkTop>(&format!("/sites/{site_ids_str}/dataPeriod.json"), ())
			.await
			.map(|res| res.date_period_list.list)
	}

	/// Return the energy production start and end dates of the site.
	///
	/// Note: this API returns the same energy measurements that appear in the Site Dashboard.
	///
	/// Usage limitation: This API is limited to one year when using timeUnit=DAY (i.e., daily resolution) and to one
	/// month when using timeUnit=QUARTER_OF_AN_HOUR or timeUnit=HOUR. This means that the period between endTime and
	/// startTime should not exceed one year or one month respectively. If the period is longer, the system will
	/// generate error 403 with proper description.
	pub async fn site_energy(&self, site_id: u64, params: &request::SiteEnergy) -> Result<site::Energy, Error<C::Error>> {
		self
			.fetch_json::<site::EnergyTop>(&format!("/site/{site_id}/energy.json"), params)
			.await
			.map(|res| res.energy)
	}

	/// Return the energy production start and end dates of the multiple sites.
	///
	/// Note that if the list contains site IDs for which the user has no permission to view, the system will generate a
	/// 403 Forbidden error with a proper description.
	pub async fn site_energy_bulk(
		&self,
		site_ids: &[u64],
		params: &request::SiteEnergy,
	) -> Result<site::EnergyBulkList, Error<C::Error>> {
		let site_ids_str = Self::join_site_ids(site_ids);
		self
			.fetch_json::<site::EnergyBulkTop>(&format!("/sites/{site_ids_str}/energy.json"), params)
			.await
			.map(|res| res.sites_energy)
	}

	/// Return the site total energy produced for a given period.
	///
	/// Note: This API only returns on-grid energy for the requested period. In sites with storage/backup, this may mean
	/// that results can differ from what appears in the Site Dashboard. Use the regular Site Energy API to obtain
	/// results that match the Site Dashboard calculation.
	///
	/// Usage limitation: This API is limited to one year when using timeUnit=DAY (i.e., daily resolution). This means
	/// that the period between endTime and startTime should not exceed one year). If the period is longer, the system
	/// will generate error 403 with proper description
	pub async fn site_time_frame_energy(
		&self,
		site_id: u64,
		params: &request::SiteTotalEnergy,
	) -> Result<site::TimeframeEnergy, Error<C::Error>> {
		self
			.fetch_json::<site::TimeframeEnergyTop>(&format!("/site/{site_id}/timeFrameEnergy.json"), params)
			.await
			.map(|res| res.timeframe_energy)
	}

	/// Return the multiple sites total energy produced for a given period.
	///
	/// Note that if the list contains site IDs for which the user has no permission to view, the system will generate a
	/// 403 Forbidden error with a proper description.
	pub async fn site_time_frame_energy_bulk(
		&self,
		site_ids: &[u64],
		params: &request::SiteTotalEnergy,
	) -> Result<Vec<site::SiteTimeframeEnergy>, Error<C::Error>> {
		let site_ids_str = Self::join_site_ids(site_ids);
		self
			.fetch_json::<site::TimeframeEnergyBulkTop>(&format!("/sites/{site_ids_str}/timeFrameEnergy.json"), params)
			.await
			.map(|res| res.timeframe_energy_list.list)
	}

	/// Return the site power measurements in 15 minutes resolution.
	///
	/// Usage limitation: This API is limited to one-month period. This means that the period between endTime and
	/// startTime should not exceed one month. If the period is longer, the system will generate error 403 with proper
	/// description.
	pub async fn site_power(&self, site_id: u64, params: &request::DateTimeRange) -> Result<site::Power, Error<C::Error>> {
		self
			.fetch_json::<site::PowerTop>(&format!("/site/{site_id}/power.json"), params)
			.await
			.map(|res| res.power)
	}

	/// Return the multiple sites power measurements in 15 minutes resolution.
	///
	/// Note that if the list contains site IDs for which the user has no permission to view, the system will generate a
	/// 403 Forbidden error with a proper description.
	pub async fn site_power_bulk(
		&self,
		site_ids: &[u64],
		params: &request::DateTimeRange,
	) -> Result<site::PowerValueList, Error<C::Error>> {
		let site_ids_str = Self::join_site_ids(site_ids);
		self
			.fetch_json::<site::PowerBulkTop>(&format!("/sites/{site_ids_str}/power.json"), params)
			.await
			.map(|res| res.power_date_values_list)
	}

	/// Display the site overview data.
	pub async fn site_overview(&self, site_id: u64) -> Result<site::Overview, Error<C::Error>> {
		self
			.fetch_json::<site::OverviewTop>(&format!("/site/{site_id}/overview.json"), ())
			.await
			.map(|res| res.overview)
	}

	/// Display the multiple sites overview data.
	///
	/// Note that if the list contains site IDs for which the user has no permission to view, the system will generate a
	/// 403 Forbidden error with a proper description.
	pub async fn site_overview_bulk(&self, site_ids: &[u64]) -> Result<Vec<site::SiteOverview>, Error<C::Error>> {
		let site_ids_str = Self::join_site_ids(site_ids);
		self
			.fetch_json::<site::OverviewBulkTop>(&format!("/sites/{site_ids_str}/overview.json"), ())
			.await
			.map(|res| res.sites_overviews.list)
	}

	/// Detailed site power measurements from meters such as consumption, export (feed-in), import (purchase), etc.
	///
	/// Note: Calculated meter readings (also referred to as "virtual meters"), such as self-consumption, are calculated
	/// using the data measured by the meter and the inverters.
	///
	/// Usage limitation: This API is limited to one-month period. This means that the period between endTime and
	/// startTime should not exceed one month. If the period is longer, the system will generate error 403 with proper
	/// description.
	pub async fn site_power_details(
		&self,
		site_id: u64,
		params: &request::SitePowerDetails<'_>,
	) -> Result<site::PowerDetails, Error<C::Error>> {
		self
			.fetch_json::<site::PowerDetailsTop>(&format!("/site/{site_id}/powerDetails.json"), params)
			.await
			.map(|res| res.power_details)
	}

	/// Detailed site energy measurements from meters such as consumption, export (feed-in), import (purchase), etc.
	///
	/// Note: Calculated meter readings (also referred to as "virtual meters"), such as self-consumption, are calculated
	/// using the data measured by the meter and the inverters.
	///
	/// Usage limitation: This API is limited to:
	/// * A year when using daily resolution (timeUnit=DAY)
	/// * A month when using hourly resolution of higher (timeUnit=QUARTER_OF_AN_HOUR or timeUnit=HOUR)
	/// * Lower resolutions (weekly, monthly, yearly) have no period limitation
	///
	/// In case the requested resolution is not allowed for the requested period, error 403 with proper description will
	/// be returned.
	pub async fn site_energy_details(
		&self,
		site_id: u64,
		params: &request::MetersDateTimeRange<'_>,
	) -> Result<site::EnergyDetails, Error<C::Error>> {
		self
			.fetch_json::<site::EnergyDetailsTop>(&format!("/site/{site_id}/energyDetails.json"), params)
			.await
			.map(|res| res.energy_details)
	}

	/// Retrieves the current power flow between all elements of the site including PV array, storage (battery), loads (consumption) and grid.
	///
	/// Note: Applies when export, import and consumption can be measured.
	pub async fn site_current_power_flow(&self, site_id: u64) -> Result<site::CurrentPowerFlow, Error<C::Error>> {
		self
			.fetch_json::<site::CurrentPowerFlowTop>(&format!("/site/{site_id}/currentPowerFlow.json"), ())
			.await
			.map(|res| res.site_current_power_flow)
	}

	/// Get detailed storage information from batteries: the state of energy, power and lifetime energy.
	///
	/// Note: Applicable to systems with batteries.
	///
	/// Usage limitation: This API is limited to one-week period. Specifying a period that is longer than 7 days will
	/// generate error 403 with proper description.
	///
	/// Disclaimers:
	/// 1. As LG battery does not provide lifetime charge/discharge data, the monitoring system aggregates the delta
	///    charge/discharge values. In cases where telemetries containing delta energy values are lost or not sent, the
	///    calculated lifetime energy values will be incomplete. Values provided are not revenue grade.
	/// 2. AC coupling is not supported with 3rd party inverters.
	pub async fn site_storage_data(
		&self,
		site_id: u64,
		params: &request::SiteStorageData<'_>,
	) -> Result<Vec<site::StorageBattery>, Error<C::Error>> {
		self
			.fetch_json::<site::StorageDataTop>(&format!("/site/{site_id}/storageData.json"), params)
			.await
			.map(|res| res.storage_data.list)
	}

	/// Display the site image as uploaded by the user.
	///
	/// Performance: The image element returns with a hash element, which is consistent as long as the image is not
	/// changed. When executing the Site Image API while using the hash element, the server matches the image hash and
	/// the hash sent in the URL. If a match is found, the API returns an HTTP 304 code. In case the image hash that
	/// appears in the URL is different than the one stored in the server, the image will be downloaded. When using the
	/// maxWidth and MaxHeight parameters, the hash element will be ignored.
	///
	/// Image sizes: By default, the API returns the same image that was uploaded to the monitoring portal. If an image
	/// in a different scale is required, the API supports it via the maxWidth and maxHeight parameters. The system will
	/// scale the image while keeping the aspect ratio of the original image, so the returned image will be smaller.
	pub async fn site_image(&self, site_id: u64, params: &request::SiteImage) -> Result<Vec<u8>, Error<C::Error>> {
		self
			.fetch_image(&format!("/site/{site_id}/siteImage/image.jpg"), params)
			.await
	}

	/// Returns all environmental benefits based on site energy production: CO2 emissions saved, equivalent trees
	/// planted, and light bulbs powered for a day.
	pub async fn site_env_benefits(
		&self,
		site_id: u64,
		params: &request::SiteEnvBenefits,
	) -> Result<site::EnvBenefits, Error<C::Error>> {
		self
			.fetch_json::<site::EnvBenefitsTop>(&format!("/site/{site_id}/envBenefits.json"), params)
			.await
			.map(|res| res.env_benefits)
	}

	/// Return the site installer logo image as uploaded by the user. If such an image does not exist, the account
	/// installer logo is returned.
	pub async fn site_installer_image(&self, site_id: u64) -> Result<Vec<u8>, Error<C::Error>> {
		self
			.fetch_image(&format!("/site/{site_id}/installerImage/image.jpg"), ())
			.await
	}

	/// Return the inventory of SolarEdge equipment in the site, including inverters/SMIs, batteries, meters, gateways
	/// and sensors.
	pub async fn site_inventory(&self, site_id: u64) -> Result<site::Inventory, Error<C::Error>> {
		self
			.fetch_json::<site::InventoryTop>(&format!("/site/{site_id}/inventory.json"), ())
			.await
			.map(|res| res.inventory)
	}

	/// Returns for each meter on site its lifetime energy reading, metadata and the device to which it’s connected to.
	pub async fn site_meters(
		&self,
		site_id: u64,
		params: &request::MetersDateTimeRange<'_>,
	) -> Result<site::Meters, Error<C::Error>> {
		self
			.fetch_json::<site::MetersTop>(&format!("/site/{site_id}/meters.json"), params)
			.await
			.map(|res| res.meter_energy_details)
	}

	/// Returns the data of all the sensors in the site, by the gateway they are connected to.
	///
	/// Usage limitation: This API is limited to one-week period. This means that the period between endDate and
	/// startDate should not exceed one week. If the period is longer, the system will generate error 403 with a
	/// description.
	pub async fn site_sensor_data(
		&self,
		site_id: u64,
		params: &request::SensorsDateTimeRange,
	) -> Result<Vec<site::SensorData>, Error<C::Error>> {
		self
			.fetch_json::<site::SensorDataTop>(&format!("/site/{site_id}/sensors.json"), params)
			.await
			.map(|res| res.site_sensors.list)
	}

	/// Return a list of inverters/SMIs in the specific site.
	pub async fn equipment_list(&self, site_id: u64) -> Result<Vec<equipment::Reporter>, Error<C::Error>> {
		self
			.fetch_json::<equipment::ListTop>(&format!("/equipment/{site_id}/list.json"), ())
			.await
			.map(|res| res.reporters.list)
	}

	/// Returns for each meter on site its lifetime energy reading, metadata and the device to which it’s connected to.
	pub async fn equipment_sensors(&self, site_id: u64) -> Result<Vec<equipment::SensorSummary>, Error<C::Error>> {
		self
			.fetch_json::<equipment::SensorsTop>(&format!("/equipment/{site_id}/sensors.json"), ())
			.await
			.map(|res| res.site_sensors.list)
	}

	/// Return specific inverter data for a given timeframe.
	///
	/// Usage limitation: This API is limited to one-week period. This means that the period between endTime and
	/// startTime should not exceed one week. If the period is longer, the system will generate error 403 with proper
	/// description.
	pub async fn equipment_data(
		&self,
		site_id: u64,
		serial_number: &str,
		params: &request::DateTimeRange,
	) -> Result<Vec<equipment::Telemetry>, Error<C::Error>> {
		let serial_number = utf8_percent_encode(serial_number, NON_ALPHANUMERIC);
		self
			.fetch_json::<equipment::DataTop>(&format!("/equipment/{site_id}/{serial_number}/data.json"), params)
			.await
			.map(|res| res.data.list)
	}

	/// Description: Returns a list of equipment component replacements ordered by date. This method is applicable to
	/// inverters, optimizers, batteries and gateways.
	pub async fn equipment_changelog(
		&self,
		site_id: u64,
		serial_number: &str,
	) -> Result<Vec<equipment::EquipmentChangelog>, Error<C::Error>> {
		let serial_number = utf8_percent_encode(serial_number, NON_ALPHANUMERIC);
		self
			.fetch_json::<equipment::EquipmentChangelogTop>(&format!("/equipment/{site_id}/{serial_number}/changeLog.json"), ())
			.await
			.map(|res| res.changelog.list)
	}

	pub async fn accounts_list(&self, params: &request::AccountsList<'_>) -> Result<Vec<accounts::Account>, Error<C::Error>> {
		self
			.fetch_json::<accounts::ListTop>("/accounts/list.json", params)
			.await
			.map(|res| res.accounts.list)
	}

	fn join_site_ids(ids: &[u64]) -> String {
		let mut out = String::with_capacity(ids.len() * 10);
		let mut first = true;
		for id in ids {
			if first {
				write!(out, "{id}").expect("Impossible");
				first = false;
			} else {
				write!(out, ",{id}").expect("Impossible");
			}
		}
		out
	}

	fn debug_response(res: &Response<Vec<u8>>) -> String {
		for (name, value) in res.headers() {
			if name == CONTENT_TYPE && value.to_str().ok().is_some_and(|v| v.contains("application/json")) {
				return format!("{} {}", res.status(), String::from_utf8_lossy(res.body()));
			}
		}
		format!("{} Length: {} bytes", res.status(), res.body().len())
	}

	async fn perform_request(&self, url_path: &str, params: impl Serialize) -> Result<Response<Vec<u8>>, Error<C::Error>> {
		let mut url = self.base_url.join(url_path).expect("Static URL parsing failed");
		let query = serde_urlencoded::to_string(params)?;
		if !query.is_empty() {
			url.set_query(Some(&query));
		}
		trace!("{url_path}: url: {url}");
		let req = Request::get(url.to_string())
			.header("X-API-Key", &self.api_key)
			.body(vec![])
			.expect("Static request");
		let out = self
			.client
			.execute(req)
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("{url_path}: response: {}", Self::debug_response(&out));
		Ok(out)
	}

	async fn fetch_json<R: DeserializeOwned>(&self, url_path: &str, params: impl Serialize) -> Result<R, Error<C::Error>> {
		Ok(serde_json::from_slice(self.perform_request(url_path, params).await?.body())?)
	}

	async fn fetch_image(&self, url_path: &str, params: impl Serialize) -> Result<Vec<u8>, Error<C::Error>> {
		Ok(self.perform_request(url_path, params).await?.into_body())
	}
}

impl<C: Clone> Clone for Client<C> {
	fn clone(&self) -> Self {
		Self {
			client: self.client.clone(),
			base_url: self.base_url.clone(),
			api_key: self.api_key.clone(),
		}
	}
}

impl<C: fmt::Debug> fmt::Debug for Client<C> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Client")
			.field("client", &self.client)
			.field("base_url", &self.base_url)
			.field("api_key", &"<hidden>")
			.finish()
	}
}

trait ResponseExt: Sized {
	fn error_for_status<E>(self) -> Result<Self, Error<E>>;
}

impl ResponseExt for Response<Vec<u8>> {
	fn error_for_status<E>(self) -> Result<Self, Error<E>> {
		let status = self.status();
		if status.is_client_error() || status.is_server_error() {
			Err(Error::Api(status, String::from_utf8_lossy(self.body()).into_owned()))
		} else {
			Ok(self)
		}
	}
}
