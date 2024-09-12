use std::fmt;
use std::fmt::Write;

use http_adapter::{HttpClientAdapter, Request, Response};
use log::trace;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::Serialize;
use url::Url;

use crate::api::request;
use crate::{response, Error};

/// Client for accessing SolarEdge API
///
/// To be able to use it you'll need to request the API key from the Admin panel of your SolarEdge
/// installation. Then create it like this:
/// ```
/// # // Dummy implementation for doctests only, do not use as reference, use crate `http-adapter-reqwest` instead
/// # mod http_adapter_reqwest {
/// #    #[derive(Default)]
/// #    pub struct ReqwestAdapter;
/// #    #[async_trait::async_trait(?Send)]
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
	/// # // Dummy implementation for doctests only, do not use as reference, use `http-adapter-reqwest` crate instead
	/// # mod http_adapter_reqwest {
	/// #    #[derive(Default)]
	/// #    pub struct ReqwestAdapter;
	/// #    #[async_trait::async_trait(?Send)]
	/// #    impl http_adapter::HttpClientAdapter for ReqwestAdapter {
	/// #       type Error = String;
	/// #       async fn execute(&self, request: http_adapter::Request<Vec<u8>>) -> Result<http_adapter::Response<Vec<u8>>, Self::Error> { Ok(http_adapter::Response::new(vec![])) }
	/// #    }
	/// # }
	/// let client = solaredge::Client::<http_adapter_reqwest::ReqwestAdapter>::new("API_KEY");
	/// ```
	#[inline]
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
	/// # // Dummy implementation for doctests only, do not use as reference, use `http-adapter-reqwest` crate instead
	/// # mod http_adapter_reqwest {
	/// #    #[derive(Default)]
	/// #    pub struct ReqwestAdapter;
	/// #    #[async_trait::async_trait(?Send)]
	/// #    impl http_adapter::HttpClientAdapter for ReqwestAdapter {
	/// #       type Error = String;
	/// #       async fn execute(&self, request: http_adapter::Request<Vec<u8>>) -> Result<http_adapter::Response<Vec<u8>>, Self::Error> { Ok(http_adapter::Response::new(vec![])) }
	/// #    }
	/// # }
	/// let client = solaredge::Client::new_with_client(http_adapter_reqwest::ReqwestAdapter::default(), "API_KEY");
	/// ```
	#[inline]
	pub fn new_with_client(client: C, api_key: impl Into<String>) -> Self {
		Self {
			client,
			base_url: Url::parse("https://monitoringapi.solaredge.com").expect("Static URL parsing failed"),
			api_key: api_key.into(),
		}
	}

	fn prepare_url<E>(&self, path: &str, params: impl Serialize) -> Result<Url, Error<E>> {
		let mut out = self.base_url.join(path).expect("Static URL parsing failed");
		let query = serde_urlencoded::to_string(params)?;
		if !query.is_empty() {
			out.set_query(Some(&query));
		}
		out.query_pairs_mut().append_pair("api_key", &self.api_key);
		Ok(out)
	}

	fn request_get(url: Url) -> Request<Vec<u8>> {
		Request::get(url.to_string()).body(vec![]).unwrap()
	}

	fn join_site_ids(ids: &[u64]) -> String {
		let mut out = String::with_capacity(ids.len() * 10);
		let mut first = true;
		for id in ids {
			if first {
				write!(out, "{}", id).expect("Impossible");
				first = false;
			} else {
				write!(out, ",{}", id).expect("Impossible");
			}
		}
		out
	}

	/// Return the most updated version number in <major.minor.revision> format.
	pub async fn version_current(&self) -> Result<String, Error<C::Error>> {
		let url = self.prepare_url("/version/current.json", ())?;
		trace!("version_current, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("version_current, response: {:?}", res);
		let res = serde_json::from_slice::<response::VersionCurrentTop>(res.body())?;
		Ok(res.version.release)
	}

	/// Return a list of supported version numbers in <major.minor.revision> format.
	pub async fn version_supported(&self) -> Result<Vec<response::VersionSpec>, Error<C::Error>> {
		let url = self.prepare_url("/version/supported.json", ())?;
		trace!("version_supported, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("version_supported, response: {:?}", res);
		let res = serde_json::from_slice::<response::VersionSupportedTop>(res.body())?;
		Ok(res.supported)
	}

	/// Returns a list of sites related to the given token, which is the account api_key
	pub async fn sites_list(&self, params: &request::SitesList<'_>) -> Result<Vec<response::Site>, Error<C::Error>> {
		trace!("sites_list, params: {:?}", params);
		let url = self.prepare_url("/sites/list.json", params)?;
		trace!("sites_list, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("sites_list, response: {:?}", res);
		let res = serde_json::from_slice::<response::SitesListTop>(res.body())?;
		Ok(res.sites.site)
	}

	/// Displays the site details, such as name, location, status, etc.
	pub async fn site_details(&self, site_id: u64) -> Result<response::Site, Error<C::Error>> {
		trace!("site_details, site_id: {}", site_id);
		let url = self.prepare_url(&format!("/site/{}/details.json", site_id), ())?;
		trace!("site_details, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_details, response: {:?}", res);
		let res = serde_json::from_slice::<response::SiteDetailsTop>(res.body())?;
		Ok(res.details)
	}

	/// Return the energy production start and end dates of the site.
	pub async fn site_data_period(&self, site_id: u64) -> Result<response::DataPeriod, Error<C::Error>> {
		trace!("site_data_period, site_id: {}", site_id);
		let url = self.prepare_url(&format!("/site/{}/dataPeriod.json", site_id), ())?;
		trace!("site_data_period, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_data_period, response: {:?}", res);
		let res = serde_json::from_slice::<response::SiteDataPeriodTop>(res.body())?;
		Ok(res.data_period)
	}

	/// Return the energy production start and end dates of the multiple sites.
	pub async fn site_data_period_bulk(&self, site_ids: &[u64]) -> Result<Vec<response::DataPeriodBulk>, Error<C::Error>> {
		trace!("site_data_period_bulk, site_ids: {:?}", site_ids);
		let site_ids_str = Self::join_site_ids(site_ids);
		let url = self.prepare_url(&format!("/sites/{}/dataPeriod.json", site_ids_str), ())?;
		trace!("site_data_period_bulk, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_data_period_bulk, response: {:?}", res);
		let res = serde_json::from_slice::<response::SiteDataPeriodBulkTop>(res.body())?;
		Ok(res.date_period_list.site_energy_list)
	}

	/// Return the energy production start and end dates of the site.
	pub async fn site_energy(&self, site_id: u64, params: &request::SiteEnergy) -> Result<response::SiteEnergy, Error<C::Error>> {
		trace!("site_energy, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/energy.json", site_id), params)?;
		trace!("site_energy, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_energy, response: {:?}", res);
		let res = serde_json::from_slice::<response::SiteEnergyTop>(res.body())?;
		Ok(res.energy)
	}

	/// Return the energy production start and end dates of the multiple sites.
	pub async fn site_energy_bulk(
		&self,
		site_ids: &[u64],
		params: &request::SiteEnergy,
	) -> Result<response::SiteEnergyBulkList, Error<C::Error>> {
		trace!("site_energy_bulk, site_ids: {:?}, params: {:?}", site_ids, params);
		let site_ids_str = Self::join_site_ids(site_ids);
		let url = self.prepare_url(&format!("/sites/{}/energy.json", site_ids_str), params)?;
		trace!("site_energy_bulk, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_energy_bulk, response: {:?}", res);
		let res = serde_json::from_slice::<response::SiteEnergyBulkTop>(res.body())?;
		Ok(res.sites_energy)
	}

	/// Return the site total energy produced for a given period.
	pub async fn site_time_frame_energy(
		&self,
		site_id: u64,
		params: &request::SiteTotalEnergy,
	) -> Result<response::SiteTimeframeEnergy, Error<C::Error>> {
		trace!("site_time_frame_energy, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/timeFrameEnergy.json", site_id), params)?;
		trace!("site_time_frame_energy, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_time_frame_energy, response: {:?}", res);
		let res = serde_json::from_slice::<response::SiteTimeframeEnergyTop>(res.body())?;
		Ok(res.timeframe_energy)
	}

	/// Return the multiple sites total energy produced for a given period.
	pub async fn site_time_frame_energy_bulk(
		&self,
		site_ids: &[u64],
		params: &request::SiteTotalEnergy,
	) -> Result<Vec<response::SiteTimeframeEnergyBulk>, Error<C::Error>> {
		trace!("site_time_frame_energy_bulk, site_ids: {:?}, params: {:?}", site_ids, params);
		let site_ids_str = Self::join_site_ids(site_ids);
		let url = self.prepare_url(&format!("/sites/{}/timeFrameEnergy.json", site_ids_str), params)?;
		trace!("site_time_frame_energy_bulk, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_time_frame_energy_bulk, response: {:?}", res);
		let res = serde_json::from_slice::<response::SiteTimeframeEnergyBulkTop>(res.body())?;
		Ok(res.timeframe_energy_list.timeframe_energy_list)
	}

	/// Return the site power measurements in 15 minutes resolution.
	pub async fn site_power(&self, site_id: u64, params: &request::DateTimeRange) -> Result<response::SitePower, Error<C::Error>> {
		trace!("site_power, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/power.json", site_id), params)?;
		trace!("site_power, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_power, response: {:?}", res);
		let res = serde_json::from_slice::<response::SitePowerTop>(res.body())?;
		Ok(res.power)
	}

	/// Return the multiple sites power measurements in 15 minutes resolution.
	pub async fn site_power_bulk(
		&self,
		site_ids: &[u64],
		params: &request::DateTimeRange,
	) -> Result<response::SitePowerValueList, Error<C::Error>> {
		trace!("site_power_bulk, site_ids: {:?}, params: {:?}", site_ids, params);
		let site_ids_str = Self::join_site_ids(site_ids);
		let url = self.prepare_url(&format!("/sites/{}/power.json", site_ids_str), params)?;
		trace!("site_power_bulk, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_power_bulk, response: {:?}", res);
		let res = serde_json::from_slice::<response::SitePowerBulkTop>(res.body())?;
		Ok(res.power_date_values_list)
	}

	/// Display the site overview data.
	pub async fn site_overview(&self, site_id: u64) -> Result<response::SiteOverview, Error<C::Error>> {
		trace!("site_overview, site_id: {}", site_id);
		let url = self.prepare_url(&format!("/site/{}/overview.json", site_id), ())?;
		trace!("site_overview, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_overview, response: {:?}", res);
		let res = serde_json::from_slice::<response::SiteOverviewTop>(res.body())?;
		Ok(res.overview)
	}

	// todo site overview bulk

	/// Detailed site power measurements from meters such as consumption, export (feed-in), import (purchase), etc.
	pub async fn site_power_details(
		&self,
		site_id: u64,
		params: &request::SitePowerDetails<'_>,
	) -> Result<response::SiteMetersDetails, Error<C::Error>> {
		trace!("site_power_details, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/powerDetails.json", site_id), params)?;
		trace!("site_power_details, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_power_details, response: {:?}", res);
		let res = serde_json::from_slice::<response::SitePowerDetailsTop>(res.body())?;
		Ok(res.power_details)
	}

	/// Detailed site energy measurements from meters such as consumption, export (feed-in), import (purchase), etc.
	pub async fn site_energy_details(
		&self,
		site_id: u64,
		params: &request::MetersDateTimeRange<'_>,
	) -> Result<response::SiteMetersDetails, Error<C::Error>> {
		trace!("site_energy_details, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/energyDetails.json", site_id), params)?;
		trace!("site_energy_details, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_energy_details, response: {:?}", res);
		let res = serde_json::from_slice::<response::SiteEnergyDetailsTop>(res.body())?;
		Ok(res.energy_details)
	}

	/// Retrieves the current power flow between all elements of the site including PV array, storage (battery), loads (consumption) and grid.
	pub async fn site_current_power_flow(&self, site_id: u64) -> Result<response::SiteCurrentPowerFlow, Error<C::Error>> {
		trace!("site_current_power_flow, site_id: {}", site_id);
		let url = self.prepare_url(&format!("/site/{}/currentPowerFlow.json", site_id), ())?;
		trace!("site_current_power_flow, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_current_power_flow, response: {:?}", res);
		let res = serde_json::from_slice::<response::SiteCurrentPowerFlowTop>(res.body())?;
		Ok(res.site_current_power_flow)
	}

	/// Get detailed storage information from batteries: the state of energy, power and lifetime energy.
	pub async fn site_storage_data(
		&self,
		site_id: u64,
		params: &request::SiteStorageData<'_>,
	) -> Result<response::SiteStorageData, Error<C::Error>> {
		trace!("site_storage_data, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/storageData.json", site_id), params)?;
		trace!("site_storage_data, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_storage_data, response: {:?}", res);
		let res = serde_json::from_slice::<response::SiteStorageDataTop>(res.body())?;
		Ok(res.storage_data)
	}

	// todo site image

	/// Returns all environmental benefits based on site energy production: CO2 emissions saved, equivalent trees planted, and light bulbs powered for a day.
	pub async fn site_env_benefits(
		&self,
		site_id: u64,
		params: &request::SiteEnvBenefits,
	) -> Result<response::SiteEnvBenefits, Error<C::Error>> {
		trace!("site_env_benefits, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/envBenefits.json", site_id), params)?;
		trace!("site_env_benefits, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_env_benefits, response: {:?}", res);
		let res = serde_json::from_slice::<response::SiteEnvBenefitsTop>(res.body())?;
		Ok(res.env_benefits)
	}

	// todo site installer logo image

	/// Return the inventory of SolarEdge equipment in the site, including inverters/SMIs, batteries, meters, gateways and sensors.
	pub async fn site_inventory(&self, site_id: u64) -> Result<response::SiteInventory, Error<C::Error>> {
		trace!("site_inventory, site_id: {}", site_id);
		let url = self.prepare_url(&format!("/site/{}/inventory.json", site_id), ())?;
		trace!("site_inventory, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_inventory, response: {:?}", res);
		let res = serde_json::from_slice::<response::SiteInventoryTop>(res.body())?;
		Ok(res.inventory)
	}

	/// Returns for each meter on site its lifetime energy reading, metadata and the device to which it’s connected to.
	pub async fn site_meters(
		&self,
		site_id: u64,
		params: &request::MetersDateTimeRange<'_>,
	) -> Result<response::SiteMeters, Error<C::Error>> {
		trace!("site_meters, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/meters.json", site_id), params)?;
		trace!("site_meters, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("site_meters, response: {:?}", res);
		let res = serde_json::from_slice::<response::SiteMetersTop>(res.body())?;
		Ok(res.meter_energy_details)
	}

	/// Return a list of inverters/SMIs in the specific site.
	pub async fn equipment_list(&self, site_id: u64) -> Result<Vec<response::Equipment>, Error<C::Error>> {
		trace!("equipment_list, site_id: {}", site_id);
		let url = self.prepare_url(&format!("/equipment/{}/list.json", site_id), ())?;
		trace!("equipment_list, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("equipment_list, response: {:?}", res);
		let res = serde_json::from_slice::<response::EquipmentListTop>(res.body())?;
		Ok(res.reporters.list)
	}

	/// Return specific inverter data for a given timeframe.
	pub async fn equipment_data(
		&self,
		site_id: u64,
		serial_number: &str,
		params: &request::DateTimeRange,
	) -> Result<Vec<response::EquipmentTelemetry>, Error<C::Error>> {
		trace!("equipment_data, site_id: {}, params: {:?}", site_id, params);
		let serial_number = utf8_percent_encode(serial_number, NON_ALPHANUMERIC);
		let url = self.prepare_url(&format!("/equipment/{}/{}/data.json", site_id, serial_number), params)?;
		trace!("equipment_data, url: {}", url);
		let res = self
			.client
			.execute(Self::request_get(url))
			.await
			.map_err(Error::HttpRequest)?
			.error_for_status()?;
		trace!("equipment_data, response: {:?}", res);
		let res = serde_json::from_slice::<response::EquipmentDataTop>(res.body())?;
		Ok(res.data.telemetries)
	}

	// todo equipment changelog
	// todo account list api
	// todo sensors api
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
			Err(Error::Api(status, self.into_body()))
		} else {
			Ok(self)
		}
	}
}
