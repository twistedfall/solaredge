use chrono::{NaiveDate, NaiveTime};
use http_adapter_reqwest::ReqwestAdapter;
use serde_json::json;
use solaredge::response::equipment::LData;
use solaredge::{
	Client, DateTimeRange, GasEmissionUnit, MetersDateTimeRange, SiteEnergy, SiteEnvBenefits, SitePowerDetails, SiteStorageData,
	SiteTotalEnergy, SystemUnits, TimeUnit,
};

#[test]
fn gas_emission_units_accept_known_spellings() {
	for unit in ["kg", "Kg", "KG"] {
		let parsed: GasEmissionUnit = serde_json::from_value(json!(unit)).unwrap();
		assert!(matches!(parsed, GasEmissionUnit::Kg), "{unit}");
	}
	for unit in ["lb", "Lb", "LB"] {
		let parsed: GasEmissionUnit = serde_json::from_value(json!(unit)).unwrap();
		assert!(matches!(parsed, GasEmissionUnit::Lb), "{unit}");
	}
}

#[test]
fn phase_telemetry_accepts_missing_active_power() {
	let mut phase = json!({ "acCurrent": 8.5, "acVoltage": 230.0, "acFrequency": 50.0 });
	let without_active_power: LData = serde_json::from_value(phase.clone()).unwrap();
	assert_eq!(without_active_power.active_power, None);

	phase["activePower"] = json!(1954.0);
	let with_active_power: LData = serde_json::from_value(phase).unwrap();
	assert_eq!(with_active_power.active_power, Some(1954.0));
}

#[tokio::test]
#[ignore] // requires a real API key
async fn it_works() {
	env_logger::init();
	let site_id = 0;
	let c = Client::<ReqwestAdapter>::new("");
	let start_datetime = NaiveDate::from_ymd_opt(2024, 8, 10)
		.unwrap()
		.and_time(NaiveTime::from_hms_opt(10, 24, 19).unwrap());
	let end_datetime = NaiveDate::from_ymd_opt(2024, 8, 12)
		.unwrap()
		.and_time(NaiveTime::from_hms_opt(8, 12, 14).unwrap());
	// 403 since March 2026
	// let p = SitesList {
	// 	size: Some(32),
	// 	sort_order: Some(SortOrder::Ascending),
	// 	status: Some(&[FilterSiteStatus::Active, FilterSiteStatus::Pending]),
	// 	search_text: Some("bbb"),
	// 	..Default::default()
	// };
	// let sites = c.sites_list(&p).await.unwrap();
	// dbg!(&sites);
	// let mut site_ids = sites.iter().map(|s| s.id).collect::<Vec<_>>();
	let mut site_ids = vec![site_id];
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
	for site_id in site_ids {
		let site = c.site_details(site_id).await.unwrap();
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
		let p = SiteEnvBenefits {
			system_units: Some(SystemUnits::Imperial),
		};
		let env_benefits = c.site_env_benefits(site.id, &p).await.unwrap();
		dbg!(&env_benefits);
		let p = MetersDateTimeRange {
			start_time: start_datetime,
			end_time: end_datetime,
			time_unit: None,
			meters: None,
		};
		let meters = c.site_meters(site.id, &p).await.unwrap();
		dbg!(&meters);
		let equipment = c.equipment_list(site.id).await.unwrap();
		dbg!(&equipment);
		let inventory = c.site_inventory(site.id).await.unwrap();
		dbg!(&inventory);
		for inv in inventory.inverters {
			let p = DateTimeRange {
				start_time: start_datetime,
				end_time: end_datetime,
			};
			let equipment_data = c.equipment_data(site.id, &inv.serial_number, &p).await.unwrap();
			dbg!(&equipment_data);
		}
	}
}
