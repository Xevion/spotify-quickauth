# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.7

### Added

- Archival notice.

### Changed

- Bumped MSRV to 1.79 to access stabilized `std::path::absolute`
- Adjusted `github-linguist` statistics, ignoring `.hooks`
- Widened `paths-ignore` for `build` and `test` workflows.
- Switched release step token to use `secrets.GH_RELEASE_TOKEN`, so the author is bound to `Xevion` instead of `github-actions` (a bot).
- Switched changelog format to `keep-a-changelog` style.

### Removed

- Removed `paths-ignore` for `build` workflow on `push` trigger.

## v0.1.6

### Added

- Began tracking changes in the `CHANGELOG.md` file.
- Added testing workflow to the repository, targeting only major x64 platforms.
- Added testing badge to README

### Changed

- [breaking] Lowered MSRV to 1.74 to match `librespot`'s `0.5.0-dev` branch.
- Fixed `pre-commit` hook to properly process the `CARGO_README.md` file.
