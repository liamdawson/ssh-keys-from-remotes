use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn hash_url(url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(&url);
    hasher.result_str()
}

pub struct FilesystemCacher {
    destination_dir: PathBuf,
}

impl FilesystemCacher {
    pub fn new(destination_dir: PathBuf) -> Self {
        FilesystemCacher { destination_dir }
    }

    fn cache_file_path(&self, url: &str) -> PathBuf {
        self.destination_dir.join(format!("{}.keys", hash_url(url)))
    }
}

impl super::CacheService for FilesystemCacher {
    fn cache(&self, url: &str, content: &str) -> std::io::Result<()> {
        File::create(self.cache_file_path(url))?.write_all(content.as_bytes())
    }

    fn from_cache(&self, url: &str) -> std::io::Result<String> {
        let mut buf = String::new();

        File::open(self.cache_file_path(url))?
            .read_to_string(&mut buf)
            .map({ |_| buf })
    }
}
