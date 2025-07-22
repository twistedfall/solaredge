use serde::Deserialize;

use crate::response::List;
use crate::response::site::Location;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
	pub id: u32,
	pub name: String,
	pub location: Location,
	#[serde(rename = "companyWebSite")]
	pub company_website: String,
	pub contact_person: String,
	pub email: String,
	pub phone_number: String,
	pub fax_number: String,
	pub notes: String,
	pub parent_id: u32,
	pub uris: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListTop {
	pub accounts: List<Account>,
}
