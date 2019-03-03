mod caching_client;
mod request;

pub use caching_client::CachingClient;
pub use request::{fetch, FetchResult};

pub trait UrlClient {
    fn fetch(&self, url: &str) -> String;
}
