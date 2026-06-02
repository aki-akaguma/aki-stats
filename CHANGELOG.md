# Changelog: aki-stats

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Code review results for 2026-06-01

### Changed
- Move existing code reviews to `docs/reviews/` and rename with dated filenames
- Simplify vector iteration in `run.rs` to use a `for` loop instead of `reverse()` and `pop()`
- Simplify argument handling in `main.rs` to use `std::env::args().skip(1)`

## [0.2.1] - 2026-05-20
### Added
- Clarification that input is UTF-8 only and line terminators are excluded from byte count

### Changed
- Consolidate multiple loops in `run_00` into a single pass
- Use `[u64; 128]` for `StatsAscii` instead of `Vec<u64>` to avoid heap allocation
- Update dependencies: `flood-tide` (0.2.14), `flood-tide-gen` (0.2.2), `runnel` (0.4.2)
- Increase minimum supported Rust version (MSRV) to 1.68.0

### Fixed
- Clippy warnings: `redundant_clone`, `needless_borrow`, `unnecessary_unwrap`

### Removed
- `memx-cdy` dependency

## [0.2.0] - 2025-09-15
### Added
- `specs` directory
- More tests

### Changed
- Implement `IntoIterator` compatibility for arguments in `execute()`
- Update dependencies: `runnel` (0.4.0), `rust-version-info-file` (0.2)
- Increase MSRV to 1.65.0
- Refactor `run.rs` and `lib.rs`

### Fixed
- Clippy warnings: `derivable_impls`, `useless_format`
- MSRV documentation

### Removed
- `base_dir=` from `-X` options

## [0.1.18] - 2024-06-19
### Added
- GitHub Actions workflows for Ubuntu, macOS, and Windows
- Test status badges in `README.tpl`
- Miri support for tests
- Tarpaulin support in `Makefile`

### Changed
- Rename `config` to `config.toml`
- Remove `cfg(has_not_matches)`
- Refactor `Makefile`
- Update dependencies: `flood-tide` (0.2.9), `flood-tide-gen` (0.1.20), `memx-cdy` (0.1.11), `runnel` (0.3.16), `exec-target` (0.2.8), `indoc` (2.0.5), `rust-version-info-file` (0.1.8)

### Fixed
- `LICENSE-APACHE` and `LICENSE-MIT` files
- Clippy warnings: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`, `uninlined_format_args`, `unused_imports`
- Correct Rust version from "1.56.0" to "1.60.0"

### Removed
- `COPYING` file

## [0.1.17] - 2023-01-11
### Added
- Badges in `README.tpl`
- MSRV (1.56.0) in `Cargo.toml`

### Changed
- Reformat `CHANGELOG.md`
- Update dependencies: `anyhow` (1.0.68), `flood-tide` (0.2.8), `flood-tide-gen` (0.1.19), `memx-cdy` (0.1.10), `runnel` (0.3.15), `num-format` (0.4.4)

### Fixed
- Clippy warnings: `Eq` implementation for `PartialEq`, `uninlined_format_args`

## [0.1.16] - 2022-06-18
### Changed
- Migrate to Rust 2021 edition
- Update dependencies: `flood-tide` (0.2.5), `memx` (0.1.21), `memx-cdy` (0.1.8), `runnel` (0.3.11), `exec-target` (0.2.6), `flood-tide-gen` (0.1.16), `rust-version-info-file` (0.1.6), `semver` (1.0.10)

## [0.1.15] - 2022-05-22
### Changed
- Update dependencies: `runnel` (0.3.10), `memx` (0.1.20), `anyhow` (1.0.57), `libc` (0.2.126), `regex` (1.5.6), `exec-target` (0.2.5), `rust-version-info-file` (0.1.5)

## [0.1.14] - 2021-11-15
### Added
- Documentation improvements

### Changed
- Increase MSRV to 1.47.0
- Update dependencies: `flood-tide` (0.2.4), `memx` (0.1.18), `memx-cdy` (0.1.7), `runnel` (0.3.9), `anyhow` (1.0.45), `libc` (0.2.107), `exec-target` (0.2.4), `flood-tide-gen` (0.1.15), `rust-version-info-file` (0.1.3)

## [0.1.13] - 2021-09-11
### Added
- Dependency: `indoc` (1.0.3)

### Changed
- Address Clippy warnings
- Update dependencies: `anyhow` (1.0.43), `flood-tide-gen` (0.1.14), `flood-tide` (0.2.3), `memx-cdy` (0.1.6), `runnel` (0.3.8), `exec-target` (0.2.3)
- Rewrite `TARGET_EXE_PATH` using `env!` macros

## [0.1.12] - 2021-06-24
### Added
- Initialization of `memx_cdy` for faster memory operations

### Changed
- Rewrite `TARGET_EXE_PATH` using `env!` macro

### Fixed
- Bug in `#[cfg(feature = "debian_build")]`

## [0.1.11] - 2021-06-05
### Added
- Command options: `--map-ascii` and `-X map-ascii-rust-src`

## [0.1.10] - 2021-06-03
### Added
- Support for `debian_build` feature

### Changed
- Update dependencies: `flood-tide` (0.2.2), `regex` (1.5.4)

### Fixed
- Bug in command option `-X rust-version-info`

## [0.1.9] - 2021-04-23
### Fixed
- Bug in `build.rs`

## [0.1.8] - 2021-04-23
### Added
- Command option: `-X`

### Changed
- Update dependencies: `flood-tide-gen` (0.1.12), `flood-tide` (0.2.1), `regex` (1.4.6)

## [0.1.7] - 2021-04-19
### Changed
- Update dependency: `flood-tide-gen` (0.1.10)

## [0.1.6] - 2021-04-07
### Changed
- Update dependencies: `flood-tide` (0.2), `anyhow` (1.0.40), `flood-tide-gen` (0.1.8), `runnel` (0.3.6)

## [0.1.5] - 2021-03-22
### Fixed
- Unnecessary dependency on `regex`

## [0.1.4] - 2021-03-14
### Changed
- Update `regex` to fix memory leak

## [0.1.3] - 2021-03-08
### Changed
- Update dependencies: `runnel`, `rustc_version` (0.3)

## [0.1.2] - 2021-03-08
### Changed
- Update dependency: `runnel`
- Rename `xtask/src/cmd.txt` to `xtask/src/aki-stats-cmd.txt`

## [0.1.1] - 2021-03-04
### Added
- Command options: `--locale <loc>`, `-?`, `--query <q>`

### Changed
- Format numeric output using locale-specific settings

## [0.1.0] - 2021-03-03
### Added
- Initial release

[Unreleased]: https://github.com/aki-akaguma/aki-stats/compare/v0.2.1..HEAD
[0.2.1]: https://github.com/aki-akaguma/aki-stats/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.18..v0.2.0
[0.1.18]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.17..v0.1.18
[0.1.17]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.16..v0.1.17
[0.1.16]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.15..v0.1.16
[0.1.15]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.14..v0.1.15
[0.1.14]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.13..v0.1.14
[0.1.13]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.12..v0.1.13
[0.1.12]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.11..v0.1.12
[0.1.11]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.10..v0.1.11
[0.1.10]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.9..v0.1.10
[0.1.9]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.8..v0.1.9
[0.1.8]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.7..v0.1.8
[0.1.7]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.6..v0.1.7
[0.1.6]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/aki-stats/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/aki-stats/releases/tag/v0.1.0
