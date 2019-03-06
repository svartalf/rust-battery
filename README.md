# battery

[![Latest Version](https://img.shields.io/crates/v/battery.svg)](https://crates.io/crates/battery)
[![Latest Version](https://docs.rs/battery/badge.svg)](https://docs.rs/battery)
[![Build Status](https://travis-ci.org/svartalf/rust-battery.svg?branch=master)](https://travis-ci.org/svartalf/rust-battery)
[![dependency status](https://deps.rs/crate/battery/0.7.0/status.svg)](https://deps.rs/crate/battery/0.7.0)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)

Rust crate providing cross-platform information about batteries.

Gives access to a system independent battery state, capacity, charge and voltage values
recalculated as necessary to be returned [SI measurement units](https://www.bipm.org/en/measurement-units/).

## Supported platforms

* Linux 2.6.39+
* MacOS 10.10+
* Windows 7+
* FreeBSD
* DragonFlyBSD

# API stability

Until `1.0.0` version API might change in any moment, be careful.

## Examples

See full-featured `battery-cli` crate at [GitHub](https://github.com/svartalf/rust-battery/tree/master/battery-cli/)
or at [crates.io](https://crates.rs/crate/battery-cli).

For a simple example check `battery/examples/simple.rs` in the [repository](https://github.com/svartalf/rust-battery/blob/master/battery/examples/simple.rs)

## FFI

Experimental [battery-ffi](https://crates.io/crates/battery-ffi) crate
provides the FFI bindings to the `battery` crate, so it can be used with
another languages.

Check its [README](https://github.com/svartalf/rust-battery/tree/master/battery-ffi)
and [documentation](https://docs.rs/battery-ffi) for details.
