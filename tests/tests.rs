use chrono::{NaiveDate, NaiveTime};
use http_adapter_reqwest::ReqwestAdapter;
use solaredge::{
	Client, DateTimeRange, FilterSiteStatus, MetersDateTimeRange, SiteEnergy, SiteEnvBenefits, SitePowerDetails, SiteStorageData,
	SiteTotalEnergy, SitesList, SortOrder, SystemUnits, TimeUnit,
};

#[tokio::test]
#[ignore] // requires a real API key
async fn it_works() {
	env_logger::init();
	let c = Client::<ReqwestAdapter>::new("");
	let start_datetime = NaiveDate::from_ymd_opt(2024, 8, 10)
		.unwrap()
		.and_time(NaiveTime::from_hms_opt(10, 24, 19).unwrap());
	let end_datetime = NaiveDate::from_ymd_opt(2024, 8, 12)
		.unwrap()
		.and_time(NaiveTime::from_hms_opt(8, 12, 14).unwrap());
	let version = c.version_current().await.unwrap();
	dbg!(&version);
	let version_supported = c.version_supported().await.unwrap();
	dbg!(&version_supported);
	// let p = AccountsList {
	// 	size: None,
	// 	start_index: None,
	// 	search_text: None,
	// 	sort_property: None,
	// 	sort_order: None,
	// };
	// let accounts = c.accounts_list(&p).await.unwrap();
	// dbg!(&accounts);
	let p = SitesList {
		size: Some(32),
		sort_order: Some(SortOrder::Ascending),
		status: Some(&[FilterSiteStatus::Active, FilterSiteStatus::Pending]),
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
		start_date: start_datetime.date(),
		end_date: end_datetime.date(),
		time_unit: Some(TimeUnit::QuarterOfAnHour),
	};
	let energy_bulk = c.site_energy_bulk(&site_ids, &site_energy_param).await.unwrap();
	dbg!(&energy_bulk);
	let site_total_energy_param = SiteTotalEnergy {
		start_date: start_datetime.date(),
		end_date: end_datetime.date(),
	};
	let energy_bulk = c
		.site_time_frame_energy_bulk(&site_ids, &site_total_energy_param)
		.await
		.unwrap();
	dbg!(&energy_bulk);
	let date_time_range_param = DateTimeRange {
		start_time: start_datetime,
		end_time: end_datetime,
	};
	let power_bulk = c.site_power_bulk(&site_ids, &date_time_range_param).await.unwrap();
	dbg!(&power_bulk);
	let overview_bulk = c.site_overview_bulk(&site_ids).await.unwrap();
	dbg!(&overview_bulk);
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
			start_time: start_datetime,
			end_time: end_datetime,
			meters: None,
		};
		let power_details = c.site_power_details(site.id, &p).await.unwrap();
		dbg!(&power_details);
		let p = MetersDateTimeRange {
			start_time: start_datetime,
			end_time: end_datetime,
			time_unit: None,
			meters: None,
		};
		let energy_details = c.site_energy_details(site.id, &p).await.unwrap();
		dbg!(&energy_details);
		let power_flow = c.site_current_power_flow(site.id).await.unwrap();
		dbg!(&power_flow);
		let p = SiteStorageData {
			start_time: start_datetime,
			end_time: end_datetime,
			serials: None,
		};
		let storage_data = c.site_storage_data(site.id, &p).await.unwrap();
		dbg!(&storage_data);
		// let p = SiteImage {
		// 	max_width: None,
		// 	max_height: None,
		// 	hash: None,
		// };
		// let site_image = c.site_image(site.id, &p).await.unwrap();
		// dbg!(&site_image);
		// fs::write("/tmp/123.jpg", &site_image).unwrap();
		let p = SiteEnvBenefits {
			system_units: Some(SystemUnits::Imperial),
		};
		let env_benefits = c.site_env_benefits(site.id, &p).await.unwrap();
		dbg!(&env_benefits);
		let installer_image = c.site_installer_image(site.id).await.unwrap();
		dbg!(&installer_image.len());
		let p = MetersDateTimeRange {
			start_time: start_datetime,
			end_time: end_datetime,
			time_unit: None,
			meters: None,
		};
		let meters = c.site_meters(site.id, &p).await.unwrap();
		dbg!(&meters);
		// let p = SensorsDateTimeRange {
		// 	start_date: start_datetime,
		// 	end_date: end_datetime,
		// };
		// // let sensors = c.site_sensors(site.id, &p).await.unwrap();
		// dbg!(&sensors);
		let equipment = c.equipment_list(site.id).await.unwrap();
		dbg!(&equipment);
		let sensors = c.equipment_sensors(site.id).await.unwrap();
		dbg!(&sensors);
		let inventory = c.site_inventory(site.id).await.unwrap();
		dbg!(&inventory);
		for inv in inventory.inverters {
			let p = DateTimeRange {
				start_time: start_datetime,
				end_time: end_datetime,
			};
			let equipment_data = c.equipment_data(site.id, &inv.serial_number, &p).await.unwrap();
			dbg!(&equipment_data);
			let equipment_changelog = c.equipment_changelog(site.id, &inv.serial_number).await.unwrap();
			dbg!(&equipment_changelog);
		}
	}
}
