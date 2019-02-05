mod caching_client;

pub use caching_client::CachingClient;

pub trait UrlClient {
    fn fetch(&self, url: &str) -> String;
}
