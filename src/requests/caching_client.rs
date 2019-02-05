use super::UrlClient;
use crate::caching::CacheService;
use std::time::Duration;

const CACHE_CLEARING_STATUS_CODES: &[u16] = &[403, 404, 405, 406, 410, 451];

pub struct CachingClient {
    cache_service: Box<CacheService>,
}

fn try_fetch(url: &str, timeout: u64) -> reqwest::Result<reqwest::Response> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(timeout))
        .build()?;
    client.get(url).send()
}

fn should_retry(result: &reqwest::Result<reqwest::Response>) -> bool {
    !result.is_ok()
}

impl UrlClient for CachingClient {
    fn fetch(&self, url: &str) -> String {
        let result = retry::retry(
            2,
            500,
            || try_fetch(url, 2500),
            |result| !should_retry(result),
        );

        // retried until exhaustion
        if result.is_err() {
            return self.get(url);
        }

        // is_err is false for the first result, retry condition should
        // ensure Ok(_) for second
        let mut result = result.unwrap().unwrap();

        if result.status().is_success() {
            if let Ok(response) = result.text() {
                self.set(url, &response);
                response
            } else {
                self.get(url)
            }
        } else if CACHE_CLEARING_STATUS_CODES.contains(&result.status().as_u16()) {
            self.set(url, "");
            String::from("")
        } else {
            self.get(url)
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
