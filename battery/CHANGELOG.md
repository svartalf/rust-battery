# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.1] - 2019-02-27
### Fixed
- Fix energy and remaining time calculations for MacOS [#8](https://github.com/svartalf/rust-battery/issues/8), [#11](https://github.com/svartalf/rust-battery/pull/11)
- Fix multiplication overflow while calculating battery percentage in Mac OS by [@mindriot101](https://github.com/mindriot101) [#10](https://github.com/svartalf/rust-battery/pull/10)
- Fix wrong units for consumption graph in `battery-cli`, should be `W` instead of `Wh` [#9](https://github.com/svartalf/rust-battery/issues/9)
- Fix non-uniform path import that was breaking compilation for Rust<1.32 [#6](https://github.com/svartalf/rust-battery/issues/6)
- Fix `time_to_empty` and `time_to_full` calculations for Linux when charger is unplugged but driver still reports zero `energy_rate` by [@kerhong](https://github.com/kerhong) [#5](https://github.com/svartalf/rust-battery/pull/5)
