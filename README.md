# solaredge

[![Build Status](https://github.com/twistedfall/solaredge/actions/workflows/solaredge.yml/badge.svg)](https://github.com/twistedfall/solaredge/actions/workflows/solaredge.yml)
[![Documentation](https://docs.rs/solaredge/badge.svg)](https://docs.rs/solaredge)
[![Crates.io](https://img.shields.io/crates/v/solaredge)](https://crates.io/crates/solaredge)
![Maintenance](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)

[Support the project](https://github.com/sponsors/twistedfall) | [Documentation](https://docs.rs/solaredge)


## Usage

Run:
```shell
cargo add solaredge
```
Or add to your Cargo.toml:
```toml
[dependencies]
solaredge = "0.8.1"
```

## Asynchronous SolarEdge API client for Rust

Enables access to the SolarEdge equipment [API](https://www.solaredge.com/sites/default/files/se_monitoring_api.pdf)
(solar panels, inverters, meters) with the nice typed Rust interface.

The library requires an HTTP client but is client-agnostic. You can use any client that implements
[`HttpClientAdapter`](https://docs.rs/http-adapter/*/http_adapter/trait.HttpClientAdapter.html) trait.
Check [http-adapter-reqwest](https://crates.io/crates/http-adapter-reqwest) for an implementation based
on [reqwest](https://crates.io/crates/reqwest).

Sample usage with [http-adapter-reqwest](https://crates.io/crates/http-adapter-reqwest):
```rust
use solaredge::{Client, SitesList, SortOrder, FilterSiteStatus};
use http_adapter_reqwest::ReqwestAdapter;

async fn run() -> Result<(), Box<dyn std::error::Error>> {
   let client = Client::<ReqwestAdapter>::new("API_KEY");
   let version = client.version_current().await?;
   let mut p = SitesList::default();
   p.size = Some(32);
   p.sort_order = Some(SortOrder::Ascending);
   p.status = Some(&[FilterSiteStatus::Active, FilterSiteStatus::Pending]);
   let sites = client.sites_list(&p).await?;
   Ok(())
}
```

## License

LGPL-3.0
