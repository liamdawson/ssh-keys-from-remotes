use super::UrlClient;
use crate::caching::CacheService;
use std::time::Duration;
use super::FetchResult;

pub struct CachingClient {
    cache_service: Box<CacheService>,
}

impl UrlClient for CachingClient {
    fn fetch(&self, url: &str) -> String {
        match super::fetch(url) {
            FetchResult::PermanentError => {
                self.set(url, "");
                String::from("")
            },
            FetchResult::TransientError => {
                self.get(url)
            },
            FetchResult::Success(body) => {
                self.set(url, &body);
                body
            }
        }
    }
}

impl CachingClient {
    pub fn new(cache_service: Box<CacheService>) -> Self {
        Self { cache_service }
    }

    fn get(&self, url: &str) -> String {
        (*self.cache_service)
            .from_cache(url)
            .unwrap_or_else(|_| String::from(""))
    }

    fn set(&self, url: &str, value: &str) {
        // we won't act any differently if caching failed, so discard
        // the issue
        (*self.cache_service).cache(url, value).is_ok();
    }
}
