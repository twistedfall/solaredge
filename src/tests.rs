use chrono::{NaiveDate, NaiveTime};
use http_adapter_reqwest::ReqwestAdapter;
use tokio_test::block_on;

use crate::{
	Client, DateTimeRange, MetersDateTimeRange, SiteEnergy, SiteEnvBenefits, SitePowerDetails, SiteStatus, SiteStorageData,
	SiteTotalEnergy, SitesList, SortOrder, SystemUnits, TimeUnit,
};

#[test]
fn it_works() {
	env_logger::init();
	let c = Client::<ReqwestAdapter>::new("");
	let version = block_on(c.version_current()).unwrap();
	dbg!(&version);
	let version_supported = block_on(c.version_supported()).unwrap();
	dbg!(&version_supported);
	let p = SitesList {
		size: Some(32),
		sort_order: Some(SortOrder::Ascending),
		status: Some(&[SiteStatus::Active, SiteStatus::Pending]),
		search_text: Some("bbb"),
		..Default::default()
	};
	let sites = block_on(c.sites_list(&p)).unwrap();
	dbg!(&sites);
	let mut site_ids = sites.iter().map(|s| s.id).collect::<Vec<_>>();
	if site_ids.len() == 1 {
		site_ids.push(site_ids.first().copied().unwrap_or_default());
	}
	let data_bulk = block_on(c.site_data_period_bulk(&site_ids)).unwrap();
	dbg!(&data_bulk);
	let site_energy_param = SiteEnergy {
		start_date: NaiveDate::from_ymd_opt(2021, 8, 10).unwrap(),
		end_date: NaiveDate::from_ymd_opt(2021, 8, 12).unwrap(),
		time_unit: Some(TimeUnit::QuarterOfAnHour),
	};
	let energy_bulk = block_on(c.site_energy_bulk(&site_ids, &site_energy_param)).unwrap();
	dbg!(&energy_bulk);
	let site_total_energy_param = SiteTotalEnergy {
		start_date: NaiveDate::from_ymd_opt(2021, 8, 10).unwrap(),
		end_date: NaiveDate::from_ymd_opt(2021, 8, 12).unwrap(),
	};
	let energy_bulk = block_on(c.site_time_frame_energy_bulk(&site_ids, &site_total_energy_param)).unwrap();
	dbg!(&energy_bulk);
	let date_time_range_param = DateTimeRange {
		start_time: NaiveDate::from_ymd_opt(2021, 8, 10)
			.unwrap()
			.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
		end_time: NaiveDate::from_ymd_opt(2021, 8, 12)
			.unwrap()
			.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
	};
	let power_bulk = block_on(c.site_power_bulk(&site_ids, &date_time_range_param)).unwrap();
	dbg!(&power_bulk);
	for site in sites {
		let site = block_on(c.site_details(site.id)).unwrap();
		dbg!(&site);
		let data = block_on(c.site_data_period(site.id)).unwrap();
		dbg!(&data);
		let energy = block_on(c.site_energy(site.id, &site_energy_param)).unwrap();
		dbg!(&energy);
		let energy = block_on(c.site_time_frame_energy(site.id, &site_total_energy_param)).unwrap();
		dbg!(&energy);
		let power = block_on(c.site_power(site.id, &date_time_range_param)).unwrap();
		dbg!(&power);
		let overview = block_on(c.site_overview(site.id)).unwrap();
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
		let power_details = block_on(c.site_power_details(site.id, &p)).unwrap();
		dbg!(&power_details);
		let energy_details = block_on(
			c.site_energy_details(
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
			),
		)
		.unwrap();
		dbg!(&energy_details);
		let power_flow = block_on(c.site_current_power_flow(site.id)).unwrap();
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
		let storage_data = block_on(c.site_storage_data(site.id, &p)).unwrap();
		dbg!(&storage_data);
		let env_benefits = block_on(c.site_env_benefits(
			site.id,
			&SiteEnvBenefits {
				system_units: Some(SystemUnits::Metrics),
			},
		))
		.unwrap();
		dbg!(&env_benefits);
		let meters = block_on(
			c.site_meters(
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
			),
		)
		.unwrap();
		dbg!(&meters);
		let equipment = block_on(c.equipment_list(site.id)).unwrap();
		dbg!(&equipment);
		let inventory = block_on(c.site_inventory(site.id)).unwrap();
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
			let equipment_data = block_on(c.equipment_data(site.id, &inv.sn, &p)).unwrap();
			dbg!(&equipment_data);
		}
	}
}
