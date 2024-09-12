use chrono::{NaiveDate, NaiveTime};
use http_adapter_reqwest::ReqwestAdapter;

use crate::{
	Client, DateTimeRange, MetersDateTimeRange, SiteEnergy, SiteEnvBenefits, SitePowerDetails, SiteStatus, SiteStorageData,
	SiteTotalEnergy, SitesList, SortOrder, SystemUnits, TimeUnit,
};

#[tokio::test]
async fn it_works() {
	env_logger::init();
	let c = Client::<ReqwestAdapter>::new("");
	let version = c.version_current().await.unwrap();
	dbg!(&version);
	let version_supported = c.version_supported().await.unwrap();
	dbg!(&version_supported);
	let p = SitesList {
		size: Some(32),
		sort_order: Some(SortOrder::Ascending),
		status: Some(&[SiteStatus::Active, SiteStatus::Pending]),
		search_text: Some("bbb"),
		..Default::default()
	};
	let sites = c.sites_list(&p).await.unwrap();
	dbg!(&sites);
	let mut site_ids = sites.iter().map(|s| s.id).collect::<Vec<_>>();
	if site_ids.len() == 1 {
		site_ids.push(site_ids.first().copied().unwrap_or_default());
	}
	let data_bulk = c.site_data_period_bulk(&site_ids).await.unwrap();
	dbg!(&data_bulk);
	let site_energy_param = SiteEnergy {
		start_date: NaiveDate::from_ymd_opt(2021, 8, 10).unwrap(),
		end_date: NaiveDate::from_ymd_opt(2021, 8, 12).unwrap(),
		time_unit: Some(TimeUnit::QuarterOfAnHour),
	};
	let energy_bulk = c.site_energy_bulk(&site_ids, &site_energy_param).await.unwrap();
	dbg!(&energy_bulk);
	let site_total_energy_param = SiteTotalEnergy {
		start_date: NaiveDate::from_ymd_opt(2021, 8, 10).unwrap(),
		end_date: NaiveDate::from_ymd_opt(2021, 8, 12).unwrap(),
	};
	let energy_bulk = c
		.site_time_frame_energy_bulk(&site_ids, &site_total_energy_param)
		.await
		.unwrap();
	dbg!(&energy_bulk);
	let date_time_range_param = DateTimeRange {
		start_time: NaiveDate::from_ymd_opt(2021, 8, 10)
			.unwrap()
			.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
		end_time: NaiveDate::from_ymd_opt(2021, 8, 12)
			.unwrap()
			.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
	};
	let power_bulk = c.site_power_bulk(&site_ids, &date_time_range_param).await.unwrap();
	dbg!(&power_bulk);
	for site in sites {
		let site = c.site_details(site.id).await.unwrap();
		dbg!(&site);
		let data = c.site_data_period(site.id).await.unwrap();
		dbg!(&data);
		let energy = c.site_energy(site.id, &site_energy_param).await.unwrap();
		dbg!(&energy);
		let energy = c.site_time_frame_energy(site.id, &site_total_energy_param).await.unwrap();
		dbg!(&energy);
		let power = c.site_power(site.id, &date_time_range_param).await.unwrap();
		dbg!(&power);
		let overview = c.site_overview(site.id).await.unwrap();
		dbg!(&overview);
		let p = SitePowerDetails {
			start_time: NaiveDate::from_ymd_opt(2021, 8, 10)
				.unwrap()
				.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
			end_time: NaiveDate::from_ymd_opt(2021, 8, 12)
				.unwrap()
				.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
			meters: None,
		};
		let power_details = c.site_power_details(site.id, &p).await.unwrap();
		dbg!(&power_details);
		let energy_details = c
			.site_energy_details(
				site.id,
				&MetersDateTimeRange {
					start_time: NaiveDate::from_ymd_opt(2021, 8, 10)
						.unwrap()
						.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
					end_time: NaiveDate::from_ymd_opt(2021, 8, 12)
						.unwrap()
						.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
					time_unit: None,
					meters: None,
				},
			)
			.await
			.unwrap();
		dbg!(&energy_details);
		let power_flow = c.site_current_power_flow(site.id).await.unwrap();
		dbg!(&power_flow);
		let p = SiteStorageData {
			start_time: NaiveDate::from_ymd_opt(2021, 8, 10)
				.unwrap()
				.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
			end_time: NaiveDate::from_ymd_opt(2021, 8, 12)
				.unwrap()
				.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
			serials: None,
		};
		let storage_data = c.site_storage_data(site.id, &p).await.unwrap();
		dbg!(&storage_data);
		let env_benefits = c
			.site_env_benefits(
				site.id,
				&SiteEnvBenefits {
					system_units: Some(SystemUnits::Metrics),
				},
			)
			.await
			.unwrap();
		dbg!(&env_benefits);
		let meters = c
			.site_meters(
				site.id,
				&MetersDateTimeRange {
					start_time: NaiveDate::from_ymd_opt(2021, 8, 10)
						.unwrap()
						.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
					end_time: NaiveDate::from_ymd_opt(2021, 8, 12)
						.unwrap()
						.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
					time_unit: None,
					meters: None,
				},
			)
			.await
			.unwrap();
		dbg!(&meters);
		let equipment = c.equipment_list(site.id).await.unwrap();
		dbg!(&equipment);
		let inventory = c.site_inventory(site.id).await.unwrap();
		dbg!(&inventory);
		for inv in inventory.inverters {
			let p = DateTimeRange {
				start_time: NaiveDate::from_ymd_opt(2021, 8, 10)
					.unwrap()
					.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
				end_time: NaiveDate::from_ymd_opt(2021, 8, 12)
					.unwrap()
					.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
			};
			let equipment_data = c.equipment_data(site.id, &inv.sn, &p).await.unwrap();
			dbg!(&equipment_data);
		}
	}
}
