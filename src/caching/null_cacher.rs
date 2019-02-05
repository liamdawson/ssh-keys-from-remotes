pub struct NullCacher {}

impl super::CacheService for NullCacher {
    fn cache(&self, _url: &str, _content: &str) -> std::io::Result<()> {
        eprintln!("warning: unable to write to cache");

        Ok(())
    }

    fn from_cache(&self, _url: &str) -> std::io::Result<String> {
        eprintln!("warning: unable to read from cache");

        Ok(String::from(""))
    }
}
