//! A `no-std`, hardware-agnostic library for serializing Monome II protocol commands.
//!
//! This crate provides type-safe structures for II-protocol commands for various
//! Eurorack modules. Its sole purpose is to serialize these high-level commands
//! into the correct byte sequences. It does not handle I2C communication itself.
//!
//! ## Usage
//!
//! 1.  Choose a module from the library, like `er301`.
//! 2.  Instantiate a command enum, e.g., `er301::Commands::SetCv { ... }`.
//! 3.  Create a buffer to hold the serialized message. The `Command::MAX_LENGTH`
//!     associated constant can help you size this appropriately.
//! 4.  Call the `.to_bytes()` method on your command object.
//! 5.  If successful, you get a byte slice ready to be sent over I2C.
//!
//! ### Example
//!
//! ```
//! use mii::{Command, devices::ansible};
//!
//! let mut buffer = [0u8; ansible::Commands::MAX_LENGTH];
//!
//! // Set CV output
//! let cv_cmd = ansible::Commands::SetCv { port: 0, value: 4096 };
//! let message = cv_cmd.to_bytes(&mut buffer).unwrap();
//! // Send message over I2C to ansible::ADDRESS (0x20)
//!
//! // Trigger pulse
//! let pulse_cmd = ansible::Commands::SetTrPulse { port: 1 };
//! let message = pulse_cmd.to_bytes(&mut buffer).unwrap();
//! // Send message over I2C to ansible::ADDRESS (0x20)
//! ```

#![cfg_attr(not(test), no_std)]

pub mod devices;

/// Represents errors that can occur during command serialization.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SerializationError {
    /// The provided buffer was too small to serialize the command.
    BufferTooSmall,
}

/// The core trait for any object that can be serialized into an II-compatible byte message.
pub trait Command {
    /// The maximum number of bytes this command could possibly serialize to.
    /// This helps the user create a buffer of the correct size.
    const MAX_LENGTH: usize;

    /// Serializes the command into the provided byte buffer.
    ///
    /// On success, it returns a slice of the buffer containing only the written bytes.
    fn to_bytes<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], SerializationError>;
}

#[cfg(test)]
mod tests {}
