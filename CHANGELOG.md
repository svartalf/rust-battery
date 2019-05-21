# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.2] - 2019-05-21
### Fixed
- `Battery::state_of_health` and `Battery::state_of_charge` are always returning values in `0.0 ≤ x ≤ 1.0` interval

## [0.7.1] - 2019-03-31
### Changed
- `uom`, `core-foundation` and `libc` dependencies were updated to latest versions
- Zero cycles count is considered as non-existing value for Linux [#23](https://github.com/svartalf/rust-battery/issues/23)
### Removed
- `battery-cli` crate was yanked and replaced with `battop` crate (https://crates.io/crates/battop)

## [0.7.0] - 2019-03-10
### Changed
- Propagate all errors happened from `battery` and `battery-ffi` crates to the caller
- Return SI measurement units from `uom` crate for almost all public `Battery` methods
- Re-export used `uom` quantities and measurement units in public `battery::units` module
- Rename `Battery::percentage` method into `Battery::state_of_charge`
- Rename `Battery::capacity` method into `Battery::state_of_health`
- Mark `battery::State` and `battery::Technology` enums as a non-exhaustive
- Support multiple devices for FreeBSD and DragonFlyBSD [#17](https://github.com/svartalf/rust-battery/issues/17)
- Ignore devices with `scope` attributes different from `System` for Linux [#18](https://github.com/svartalf/rust-battery/issues/18)
- Update outdated `mach` dependency for Mac OS

## [0.6.2] - 2019-02-28
### Changed
- Replace looks-to-be-abandoned `CoreFoundation-sys` and `IOKit-sys` dependencies [#2](https://github.com/svartalf/rust-battery/issues/2)
### Fixed
- Free hanging mach port used for communication with Mac OS IOKit

## [0.6.1] - 2019-02-27
### Fixed
- Fix energy and remaining time calculations for MacOS [#8](https://github.com/svartalf/rust-battery/issues/8), [#11](https://github.com/svartalf/rust-battery/pull/11)
- Fix multiplication overflow while calculating battery percentage in Mac OS by [@mindriot101](https://github.com/mindriot101) [#10](https://github.com/svartalf/rust-battery/pull/10)
- Fix wrong units for consumption graph in `battery-cli`, should be `W` instead of `Wh` [#9](https://github.com/svartalf/rust-battery/issues/9)
- Fix non-uniform path import that was breaking compilation for Rust<1.32 [#6](https://github.com/svartalf/rust-battery/issues/6)
- Fix `time_to_empty` and `time_to_full` calculations for Linux when charger is unplugged but driver still reports zero `energy_rate` by [@kerhong](https://github.com/kerhong) [#5](https://github.com/svartalf/rust-battery/pull/5)
