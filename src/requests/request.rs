use retry::retry;
use reqwest::{Client, Result, Response};
use std::time::Duration;

const PERMANENT_ERROR_CODES: &[u16] = &[401, 403, 404, 405, 406, 410, 451];

#[derive(PartialEq, Clone, Debug)]
pub enum FetchResult {
    Success(String),
    TransientError,
    PermanentError
}

fn error_response_type(result: &Result<Response>) -> Option<FetchResult> {
    if let Ok(response) = result {
        if response.status().is_success() {
            return None;
        } else if PERMANENT_ERROR_CODES.contains(&response.status().as_u16()) {
            return Some(FetchResult::PermanentError);
        }
    }

    Some(FetchResult::TransientError)
}

fn try_fetch(url: &str, timeout: u64) -> Result<Response> {
    let client = Client::builder().timeout(Duration::from_millis(timeout)).build()?;
    client.get(url).send()
}

pub fn fetch(url: &str) -> FetchResult {
    let retry_result = retry(2, 500, || try_fetch(url, 2500), |res| error_response_type(res) != Some(FetchResult::TransientError));

    if retry_result.is_err() {
        return FetchResult::TransientError;
    }

    // retry condition should make the second unwrap safe
    let mut request_result = retry_result.unwrap().unwrap();

    if request_result.status().is_success() {
        if let Ok(response_text) = request_result.text() {
            return FetchResult::Success(response_text);
        }
    }

    // treat a decoding error as a permanent error, to be safe
    FetchResult::PermanentError
}

#[cfg(test)]
mod tests {
    use mockito;
    use super::{fetch, FetchResult};

    #[test]
    fn it_returns_body() {
        let test_string = "I'm a happy little vegemite!";

        let url = &mockito::server_url();
        let request_mock = mockito::mock("GET", "/").with_status(200).with_body(test_string).create();

        assert_eq!(FetchResult::Success(String::from(test_string)), fetch(url));

        request_mock.assert();
    }

    #[test]
    fn tries_twice_before_failing() {
        let url = &mockito::server_url();
        let request_mock = mockito::mock("GET", "/").with_status(500).expect(2).create();

        assert_eq!(FetchResult::TransientError, fetch(url));

        request_mock.assert();
    }
}