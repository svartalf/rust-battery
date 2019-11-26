# battery-ffi

[![Latest Version](https://img.shields.io/crates/v/battery-ffi.svg)](https://crates.io/crates/battery-ffi)
[![Latest Version](https://docs.rs/battery-ffi/badge.svg)](https://docs.rs/battery-ffi)
[![Build Status](https://travis-ci.org/svartalf/rust-battery.svg?branch=master)](https://travis-ci.org/svartalf/rust-battery)
[![dependency status](https://deps.rs/crate/battery-ffi/0.7.5/status.svg)](https://deps.rs/crate/battery-ffi/0.7.5)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.32+-yellow.svg)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)
[![backers](https://opencollective.com/rust-battery/tiers/backer/badge.svg?label=backer&color=brightgreen)](https://opencollective.com/rust-battery)
[![Sponsors on Open Collective](https://opencollective.com/rust-battery/sponsors/badge.svg)](#sponsors)

> Rust crate providing the FFI bindings for the [`battery`](https://github.com/svartalf/rust-battery/tree/master/battery) library

## Overview

This experimental library provides the [FFI](https://en.wikipedia.org/wiki/Foreign_function_interface)
for the `battery` crate, so it can be used with other languages, such as C, Python or NodeJS.

See the [documentation](https://docs.rs/battery-ffi) for available functions.

## Install

Clone the repository and run the following command in the `battery-ffi/` folder:

```bash
cargo build --release
```

This will generate the library file (`.so`, `.dylib` or `.dll` depending on your OS),
which can be found at the `./target/release` folder after the compilation.

In addition, compilation process will create the `battery_ffi.h` file,
which might be useful for automatic bindings generation
or just with plain C or C++ development.\
It will be located somewhere at `target/*/build/battery-ffi-*/out/`,
depending on the build profile (`debug`/`release`) and build hash.

## Examples

`battery-ffi/examples/` folder in the [repository](https://github.com/svartalf/rust-battery)
contains examples for C and Python.

## License

Licensed under either of [Apache License 2.0](https://github.com/svartalf/rust-battery/blob/master/LICENSE-APACHE)
or [MIT license](https://github.com/svartalf/rust-battery/blob/master/LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Donations

If you appreciate my work and want to support me, you can do it [here](https://svartalf.info/donate/) or
support this project at [Open Collective](https://opencollective.com/rust-battery).

## Contributors

This project exists thanks to all the people who contribute.
<a href="https://github.com/svartalf/rust-battery/graphs/contributors"><img src="https://opencollective.com/rust-battery/contributors.svg?width=890&button=false" /></a>

## Backers

Thank you to all our backers! 🙏 [[Become a backer](https://opencollective.com/rust-battery#backer)]

<a href="https://opencollective.com/rust-battery#backers" target="_blank"><img src="https://opencollective.com/rust-battery/backers.svg?width=890"></a>

## Sponsors

Support this project by becoming a sponsor. Your logo will show up here with a link to your website. [[Become a sponsor](https://opencollective.com/rust-battery#sponsor)]

<a href="https://opencollective.com/rust-battery/sponsor/0/website" target="_blank"><img src="https://opencollective.com/rust-battery/sponsor/0/avatar.svg"></a>
<a href="https://opencollective.com/rust-battery/sponsor/1/website" target="_blank"><img src="https://opencollective.com/rust-battery/sponsor/1/avatar.svg"></a>
<a href="https://opencollective.com/rust-battery/sponsor/2/website" target="_blank"><img src="https://opencollective.com/rust-battery/sponsor/2/avatar.svg"></a>
<a href="https://opencollective.com/rust-battery/sponsor/3/website" target="_blank"><img src="https://opencollective.com/rust-battery/sponsor/3/avatar.svg"></a>
<a href="https://opencollective.com/rust-battery/sponsor/4/website" target="_blank"><img src="https://opencollective.com/rust-battery/sponsor/4/avatar.svg"></a>
<a href="https://opencollective.com/rust-battery/sponsor/5/website" target="_blank"><img src="https://opencollective.com/rust-battery/sponsor/5/avatar.svg"></a>
<a href="https://opencollective.com/rust-battery/sponsor/6/website" target="_blank"><img src="https://opencollective.com/rust-battery/sponsor/6/avatar.svg"></a>
<a href="https://opencollective.com/rust-battery/sponsor/7/website" target="_blank"><img src="https://opencollective.com/rust-battery/sponsor/7/avatar.svg"></a>
<a href="https://opencollective.com/rust-battery/sponsor/8/website" target="_blank"><img src="https://opencollective.com/rust-battery/sponsor/8/avatar.svg"></a>
<a href="https://opencollective.com/rust-battery/sponsor/9/website" target="_blank"><img src="https://opencollective.com/rust-battery/sponsor/9/avatar.svg"></a>
