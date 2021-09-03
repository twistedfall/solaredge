use std::fmt;

use log::trace;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde::Serialize;
use url::Url;

use crate::{
	api::{request, response},
	Error,
	HttpClientAdapter,
};

/// Client for accessing SolarEdge API
///
/// To be able to use it you'll need to request the API key from the Admin panel of your SolarEdge
/// installation. Then create it like this:
/// ```
/// # // Dummy implementation for doctests only, do not use as reference, use crate `solaredge-reqwest` instead
/// # mod solaredge_reqwest {
/// #    #[derive(Default)]
/// #    pub struct ReqwestAdapter;
/// #    #[async_trait::async_trait]
/// #    impl solaredge::HttpClientAdapter for ReqwestAdapter {
/// #       type Error = String;
/// #       async fn get(&self, url: url::Url) -> Result<String, Self::Error> { Ok("".to_string()) }
/// #    }
/// # }
/// let client = solaredge::Client::<solaredge_reqwest::ReqwestAdapter>::new("API_KEY");
/// ```
pub struct Client<C> {
	client: C,
	base_url: Url,
	api_key: String,
}

impl<C: HttpClientAdapter> Client<C> {
	/// Construct a new client using an HTTP client implementation that has `HttpClientAdapter::default()`
	///
	/// # Example
	/// ```
	/// # // Dummy implementation for doctests only, do not use as reference, use `solaredge-reqwest` crate instead
	/// # mod solaredge_reqwest {
	/// #    #[derive(Default)]
	/// #    pub struct ReqwestAdapter;
	/// #    #[async_trait::async_trait]
	/// #    impl solaredge::HttpClientAdapter for ReqwestAdapter {
	/// #       type Error = String;
	/// #       async fn get(&self, url: url::Url) -> Result<String, Self::Error> { Ok("".to_string()) }
	/// #    }
	/// # }
	/// let client = solaredge::Client::<solaredge_reqwest::ReqwestAdapter>::new("API_KEY");
	/// ```
	#[inline]
	pub fn new(api_key: impl Into<String>) -> Self where C: Default {
		Self::new_with_client(C::default(), api_key)
	}

	/// Construct a new client using a passed `HttpClientAdapter` implementation
	///
	/// # Example
	/// ```
	/// # // Dummy implementation for doctests only, do not use as reference, use `solaredge-reqwest` crate instead
	/// # mod solaredge_reqwest {
	/// #    #[derive(Default)]
	/// #    pub struct ReqwestAdapter;
	/// #    #[async_trait::async_trait]
	/// #    impl solaredge::HttpClientAdapter for ReqwestAdapter {
	/// #       type Error = String;
	/// #       async fn get(&self, url: url::Url) -> Result<String, Self::Error> { Ok("".to_string()) }
	/// #    }
	/// # }
	/// let client = solaredge::Client::new_with_client(solaredge_reqwest::ReqwestAdapter::default(), "API_KEY");
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

