#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
extern crate config;
extern crate crypto;

mod app_settings;
mod caching;
mod requests;

use crate::caching::{CacheService, FilesystemCacher, NullCacher};
use crate::requests::UrlClient;
use clap::App;

fn main() {
    if let Err(err) = std::panic::catch_unwind(inner_main) {
        eprintln!("application panicked: {:?}", err);
    }
}

fn inner_main() {
    let app = app_from_crate!();
    let matches = app
        .subcommand(
            App::new("fetch")
                .about("Fetch remote keys for user")
                .args_from_usage("<USERNAME> 'target username'"),
        )
        .get_matches();

    match matches.subcommand() {
        ("fetch", Some(submatches)) => {
            // no point continuing if config or username won't load
            let config = app_settings::config_from_default_location().unwrap();
            let username = submatches.value_of("USERNAME").unwrap();

            // handle the cache dir being inconstructable
            let cacher: Box<CacheService> = match app_settings::default_cache_location() {
                Ok(path) => Box::new(FilesystemCacher::new(path)),
                Err(err) => {
                    eprintln!("Could not instantiate cache: {}", err);
                    Box::new(NullCacher {})
                }
            };

            if config.users.contains_key(username) {
                println!("{}", fetch_keys(&config.users[username].urls, cacher));
            }
        }
        _ => unreachable!(),
    };
}

fn fetch_keys(urls: &[String], cacher: Box<CacheService>) -> String {
    let client = requests::CachingClient::new(cacher);

    urls.iter()
        .map({ |url| client.fetch(url) })
        .collect::<Vec<_>>()
        .join("\n")
}
