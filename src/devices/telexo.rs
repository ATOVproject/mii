//! Commands for the BPC Telexo (TXo).
//! The application is responsible for calculating the final address by adding a device
//! index (0-7) to the `BASE_ADDRESS`.

use crate::{Command, SerializationError};

/// The base I2C address for TXo modules.
pub const BASE_ADDRESS: u8 = 0x60;

/// All supported II commands for the Telexo.
#[derive(Debug, Clone, Copy)]
pub enum Commands {
    /// `set gate`: Sets the state of a gate output.
    /// - `port`: 0-3
    /// - `state`: `true` for high (1), `false` for low (0).
    SetGate { port: u8, state: bool },
    /// `set CV`: Sets the CV for a given output port.
    /// - `port`: 0-3
    /// - `value`: A signed 16-bit integer representing voltage.
    SetCv { port: u8, value: i16 },
    /// `set CV slew`: Sets the slew time for a given CV output.
    /// - `port`: 0-3
    /// - `ms`: The slew time in milliseconds as an unsigned 16-bit integer.
    SetCvSlew { port: u8, ms: u16 },
    /// `set osc pitch`: Sets the oscillator pitch.
    /// - `port`: 0-3
    /// - `pitch`: A signed 16-bit integer pitch value.
    SetOscPitch { port: u8, pitch: i16 },
    /// `set osc waveform`: Sets the oscillator waveform.
    /// - `port`: 0-3
    /// - `waveform`: An unsigned 16-bit value (0-5000).
    SetOscWaveform { port: u8, waveform: u16 },
    /// `set envelope mode`: Enables or disables envelope mode for an output.
    /// - `port`: 0-3
    /// - `enabled`: `true` to enable, `false` to disable.
    SetEnvelopeMode { port: u8, enabled: bool },
    /// `set envelope`: Triggers the envelope on or off.
    /// - `port`: 0-3
    /// - `on`: `true` for on, `false` for off.
    SetEnvelopeState { port: u8, on: bool },
}

impl Command for Commands {
    const MAX_LENGTH: usize = 4; // All listed commands are 3 or 4 bytes long.

    fn to_bytes<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], SerializationError> {
        // A single check for the max required length simplifies the match arms.
        if buffer.len() < Self::MAX_LENGTH {
            return Err(SerializationError::BufferTooSmall);
        }

        match *self {
            Self::SetGate { port, state } => {
                buffer[0] = 0x00;
                buffer[1] = port;
                buffer[2] = state as u8;
                Ok(&buffer[..3])
            }
            Self::SetCv { port, value } => {
                let value_bytes = value.to_be_bytes();
                buffer[0] = 0x11;
                buffer[1] = port;
                buffer[2] = value_bytes[0];
                buffer[3] = value_bytes[1];
                Ok(&buffer[..4])
            }
            Self::SetCvSlew { port, ms } => {
                let ms_bytes = ms.to_be_bytes();
                buffer[0] = 0x12;
                buffer[1] = port;
                buffer[2] = ms_bytes[0];
                buffer[3] = ms_bytes[1];
                Ok(&buffer[..4])
            }
            Self::SetOscPitch { port, pitch } => {
                let pitch_bytes = pitch.to_be_bytes();
                buffer[0] = 0x41;
                buffer[1] = port;
                buffer[2] = pitch_bytes[0];
                buffer[3] = pitch_bytes[1];
                Ok(&buffer[..4])
            }
            Self::SetOscWaveform { port, waveform } => {
                let wf_bytes = waveform.to_be_bytes();
                buffer[0] = 0x4A;
                buffer[1] = port;
                buffer[2] = wf_bytes[0];
                buffer[3] = wf_bytes[1];
                Ok(&buffer[..4])
            }
            Self::SetEnvelopeMode { port, enabled } => {
                buffer[0] = 0x60;
                buffer[1] = port;
                buffer[2] = enabled as u8;
                Ok(&buffer[..3])
            }
            Self::SetEnvelopeState { port, on } => {
                buffer[0] = 0x6D;
                buffer[1] = port;
                buffer[2] = on as u8;
                Ok(&buffer[..3])
            }
        }
    }
}
