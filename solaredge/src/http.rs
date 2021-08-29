use async_trait::async_trait;
use url::Url;

/// Adapter to allow different HTTP clients to be used with the library
#[async_trait]
pub trait HttpClientAdapter: Default {
	/// Error type used by the underlying HTTP library
	type Error;

	/// Fetch the specified URL using the GET method
	///
	/// Returns the text contents of the resource located at the indicated URL
	async fn get(&self, url: Url) -> Result<String, Self::Error>;
}
