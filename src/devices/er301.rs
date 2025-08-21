//! Commands for the Orthogonal Devices ER-301.

use crate::{Command, SerializationError};

/// The default I2C address for the first ER-301. Addresses can go up to `0x33`.
pub const ADDRESS: u8 = 0x31;

/// All supported II commands for the ER-301.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy)]
pub enum Commands {
    /// `set gate`: Sets the state of a gate output.
    /// - `port`: 0-99
    /// - `state`: `true` for high (1), `false` for low (0).
    SetGate { port: u8, state: bool },
    /// `set CV`: Sets the CV for a given output port.
    /// - `port`: 0-99
    /// - `value`: A signed 16-bit integer representing voltage.
    SetCv { port: u8, value: i16 },
    /// `set CV slew`: Sets the slew time for a given CV output.
    /// - `port`: 0-99
    /// - `ms`: The slew time in milliseconds as an unsigned 16-bit integer.
    SetCvSlew { port: u8, ms: u16 },
}

impl Command for Commands {
    const MAX_LENGTH: usize = 4; // SetCv and SetCvSlew are the longest.

    fn to_bytes<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], SerializationError> {
        match *self {
            Self::SetGate { port, state } => {
                if buffer.len() < 3 {
                    return Err(SerializationError::BufferTooSmall);
                }
                buffer[0] = 0x00;
                buffer[1] = port;
                buffer[2] = state as u8;
                Ok(&buffer[..3])
            }
            Self::SetCv { port, value } => {
                if buffer.len() < 4 {
                    return Err(SerializationError::BufferTooSmall);
                }
                let value_bytes = value.to_be_bytes(); // II protocol is big-endian
                buffer[0] = 0x11;
                buffer[1] = port;
                buffer[2] = value_bytes[0];
                buffer[3] = value_bytes[1];
                Ok(&buffer[..4])
            }
            Self::SetCvSlew { port, ms } => {
                if buffer.len() < 4 {
                    return Err(SerializationError::BufferTooSmall);
                }
                let ms_bytes = ms.to_be_bytes();
                buffer[0] = 0x12;
                buffer[1] = port;
                buffer[2] = ms_bytes[0];
                buffer[3] = ms_bytes[1];
                Ok(&buffer[..4])
            }
        }
    }
}
