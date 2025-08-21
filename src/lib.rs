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
//! use mii::{Command, er301};
//!
//! // Create a buffer. The longest ER-301 command is 4 bytes.
//! let mut buffer = [0u8; er301::Commands::MAX_LENGTH];
//!
//! // 1. Define the command you want to send.
//! let command = er301::Commands::SetCv { port: 5, value: 8192 };
//!
//! // 2. Serialize the command into the buffer.
//! let message: &[u8] = command.to_bytes(&mut buffer).unwrap();
//!
//! // 3. The `message` slice is now ready to be sent over I2C
//! //    to the device at `er301::ADDRESS`.
//! assert_eq!(message, &[0x11, 5, 0x20, 0x00]);
//! assert_eq!(er301::ADDRESS, 0x31);
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
