//! HTTP client adapter for [solaredge](https://crates.io/crates/solaredge) based on
//! [reqwest](https://crates.io/crates/reqwest)

use async_trait::async_trait;
use url::Url;

use solaredge::HttpClientAdapter;

/// For usage with `solaredge::Client`, see [solaredge](https://crates.io/crates/solaredge) crate
#[derive(Clone, Debug)]
pub struct ReqwestAdapter {
	client: reqwest::Client,
}

impl Default for ReqwestAdapter {
	#[inline]
	fn default() -> Self {
		Self {
			client: reqwest::Client::new(),
		}
	}
}

#[async_trait]
impl HttpClientAdapter for ReqwestAdapter {
	type Error = reqwest::Error;

	async fn get(&self, url: Url) -> Result<String, Self::Error> {
		let res = self.client.get(url).send().await?.error_for_status()?;
		Ok(res.text().await?)
	}
}
