# battery-ffi

[![Latest Version](https://img.shields.io/crates/v/battery-ffi.svg)](https://crates.io/crates/battery-ffi)
[![Latest Version](https://docs.rs/battery-ffi/badge.svg)](https://docs.rs/battery-ffi)
[![Build Status](https://travis-ci.org/svartalf/rust-battery.svg?branch=master)](https://travis-ci.org/svartalf/rust-battery)
[![dependency status](https://deps.rs/crate/battery-ffi/0.7.1/status.svg)](https://deps.rs/crate/battery-ffi/0.7.1)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)
[![backers](https://opencollective.com/rust-battery/tiers/backer/badge.svg?label=backer&color=brightgreen)](https://opencollective.com/rust-battery)

This is a FFI bindings for [battery](https://github.com/svartalf/rust-battery/tree/master/battery)
library.

See the [documentation](https://docs.rs/battery-ffi) for available functions.

## Bindings generation

Among library creation this crate generates `battery_ffi.h` file,
which might be useful for automatic bindings generation or just with plain `C`/`C++`.

After build it will be located somewhere at `target/*/build/battery-ffi-*/out/`,
depending on build profile (`debug`/`release`) and build hash.

## Examples

`battery-ffi/examples/` folder in the [repository](https://github.com/svartalf/rust-battery)
contains examples for C and Python.

## Donations

If you appreciate my work and want to support me, you can do it [here](https://svartalf.info/donate/) or
support this project at [Open Collective](https://opencollective.com/rust-battery).
