# Changelog
All notable changes to the crates in this workspace will be documented in this
file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

## [0.9.0]
### Added
- `flipperzero::{log, error, warn, info, debug, trace}` logging macros.
- `flipperzero::furi::log`, providing the `Level` and `LevelFilter` types for
  use with the logging macros.
- `flipperzero::furi::rng`, providing the `HwRng` type compatible with the
  `rand` crate.
- `flipperzero::furi::string`, providing the `FuriString` type that implements a
  `CString`-like string with a `String`-like API.
- In `flipperzero::furi::thread`:
  - `Builder, Thread, ThreadId, JoinHandle`
  - `spawn, current, yield_now`
- In `flipperzero::io`:
  - `Error::description`
  - `impl ufmt::uDisplay for Error`
- `flipperzero::notification`, providing constants for standard notification
  sequences, and APIs for running them.
- `flipperzero::toolbox`, providing wrapper types around several tools from the
  Flipper Zero SDK.
- `flipperzero-test` crate, providing a test harness for running tests directly
  on a Flipper Zero.

### Changed
- Migrated to SDK API 23.0 (firmware 0.82.3).
- MSRV is now 1.70.
- Bumped pinned nightly Rust version to `nightly-2023-05-03`.
- `flipperzero_rt::entry!` now waits for threads with the same app ID to finish.
  This prevents crashes when a `JoinHandle` is dropped instead of joined, and
  would outlive the main function.

## [0.8.0]
### Added
- `flipperzero::io`, providing `Read, Write, Seek` traits similar to the ones in
  `std::io`.
- `flipperzero::storage`, providing `OpenOptions` and `File` structs similar to
  the ones in `std::fs`.
- `impl ufmt::{uDebug, uDisplay} for flipperzero_sys::furi::Status`

### Changed
- Migrated to SDK API 20.0 (firmware 0.80.1).
- MSRV is now 1.64.
- Bumped pinned nightly Rust version to `nightly-2023-03-10`.
- `flipperzero::{print, println}` macros now use the `ufmt` crate, and are
  restricted to its supported argument syntax.

## [0.7.2]
### Fixed
- Removed unintended prefix from all bindings documentation.

## [0.7.1]
### Added
- `flipperzero::dolphin`, providing types for interacting with the dolphin.

### Changed
- `flipperzero_sys` bindings documentation is now transformed from Doxygen.

## [0.7.0]
### Changed
- Migrated to SDK API 14.0 (firmware 0.77.1).

## [0.6.0]
### Added
- `flipperzero/alloc` feature flag.
- `flipperzero::dialogs`, providing the `DialogsApp` and `DialogMessage` types
  for creating simple dialogs.
- `flipperzero::gui::canvas:Align` enum, used by `DialogMessage`.
- `flipperzero_sys::furi::UnsafeRecord`
- Inline functions required to use the Furi HAL GPIO interface.

### Changed
- Migrated to SDK API 11.2 (firmware 0.74.2).
- Macros now allow trailing commas.

## [0.5.0]
### Changed
- Migrated to SDK API 10.1 (firmware 0.73.1).

## [0.4.1]
### Added
- Custom icon support to `flipperzero_rt::manifest!` macro.

## [0.4.0]
### Changed
- Migrated to SDK API 7.5 (firmware 0.71.1).
- `flipperzero_rt::manifest::ManifestBase` fields `api_version_major` and
  `api_version_minor` have been combined into an `api_version` field.

## [0.3.1]
### Fixed
- `flipperzero-sys` bindings now use short enums as required.

## [0.3.0]
### Added
- `flipperzero-rt` crate, enabling standalone app binaries to be built.

### Changed
- Hand-written bindings are replaced by generated bindings from SDK API 2.2
  (firmware 0.69).

### Removed
- `flipperzero::panic_handler` (moved to `flipperzero_rt::panic_handler`).

## [0.2.0]
### Added
- `flipperzero::{print, println}` macros.
- `flipperzero::furi::{message_queue, sync}` modules.
- `flipperzero::furi::{Result, Error}` type aliases.
- `flipperzero-alloc` crate, providing global allocator support.
- `flipperzero-sys` crate has more hand-written bindings.

### Changed
- MSRV is now 1.64.
- `flipperzero::furi::Stdout` moved into `flipperzero::furi::io` module.
- `flipperzero::furi::sleep` moved into `flipperzero::furi::thread` module.

## [0.1.0]
Initial release!
