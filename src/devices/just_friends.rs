//! Commands for the Mannequins / Whimsical Raps Just Friends.

use crate::{Command, SerializationError};

/// The fixed I2C address for Just Friends.
pub const ADDRESS: u8 = 0x70;

/// All supported II commands for Just Friends.
#[derive(Debug, Clone, Copy)]
pub enum Commands {
    /// `set gate`: Sets the state of a gate output.
    /// - `output`: 1-6, or 0 for all.
    /// - `state`: `true` for high (1), `false` for low (0).
    SetGate { output: u8, state: bool },
    /// `play note`: Triggers a note on one or all outputs.
    /// - `output`: 1-6, or 0 for all.
    /// - `pitch`: Signed 16-bit note pitch value.
    /// - `volume`: Signed 16-bit velocity/volume value.
    PlayNote { output: u8, pitch: i16, volume: i16 },
}

impl Command for Commands {
    const MAX_LENGTH: usize = 6; // PlayNote is the longest command.

    fn to_bytes<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], SerializationError> {
        match *self {
            Self::SetGate { output, state } => {
                if buffer.len() < 3 {
                    return Err(SerializationError::BufferTooSmall);
                }
                buffer[0] = 0x01;
                buffer[1] = output;
                buffer[2] = state as u8;
                Ok(&buffer[..3])
            }
            Self::PlayNote {
                output,
                pitch,
                volume,
            } => {
                if buffer.len() < 6 {
                    return Err(SerializationError::BufferTooSmall);
                }
                let pitch_bytes = pitch.to_be_bytes();
                let volume_bytes = volume.to_be_bytes();
                buffer[0] = 0x08;
                buffer[1] = output;
                buffer[2] = pitch_bytes[0];
                buffer[3] = pitch_bytes[1];
                buffer[4] = volume_bytes[0];
                buffer[5] = volume_bytes[1];
                Ok(&buffer[..6])
            }
        }
    }
}
