# Universal TunTap

[![codecov](https://codecov.io/gh/siegfried/utuntap/branch/master/graph/badge.svg)](https://codecov.io/gh/siegfried/utuntap)

A low level Rust library for Tun/Tap devices.

## Usage

It simply provides more options to open Tun/Tap device files.

```rust
use utuntap::tun;


let (mut file, filename) = tun::OpenOptions::new()
    .packet_info(false) // Only available on Linux
    .number(10)
    .open()
    .expect("failed to open device");
```

## Support Platforms

### Linux:

[![Build Status](https://travis-ci.org/siegfried/utuntap.svg?branch=master)](https://travis-ci.org/siegfried/utuntap)