	/// Return the most updated version number in <major.minor.revision> format.
	pub async fn version_current(&self) -> Result<String, Error<C::Error>> {
		let url = self.prepare_url("/version/current.json", ())?;
		trace!("version_current, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("version_current, response: {}", res);
		let res = serde_json::from_str::<response::VersionCurrentTop>(&res)?;
		Ok(res.version.release)
	}

	/// Return a list of supported version numbers in <major.minor.revision> format.
	pub async fn version_supported(&self) -> Result<Vec<response::VersionSpec>, Error<C::Error>> {
		let url = self.prepare_url("/version/supported.json", ())?;
		trace!("version_supported, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("version_supported, response: {}", res);
		let res = serde_json::from_str::<response::VersionSupportedTop>(&res)?;
		Ok(res.supported)
	}

	/// Returns a list of sites related to the given token, which is the account api_key
	pub async fn sites_list(&self, params: &request::SitesList<'_>) -> Result<Vec<response::Site>, Error<C::Error>> {
		trace!("sites_list, params: {:?}", params);
		let url = self.prepare_url("/sites/list.json", params)?;
		trace!("sites_list, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("sites_list, response: {}", res);
		let res = serde_json::from_str::<response::SitesListTop>(&res)?;
		Ok(res.sites.site)
	}

	/// Displays the site details, such as name, location, status, etc.
	pub async fn site_details(&self, site_id: u64) -> Result<response::Site, Error<C::Error>> {
		trace!("site_details, site_id: {}", site_id);
		let url = self.prepare_url(&format!("/site/{}/details.json", site_id), ())?;
		trace!("site_details, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("site_details, response: {}", res);
		let res = serde_json::from_str::<response::SiteDetailsTop>(&res)?;
		Ok(res.details)
	}

	/// Return the energy production start and end dates of the site.
	pub async fn site_data_period(&self, site_id: u64) -> Result<response::DataPeriod, Error<C::Error>> {
		trace!("site_data_period, site_id: {}", site_id);
		let url = self.prepare_url(&format!("/site/{}/dataPeriod.json", site_id), ())?;
		trace!("site_data_period, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("site_data_period, response: {}", res);
		let res = serde_json::from_str::<response::SiteDataPeriodTop>(&res)?;
		Ok(res.data_period)
	}

	// todo site data bulk

	/// Return the energy production start and end dates of the site.
	pub async fn site_energy(&self, site_id: u64, params: &request::SiteEnergy) -> Result<response::SiteEnergy, Error<C::Error>> {
		trace!("site_energy, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/energy.json", site_id), params)?;
		trace!("site_energy, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("site_energy, response: {}", res);
		let res = serde_json::from_str::<response::SiteEnergyTop>(&res)?;
		Ok(res.energy)
	}

	// todo site energy bulk

	/// Return the site total energy produced for a given period.
	pub async fn site_time_frame_energy(&self, site_id: u64, params: &request::SiteTotalEnergy) -> Result<response::SiteTimeframeEnergy, Error<C::Error>> {
		trace!("site_time_frame_energy, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/timeFrameEnergy.json", site_id), params)?;
		trace!("site_time_frame_energy, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("site_time_frame_energy, response: {}", res);
		let res = serde_json::from_str::<response::SiteTimeframeEnergyTop>(&res)?;
		Ok(res.timeframe_energy)
	}

	// todo site total energy bulk

	/// Return the site power measurements in 15 minutes resolution.
	pub async fn site_power(&self, site_id: u64, params: &request::DateTimeRange) -> Result<response::SitePower, Error<C::Error>> {
		trace!("site_power, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/power.json", site_id), params)?;
		trace!("site_power, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("site_power, response: {}", res);
		let res = serde_json::from_str::<response::SitePowerTop>(&res)?;
		Ok(res.power)
	}

	// todo site power bulk

	/// Display the site overview data.
	pub async fn site_overview(&self, site_id: u64) -> Result<response::SiteOverview, Error<C::Error>> {
		trace!("site_overview, site_id: {}", site_id);
		let url = self.prepare_url(&format!("/site/{}/overview.json", site_id), ())?;
		trace!("site_overview, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("site_overview, response: {}", res);
		let res = serde_json::from_str::<response::SiteOverviewTop>(&res)?;
		Ok(res.overview)
	}

	// todo site overview bulk

	/// Detailed site power measurements from meters such as consumption, export (feed-in), import (purchase), etc.
	pub async fn site_power_details(&self, site_id: u64, params: &request::SitePowerDetails<'_>) -> Result<response::SiteMetersDetails, Error<C::Error>> {
		trace!("site_power_details, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/powerDetails.json", site_id), params)?;
		trace!("site_power_details, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("site_power_details, response: {}", res);
		let res = serde_json::from_str::<response::SitePowerDetailsTop>(&res)?;
		Ok(res.power_details)
	}

	/// Detailed site energy measurements from meters such as consumption, export (feed-in), import (purchase), etc.
	pub async fn site_energy_details(&self, site_id: u64, params: &request::MetersDateTimeRange<'_>) -> Result<response::SiteMetersDetails, Error<C::Error>> {
		trace!("site_energy_details, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/energyDetails.json", site_id), params)?;
		trace!("site_energy_details, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("site_energy_details, response: {}", res);
		let res = serde_json::from_str::<response::SiteEnergyDetailsTop>(&res)?;
		Ok(res.energy_details)
	}

	/// Retrieves the current power flow between all elements of the site including PV array, storage (battery), loads (consumption) and grid.
	pub async fn site_current_power_flow(&self, site_id: u64) -> Result<response::SiteCurrentPowerFlow, Error<C::Error>> {
		trace!("site_current_power_flow, site_id: {}", site_id);
		let url = self.prepare_url(&format!("/site/{}/currentPowerFlow.json", site_id), ())?;
		trace!("site_current_power_flow, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("site_current_power_flow, response: {}", res);
		let res = serde_json::from_str::<response::SiteCurrentPowerFlowTop>(&res)?;
		Ok(res.site_current_power_flow)
	}

	/// Get detailed storage information from batteries: the state of energy, power and lifetime energy.
	pub async fn site_storage_data(&self, site_id: u64, params: &request::SiteStorageData<'_>) -> Result<response::SiteStorageData, Error<C::Error>> {
		trace!("site_storage_data, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/storageData.json", site_id), params)?;
		trace!("site_storage_data, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("site_storage_data, response: {}", res);
		let res = serde_json::from_str::<response::SiteStorageDataTop>(&res)?;
		Ok(res.storage_data)
	}

	// todo site image

	/// Returns all environmental benefits based on site energy production: CO2 emissions saved, equivalent trees planted, and light bulbs powered for a day.
	pub async fn site_env_benefits(&self, site_id: u64, params: &request::SiteEnvBenefits) -> Result<response::SiteEnvBenefits, Error<C::Error>> {
		trace!("site_env_benefits, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/envBenefits.json", site_id), params)?;
		trace!("site_env_benefits, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("site_env_benefits, response: {}", res);
		let res = serde_json::from_str::<response::SiteEnvBenefitsTop>(&res)?;
		Ok(res.env_benefits)
	}

	// todo site installer logo image

	/// Return the inventory of SolarEdge equipment in the site, including inverters/SMIs, batteries, meters, gateways and sensors.
	pub async fn site_inventory(&self, site_id: u64) -> Result<response::SiteInventory, Error<C::Error>> {
		trace!("site_inventory, site_id: {}", site_id);
		let url = self.prepare_url(&format!("/site/{}/inventory.json", site_id), ())?;
		trace!("site_inventory, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("site_inventory, response: {}", res);
		let res = serde_json::from_str::<response::SiteInventoryTop>(&res)?;
		Ok(res.inventory)
	}

	/// Returns for each meter on site its lifetime energy reading, metadata and the device to which it’s connected to.
	pub async fn site_meters(&self, site_id: u64, params: &request::MetersDateTimeRange<'_>) -> Result<response::SiteMeters, Error<C::Error>> {
		trace!("site_meters, site_id: {}, params: {:?}", site_id, params);
		let url = self.prepare_url(&format!("/site/{}/meters.json", site_id), params)?;
		trace!("site_meters, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("site_meters, response: {}", res);
		let res = serde_json::from_str::<response::SiteMetersTop>(&res)?;
		Ok(res.meter_energy_details)
	}

	/// Return a list of inverters/SMIs in the specific site.
	pub async fn equipment_list(&self, site_id: u64) -> Result<Vec<response::Equipment>, Error<C::Error>> {
		trace!("equipment_list, site_id: {}", site_id);
		let url = self.prepare_url(&format!("/equipment/{}/list.json", site_id), ())?;
		trace!("equipment_list, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("equipment_list, response: {}", res);
		let res = serde_json::from_str::<response::EquipmentListTop>(&res)?;
		Ok(res.reporters.list)
	}

	/// Return specific inverter data for a given timeframe.
	pub async fn equipment_data(&self, site_id: u64, serial_number: &str, params: &request::DateTimeRange) -> Result<Vec<response::EquipmentTelemetry>, Error<C::Error>> {
		trace!("equipment_data, site_id: {}, params: {:?}", site_id, params);
		let serial_number = utf8_percent_encode(serial_number, NON_ALPHANUMERIC);
		let url = self.prepare_url(&format!("/equipment/{}/{}/data.json", site_id, serial_number), params)?;
		trace!("equipment_data, url: {}", url);
		let res = self.client.get(url).await.map_err(Error::HttpRequest)?;
		trace!("equipment_data, response: {}", res);
		let res = serde_json::from_str::<response::EquipmentDataTop>(&res)?;
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
