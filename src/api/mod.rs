use chrono::{NaiveDate, NaiveDateTime, NaiveTime, ParseResult};
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

fn str_to_datetime(s: &str) -> ParseResult<NaiveDateTime> {
	match NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
		Ok(d) => Ok(d),
		Err(_) => {
			let date = NaiveDate::parse_from_str(s, "%Y-%m-%d")?;
			Ok(NaiveDateTime::new(
				date,
				NaiveTime::from_hms_opt(0, 0, 0).expect("Static time"),
			))
		}
	}
}

fn str_to_date(s: &str) -> ParseResult<NaiveDate> {
	NaiveDate::parse_from_str(s, "%Y-%m-%d")
}

struct DateTimeSerde;

impl DateTimeSerde {
	fn serialize<S: Serializer>(d: &NaiveDateTime, ser: S) -> Result<S::Ok, S::Error> {
		d.format("%Y-%m-%d %H:%M:%S").to_string().serialize(ser)
	}

	fn deserialize<'d, D: Deserializer<'d>>(d: D) -> Result<NaiveDateTime, D::Error> {
		let s = String::deserialize(d)?;
		str_to_datetime(&s).map_err(|e| D::Error::custom(format!("DateTime parse error, input: {s}, error: {e}")))
	}
}

struct DateTimeSerdeOpt;

impl DateTimeSerdeOpt {
	#[allow(unused)]
	fn serialize<S: Serializer>(d: &Option<NaiveDateTime>, ser: S) -> Result<S::Ok, S::Error> {
		if let Some(d) = d {
			DateTimeSerde::serialize(d, ser)
		} else {
			ser.serialize_none()
		}
	}

	fn deserialize<'d, D: Deserializer<'d>>(d: D) -> Result<Option<NaiveDateTime>, D::Error> {
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
	fn serialize<S: Serializer>(d: &NaiveDate, ser: S) -> Result<S::Ok, S::Error> {
		d.format("%Y-%m-%d").to_string().serialize(ser)
	}

	fn deserialize<'d, D: Deserializer<'d>>(d: D) -> Result<NaiveDate, D::Error> {
		let s = String::deserialize(d)?;
		str_to_date(&s).map_err(|e| D::Error::custom(format!("Date parse error, input: {s}, error: {e}")))
	}
}
