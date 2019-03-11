use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/*
## Notes for future:

* If introducing caching options that aren't filesystem based, convert to a
  struct that takes the `base: &Path` for a more consistent interface
*/

/// Converts a key (which may be an invalid filename) to a deterministic,
/// filename-safe string
fn hash_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(key);
    hasher.result_str()
}

fn path_for_key(key: &str, base: &Path) -> PathBuf {
    base.join(hash_key(key))
}

fn key_age(key: &str, base: &Path) -> std::io::Result<SystemTime> {
    File::open(path_for_key(key, base))?.metadata()?.modified()
}

fn appropriate_key_age(key_age: SystemTime, max_age_option: Option<Duration>) -> bool {
    max_age_option.map_or(true, |max_age| (SystemTime::now() - max_age) < key_age)
}

/// Indicates if the given key has a valid cache entry, with a maximum age of
/// `max_age_option` (if supplied).
///
/// Caveats: will fail if the file modified time metadata was inaccessible,
/// (even if `max_age_option` is `None`), or if the file is older than the
/// Unix epoch.
pub fn contains_key(key: &str, base: &Path, max_age_option: Option<Duration>) -> bool {
    if let Ok(age) = key_age(key, base) {
        appropriate_key_age(age, max_age_option)
    } else {
        false
    }
}

pub fn get(key: &str, base: &Path, max_age_option: Option<Duration>) -> std::io::Result<String> {
    let age = key_age(key, base)?;
    if appropriate_key_age(age, max_age_option) {
        std::fs::read_to_string(path_for_key(key, base))
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::NotFound))
    }
}

pub fn get_lossy(key: &str, base: &Path, max_age_option: Option<Duration>) -> String {
    if let Ok(result) = get(key, base, max_age_option) {
        result
    } else {
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn it_deterministically_hashes_key_names() {
        // if this value changes, it's a breaking version change
        assert_eq!(
            "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3",
            hash_key("Hello, world!")
        );
    }

    #[test]
    fn it_generates_the_expected_file_path() {
        let base_path = PathBuf::from("foo");

        assert_eq!(
            PathBuf::from("foo/315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3"),
            path_for_key("Hello, world!", &base_path)
        );
    }

    #[test]
    fn it_accepts_ages_less_than_max_age() {
        let two_hours = Some(Duration::from_secs(3600 * 2));
        let half_hour = Duration::from_secs(1800);

        assert!(appropriate_key_age(
            SystemTime::now() - half_hour,
            two_hours
        ));
    }

    #[test]
    fn it_accepts_any_age_when_no_max_age() {
        assert!(appropriate_key_age(SystemTime::UNIX_EPOCH, None));
    }

    #[test]
    fn it_rejects_ages_older_than_max_age() {
        let half_hour = Some(Duration::from_secs(1800));
        let two_hours = Duration::from_secs(3600 * 2);

        assert!(!appropriate_key_age(
            SystemTime::now() - two_hours,
            half_hour
        ));
    }
}
