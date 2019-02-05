# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 - 2019-02-10
### Added
- Users can now have keys fetched via `/etc/ssh-keys-remotes.toml`
- Keys will be cached (by url) on successful fetch, and cleared on certain unsuccessful outcomes.
