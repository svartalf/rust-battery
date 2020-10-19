# battery

[![Latest Version](https://img.shields.io/crates/v/battery.svg)](https://crates.io/crates/battery)
[![Latest Version](https://docs.rs/battery/badge.svg)](https://docs.rs/battery)
[![Build Status](https://github.com/svartalf/rust-battery/workflows/Continuous%20integration/badge.svg)](https://github.com/svartalf/rust-battery/actions?workflow=Continuous+integration)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.36+-yellow.svg)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)
[![Backers on Open Collective](https://opencollective.com/rust-battery/backers/badge.svg)](#backers)
[![Sponsors on Open Collective](https://opencollective.com/rust-battery/sponsors/badge.svg)](#sponsors)

> Rust crate providing cross-platform information about the notebook batteries.

## Table of contents

 * [Overview](#overview)
 * [Supported platforms](#supported-platforms)
 * [Install](#install)
 * [Examples](#examples)
 * [FFI bindings](#ffi-bindings)
 * [Users](#users)
 * [License](#license)
 * [Donations](#donations)
 * [Contributors](#contributors)
 * [Backers](#backers)
 * [Sponsors](#sponsors)

## Overview

`battery` provides a cross-platform unified API to a notebook batteries state.

Its main goal is to wrap the OS-specific interfaces, cover all the hacks and legacy cases
and get the batteries information (such as state of charge, energy rate, voltage and temperature)
as a typed values, recalculated as necessary to be returned as a [SI measurement units](https://www.bipm.org/en/measurement-units/).

## Supported platforms

* Linux 2.6.39+
* MacOS 10.10+
* iOS
* Windows 7+
* FreeBSD
* DragonFlyBSD

Do note that iOS implementation uses IOKit bindings, your application
might be automatically rejected by Apple based on that fact. Use it on your own risk.

## Install

As a prerequisite, `battery` crate requires at least Rustc version **1.36** or greater.

Add the following line into a `Cargo.toml`:

```toml
[dependencies]
battery = "0.7.7"
```

## Examples

```rust
fn main() -> Result<(), battery::Error> {
    let manager = battery::Manager::new()?;

    for (idx, maybe_battery) in manager.batteries()?.enumerate() {
        let battery = maybe_battery?;
        println!("Battery #{}:", idx);
        println!("Vendor: {:?}", battery.vendor());
        println!("Model: {:?}", battery.model());
        println!("State: {:?}", battery.state());
        println!("Time to full charge: {:?}", battery.time_to_full());
        println!("");
    }

    Ok(())
}
```

See the `battery/examples/` folder in the [repository](https://github.com/svartalf/rust-battery/blob/master/battery/examples/simple.rs)
for additional examples.

## FFI bindings

Experimental [battery-ffi](https://crates.io/crates/battery-ffi) crate provides the FFI bindings to the `battery` crate,
so it can be used with other languages, such as C, Python or NodeJS.

Check its [README](https://github.com/svartalf/rust-battery/tree/master/battery-ffi)
and [documentation](https://docs.rs/battery-ffi) for details.

## Users

This an incomplete list of the `battery` crate users. If you are using it too,
send me a message and I'll add your project here!

### battop

[`battop`](https://github.com/svartalf/rust-battop) is an interactive viewer,
similar to `top`, `htop` and other \*top utilities, but about the batteries installed in your notebook.\
It is using the `battery` crate API to show the batteries information in your terminal.

### starship

[`starship`](https://github.com/starship/starship) is a Rust port of the minimalistic, powerful,
and extremely customizable prompt Spaceship ZSH.\
It is using the `battery` crate to show the the current battery level and status in a shell prompt.

Here is what [@matchai](https://github.com/matchai) says:

> I really appreciate how easily we were able to get your library up and running!
> Battery APIs were a headache for us in predecessors of this project 😅

And there is [this tweet](https://twitter.com/matchai/status/1135906726392283136) also!

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
