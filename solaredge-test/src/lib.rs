#[cfg(test)]
mod tests {
	use chrono::{NaiveDate, NaiveTime};
	use tokio_test::block_on;

	use solaredge::{Client, DateTimeRange, SiteEnergy, SiteEnergyDetails, SitePowerDetails, SitesList, SiteStatus, SiteStorageData, SiteTotalEnergy, SortOrder, TimeUnit};
	use solaredge_reqwest::ReqwestAdapter;

	#[test]
	fn it_works() {
		env_logger::init();
		let c = Client::<ReqwestAdapter>::new("");
		let version = block_on(c.version_current()).unwrap();
		dbg!(&version);
		let version_supported = block_on(c.version_supported()).unwrap();
		dbg!(&version_supported);
		let mut p = SitesList::default();
		p.size = Some(32);
		p.sort_order = Some(SortOrder::Ascending);
		p.status = Some(&[SiteStatus::Active, SiteStatus::Pending]);
		p.search_text = Some("bbb");
		let sites = block_on(c.sites_list(&p)).unwrap();
		dbg!(&sites);
		for site in sites {
			let site = block_on(c.site_details(site.id)).unwrap();
			dbg!(&site);
			let data = block_on(c.site_data_period(site.id)).unwrap();
			dbg!(&data);
			let p = SiteEnergy {
				start_date: NaiveDate::from_ymd(2021, 8, 10),
				end_date: NaiveDate::from_ymd(2021, 8, 12),
				time_unit: Some(TimeUnit::QuarterOfAnHour),
			};
			let energy = block_on(c.site_energy(site.id, &p)).unwrap();
			dbg!(&energy);
			let p = SiteTotalEnergy {
				start_date: NaiveDate::from_ymd(2021, 8, 10),
				end_date: NaiveDate::from_ymd(2021, 8, 12),
			};
			let energy = block_on(c.site_time_frame_energy(site.id, &p)).unwrap();
			dbg!(&energy);
			let p = DateTimeRange {
				start_time: NaiveDate::from_ymd(2021, 8, 10).and_time(NaiveTime::from_hms(0, 0, 0)),
				end_time: NaiveDate::from_ymd(2021, 8, 12).and_time(NaiveTime::from_hms(0, 0, 0)),
			};
			let energy = block_on(c.site_power(site.id, &p)).unwrap();
			dbg!(&energy);
			let overview = block_on(c.site_overview(site.id)).unwrap();
			dbg!(&overview);
			let p = SitePowerDetails {
				start_time: NaiveDate::from_ymd(2021, 8, 10).and_time(NaiveTime::from_hms(0, 0, 0)),
				end_time: NaiveDate::from_ymd(2021, 8, 12).and_time(NaiveTime::from_hms(0, 0, 0)),
				meters: None,
			};
			let power_details = block_on(c.site_power_details(site.id, &p)).unwrap();
			dbg!(&power_details);
			let p = SiteEnergyDetails {
				start_time: NaiveDate::from_ymd(2021, 8, 10).and_time(NaiveTime::from_hms(0, 0, 0)),
				end_time: NaiveDate::from_ymd(2021, 8, 12).and_time(NaiveTime::from_hms(0, 0, 0)),
				time_unit: None,
				meters: None,
			};
			let energy_details = block_on(c.site_energy_details(site.id, &p)).unwrap();
			dbg!(&energy_details);
			let power_flow = block_on(c.site_current_power_flow(site.id)).unwrap();
			dbg!(&power_flow);
			let p = SiteStorageData {
				start_time: NaiveDate::from_ymd(2021, 8, 10).and_time(NaiveTime::from_hms(0, 0, 0)),
				end_time: NaiveDate::from_ymd(2021, 8, 12).and_time(NaiveTime::from_hms(0, 0, 0)),
				serials: None,
			};
			let storage_data = block_on(c.site_storage_data(site.id, &p)).unwrap();
			dbg!(&storage_data);
			let equipment = block_on(c.equipment_list(site.id)).unwrap();
			dbg!(&equipment);
			let inventory = block_on(c.site_inventory(site.id)).unwrap();
			dbg!(&inventory);
			for inv in inventory.inverters {
				let p = DateTimeRange {
					start_time: NaiveDate::from_ymd(2021, 8, 10).and_time(NaiveTime::from_hms(0, 0, 0)),
					end_time: NaiveDate::from_ymd(2021, 8, 12).and_time(NaiveTime::from_hms(0, 0, 0)),
				};
				let equipment_data = block_on(c.equipment_data(site.id, &inv.sn, &p)).unwrap();
				dbg!(&equipment_data);
			}
		}
	}
}
