# solaredge

## Documentation

See [full documentation](https://docs.rs/solaredge)

## Usage

Add this to your Cargo.toml:
```
[dependencies]
solaredge = "0.5.1"
```

## Asynchronous SolarEdge API client for Rust

Work in progress, only basic API is implemented so far. See `todo` markers in
[client.rs](https://github.com/twistedfall/solaredge/blob/master/solaredge/src/client.rs) for
the specific missing functions.

Enables access the SolarEdge equipment [API](https://www.solaredge.com/sites/default/files/se_monitoring_api.pdf)
(solar panels, inverters, meters) with the nice typed Rust interface.

The library requires an HTTP client, but is client-agnostic. You can use any client that implements
[`HttpClientAdapter`](https://docs.rs/http-adapter/*/http_adapter/trait.HttpClientAdapter.html) trait.
Check [http-adapter-reqwest](https://crates.io/crates/http-adapter-reqwest) for an implementation based
on [reqwest](https://crates.io/crates/reqwest).

Sample usage with [http-adapter-reqwest](https://crates.io/crates/http-adapter-reqwest):
```rust
use solaredge::{Client, SitesList, SortOrder, SiteStatus};
use http_adapter_reqwest::ReqwestAdapter;

async fn run() -> Result<(), Box<dyn std::error::Error>> {
   let client = Client::<ReqwestAdapter>::new("API_KEY");
   let version = client.version_current().await?;
   let mut p = SitesList::default();
   p.size = Some(32);
   p.sort_order = Some(SortOrder::Ascending);
   p.status = Some(&[SiteStatus::Active, SiteStatus::Pending]);
   let sites = client.sites_list(&p).await?;
   Ok(())
}
```

License: LGPL-3.0
