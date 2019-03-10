mod fs_cacher;
mod null_cacher;
mod file;

pub use fs_cacher::FilesystemCacher;
pub use null_cacher::NullCacher;

pub trait CacheService {
    fn cache(&self, url: &str, content: &str) -> std::io::Result<()>;
    fn from_cache(&self, url: &str) -> std::io::Result<String>;
}
