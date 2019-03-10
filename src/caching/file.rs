use std::path::{Path, PathBuf};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::time::Duration;

/// Converts a key (which may be an invalid filename) to a deterministic,
/// filename-safe string
fn key_to_hash(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(key);
    hasher.result_str()
}

fn key_to_file_path(base: &Path, key: &str) -> PathBuf {
    base.join(key_to_hash(key))
}

pub fn has_valid_cache(base: &Path, key: &str, max_age: &Duration) -> bool {
  if let Ok(file) = std::fs::File::open(key_to_file_path(base, key)) {
    if let Ok(metadata) = file.metadata() {
      if let Ok(modified_time) = metadata.modified() {
        let lifetime = modified_time + *max_age;
        return std::time::SystemTime::now() <= lifetime;
      }
    }
  }

  false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn it_deterministically_hashes_key_names() {
        // if this value changes, it's a breaking version change
        assert_eq!("315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3", key_to_hash("Hello, world!"));
    }

    #[test]
    fn it_generates_the_expected_file_path() {
      let base_path = PathBuf::from("foo");

      assert_eq!(PathBuf::from("foo/315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3"),
        key_to_file_path(&base_path, "Hello, world!"));
    }
}
