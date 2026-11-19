use jiff::civil::{Date, DateTime};
use serde::de::Error as _;
use serde::ser::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_variant::to_variant_name;

pub mod enums;
pub mod request;
pub mod response;

fn serialize_comma_slice<T: Serialize, S: Serializer>(slice: &[T], ser: S) -> Result<S::Ok, S::Error> {
	let mut res = String::new();
	let mut first = true;
	for v in slice {
		if !first {
			res.push(',');
		}
		res.push_str(to_variant_name(v).map_err(|s| S::Error::custom(s.to_string()))?);
		if first {
			first = false;
		}
	}
	ser.serialize_str(&res)
}

fn serialize_comma_slice_opt<T: Serialize, S: Serializer>(slice: &Option<&[T]>, ser: S) -> Result<S::Ok, S::Error> {
	if let Some(slice) = slice {
		serialize_comma_slice(slice, ser)
	} else {
		ser.serialize_none()
	}
}

const DATE_FORMAT: &str = "%Y-%m-%d";
const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

fn str_to_datetime(s: &str) -> Result<DateTime, jiff::Error> {
	DateTime::strptime(DATETIME_FORMAT, s).or_else(|_| Date::strptime(DATE_FORMAT, s).map(|date| date.at(0, 0, 0, 0)))
}

struct DateTimeSerde;

impl DateTimeSerde {
	fn serialize<S: Serializer>(d: &DateTime, ser: S) -> Result<S::Ok, S::Error> {
		d.strftime(DATETIME_FORMAT).to_string().serialize(ser)
	}

	fn deserialize<'d, D: Deserializer<'d>>(d: D) -> Result<DateTime, D::Error> {
		let s = String::deserialize(d)?;
		str_to_datetime(&s).map_err(|e| D::Error::custom(format!("DateTime parse error, input: {s}, error: {e}")))
	}
}

struct DateTimeSerdeOpt;

impl DateTimeSerdeOpt {
	#[allow(unused)]
	fn serialize<S: Serializer>(d: &Option<DateTime>, ser: S) -> Result<S::Ok, S::Error> {
		if let Some(d) = d {
			DateTimeSerde::serialize(d, ser)
		} else {
			ser.serialize_none()
		}
	}

	fn deserialize<'d, D: Deserializer<'d>>(d: D) -> Result<Option<DateTime>, D::Error> {
		Ok(match Option::<String>::deserialize(d)? {
			None => None,
			Some(s) => {
				Some(str_to_datetime(&s).map_err(|e| D::Error::custom(format!("DateTime parse error, input: {s}, error: {e}")))?)
			}
		})
	}
}

struct DateSerde;

impl DateSerde {
	fn serialize<S: Serializer>(d: &Date, ser: S) -> Result<S::Ok, S::Error> {
		d.strftime(DATE_FORMAT).to_string().serialize(ser)
	}

	fn deserialize<'d, D: Deserializer<'d>>(d: D) -> Result<Date, D::Error> {
		let s = String::deserialize(d)?;
		Date::strptime(DATE_FORMAT, &s).map_err(|e| D::Error::custom(format!("Date parse error, input: {s}, error: {e}")))
	}
}
