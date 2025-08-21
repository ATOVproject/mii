[![crates.io](https://img.shields.io/crates/d/mii.svg)](https://crates.io/crates/mii)
[![crates.io](https://img.shields.io/crates/v/mii.svg)](https://crates.io/crates/mii)
[![Documentation](https://docs.rs/mii/badge.svg)](https://docs.rs/mii)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.85+-blue.svg)

# `mii`

> A `no_std` Rust library for serializing monome ii protocol commands

A hardware-agnostic library that provides type-safe structures for ii-protocol commands used with various Eurorack modules. This crate focuses solely on command serialization into correct byte sequences - it does not handle I2C communication itself.

## Features

- **`no_std` compatible** - Perfect for embedded environments
- **Type-safe command structures** - Compile-time guarantees for valid commands
- **Hardware agnostic** - Bring your own I2C implementation
- **Multiple device support** - Covers popular Eurorack modules
- **Zero-copy serialization** - Efficient buffer-based approach

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
mii = "1"
```

### Basic Usage

```rust
use mii::{Command, er301};

// Create a buffer sized for the longest possible command
let mut buffer = [0u8; er301::Commands::MAX_LENGTH];

// Create a command
let command = er301::Commands::SetCv { port: 5, value: 8192 };

// Serialize to bytes
let message: &[u8] = command.to_bytes(&mut buffer).unwrap();

// Send over I2C to the device
// send_i2c(er301::ADDRESS, message);
```

## Supported Devices

| Device | Module | I2C Address | Commands |
|--------|--------|-------------|----------|
| **Ansible** | Monome Ansible | `0x20` | CV, trigger, slew, presets, Kria step control |
| **ER-301** | Orthogonal Devices ER-301 | `0x31` | Gate, CV, CV slew |
| **Just Friends** | Mannequins Just Friends | `0x70` | Gate, note playback |
| **Telexo (TXo)** | BPC Telexo | `0x60` + device index | Gate, CV, oscillator, envelope |

### Device Examples

#### Ansible
```rust
use mii::{Command, ansible};

let mut buffer = [0u8; ansible::Commands::MAX_LENGTH];

// Set CV output
let cv_cmd = ansible::Commands::SetCv { port: 0, value: 4096 };
let message = cv_cmd.to_bytes(&mut buffer).unwrap();

// Trigger pulse
let pulse_cmd = ansible::Commands::SetTrPulse { port: 1 };
let message = pulse_cmd.to_bytes(&mut buffer).unwrap();
```

#### Just Friends
```rust
use mii::{Command, just_friends};

let mut buffer = [0u8; just_friends::Commands::MAX_LENGTH];

// Play a note
let note_cmd = just_friends::Commands::PlayNote { 
    output: 1, 
    pitch: 1000, 
    volume: 8000 
};
let message = note_cmd.to_bytes(&mut buffer).unwrap();
```

#### Telexo
```rust
use mii::{Command, telexo};

let mut buffer = [0u8; telexo::Commands::MAX_LENGTH];

// Set oscillator pitch
let osc_cmd = telexo::Commands::SetOscPitch { port: 0, pitch: 2048 };
let message = osc_cmd.to_bytes(&mut buffer).unwrap();

// Note: Telexo uses BASE_ADDRESS + device index (0-7)
// Final address = telexo::BASE_ADDRESS + device_index
```

## [API Reference]

[API Reference]: https://docs.rs/mii

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.85 and up. It *might* compile with older versions but that may change in any new patch release.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of Conduct][CoC], the maintainer of this crate promises to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
