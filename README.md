# battery

[![Latest Version](https://img.shields.io/crates/v/battery.svg)](https://crates.io/crates/battery)
[![Latest Version](https://docs.rs/battery/badge.svg)](https://docs.rs/battery)
[![dependency status](https://deps.rs/crate/battery/0.3.1/status.svg)](https://deps.rs/crate/battery/0.3.1)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)

[crate-image]: https://img.shields.io/crates/v/cargo-audit.svg
[crate-link]: https://crates.io/crates/cargo-audit
[build-image]: https://travis-ci.org/RustSec/cargo-audit.svg?branch=master
[build-link]: https://travis-ci.org/RustSec/cargo-audit
[appveyor-image]: https://ci.appveyor.com/api/projects/status/oa39c0in9qkxpoiv?svg=true
[appveyor-link]: https://ci.appveyor.com/project/tarcieri/cargo-audit
[license-image]: 

Rust crate providing cross-platform information about batteries.

Gives access to a system independent battery state, capacity, charge and voltage values
recalculated as necessary to be returned in `W`, `Wh` or `V` units.

## Supported platforms

* Linux 2.6.39+
* MacOS (10.10+ probably, needs to be confirmed)

