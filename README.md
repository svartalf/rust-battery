# battery

[![Latest Version](https://img.shields.io/crates/v/battery.svg)](https://crates.io/crates/battery)
[![Latest Version](https://docs.rs/battery/badge.svg)](https://docs.rs/battery)
[![Build Status](https://travis-ci.org/svartalf/rust-battery.svg?branch=master)](https://travis-ci.org/svartalf/rust-battery)
[![dependency status](https://deps.rs/crate/battery/0.4.0/status.svg)](https://deps.rs/crate/battery/0.4.0)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)

Rust crate providing cross-platform information about batteries.

Gives access to a system independent battery state, capacity, charge and voltage values
recalculated as necessary to be returned in `mW`, `mWh` or `mV` units.

## Supported platforms

* Linux 2.6.39+
* MacOS (10.10+ probably, needs to be confirmed)
* Windows 7+

# API stability

Until `1.0.0` version API might change in any moment, be careful.

## Example

This crate acts both as library and as binary executable, so you can install and check it:

1. Install `battery` crate:

```bash
$ cargo install battery
```

2. Call the installed file:

```bash
$ ~/.cargo/bin/battery
Device:                 0
vendor:                 DP
model:                  bq20z451
S/N:                    N/A
battery
  state:                full
  energy:               35.32 Wh
  energy-full:          36.21 Wh
  energy-full-design:   54.34 Wh
  energy-rate:          0.00 Wh
  voltage:              8.44 V
  percentage:           65.00%
  temperature:          36.60 °C
  technology:           lithium-ion
```

I guess I'll need to replace my battery soon 😩
