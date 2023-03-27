use hyper::{client::HttpConnector, Body};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

/// Constructs [hyper::Client] that supports both `http` and `https`.
/// Note that only `http1` is supported.
pub fn https_client() -> hyper::Client<HttpsConnector<HttpConnector>> {
    hyper::Client::builder().build::<_, Body>(
        HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build(),
    )
}
