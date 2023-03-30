//! # Asynchronous SolarEdge API client for Rust
//!
//! Work in progress, only basic API is implemented so far. See `todo` markers in
//! [client.rs](https://github.com/twistedfall/solaredge/blob/master/solaredge/src/client.rs) for
//! the specific missing functions.
//!
//! Enables access the SolarEdge equipment [API](https://www.solaredge.com/sites/default/files/se_monitoring_api.pdf)
//! (solar panels, inverters, meters) with the nice typed Rust interface.
//!
//! The library requires an HTTP client, but is client-agnostic. You can use any client that implements
//! [`HttpClientAdapter`](https://docs.rs/solaredge/*/solaredge/trait.HttpClientAdapter.html) interface.
//! Check [solaredge-reqwest](https://crates.io/crates/solaredge-reqwest) for an implementation based
//! on [reqwest](https://crates.io/crates/reqwest).
//!
//! Sample usage with [solaredge-reqwest](https://crates.io/crates/solaredge-reqwest):
//! ```
//! # // Dummy implementation for doctests only, do not use as reference, use `solaredge-reqwest` crate instead
//! # mod solaredge_reqwest {
//! #    #[derive(Default)]
//! #    pub struct ReqwestAdapter;
//! #    #[async_trait::async_trait]
//! #    impl solaredge::HttpClientAdapter for ReqwestAdapter {
//! #       type Error = String;
//! #       async fn get(&self, url: url::Url) -> Result<String, Self::Error> { Ok("".to_string()) }
//! #    }
//! # }
//! use solaredge::{Client, SitesList, SortOrder, SiteStatus};
//! use solaredge_reqwest::ReqwestAdapter;
//!
//! async fn run() -> Result<(), Box<dyn std::error::Error>> {
//!    let client = Client::<ReqwestAdapter>::new("API_KEY");
//!    let version = client.version_current().await?;
//!    let mut p = SitesList::default();
//!    p.size = Some(32);
//!    p.sort_order = Some(SortOrder::Ascending);
//!    p.status = Some(&[SiteStatus::Active, SiteStatus::Pending]);
//!    let sites = client.sites_list(&p).await?;
//!    Ok(())
//! }
//! ```

pub use api::enums::*;
pub use api::request::*;
pub use api::response;
pub use client::Client;
pub use error::Error;
pub use http::HttpClientAdapter;

pub mod api;
pub mod client;
mod error;
mod http;
