use std::fmt;

use http_adapter::http;

#[derive(Debug)]
pub enum Error<E> {
	UrlParse(url::ParseError),
	UrlEncode(serde_urlencoded::ser::Error),
	HttpRequest(E),
	Json(serde_json::Error),
	Api(http::StatusCode, Vec<u8>),
}

impl<E: fmt::Display> fmt::Display for Error<E> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::UrlParse(e) => {
				write!(f, "URL parse error: {e}")
			}
			Error::UrlEncode(e) => {
				write!(f, "Parameter encoding error: {e}")
			}
			Error::HttpRequest(e) => {
				write!(f, "HTTP request error: {e}")
			}
			Error::Json(e) => {
				write!(f, "JSON error: {e}")
			}
			Error::Api(status, _) => {
				write!(f, "Solaredge HTTP API error: {status}")
			}
		}
	}
}

impl<E: fmt::Debug + fmt::Display> std::error::Error for Error<E> {}

impl<E> From<url::ParseError> for Error<E> {
	fn from(s: url::ParseError) -> Self {
		Self::UrlParse(s)
	}
}

impl<E> From<serde_urlencoded::ser::Error> for Error<E> {
	fn from(s: serde_urlencoded::ser::Error) -> Self {
		Self::UrlEncode(s)
	}
}

impl<E> From<serde_json::Error> for Error<E> {
	fn from(s: serde_json::Error) -> Self {
		Self::Json(s)
	}
}
