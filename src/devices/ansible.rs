//! Commands for the Monome Ansible module.

use crate::{Command, SerializationError};

/// The fixed I2C address for Ansible.
pub const ADDRESS: u8 = 0x20;

/// All supported II commands for Ansible.
#[derive(Debug, Clone, Copy)]
pub enum Commands {
    // --- CV Commands ---
    /// Sets the CV for a given output.
    /// - `port`: 0-3
    /// - `value`: A signed 16-bit integer representing voltage.
    SetCv { port: u8, value: i16 },
    /// Sets the slew time for a given CV output.
    /// - `port`: 0-3
    /// - `ms`: The slew time in milliseconds.
    SetCvSlew { port: u8, ms: u16 },
    /// A special mapping command, often used by grid/fader controllers.
    /// `device_port` is a calculated value, typically `(fader_index / 4) << 1`.
    /// - `device_port`: The calculated target device sub-address.
    /// - `value`: An unsigned 16-bit fader value.
    SetCvFromFader { device_port: u8, value: u16 },

    // --- Trigger Commands ---
    /// Sets the state of a trigger output.
    /// - `port`: 0-3
    /// - `state`: `true` for high (1), `false` for low (0).
    SetTrState { port: u8, state: bool },
    /// Toggles the state of a trigger output.
    /// - `port`: 0-3
    SetTrToggle { port: u8 },
    /// Sends a pulse to a trigger output.
    /// - `port`: 0-3
    SetTrPulse { port: u8 },
    /// Sets the pulse duration for a trigger output.
    /// - `port`: 0-3
    /// - `ms`: The pulse duration in milliseconds.
    SetTrPulseDuration { port: u8, ms: u16 },

    // --- App/Mode Commands ---
    /// Loads a preset.
    /// - `preset`: 0-7
    LoadPreset { preset: u8 },
    /// Saves the current state to a preset.
    /// - `preset`: 0-7
    SavePreset { preset: u8 },
    /// In Kria, sets the step value. Not applicable to other apps.
    /// - `track`: 1-4
    /// - `step`: 0-15
    /// - `state`: 0=off, 1=on, 2=toggle
    KriaSetStep { track: u8, step: u8, state: u8 },
}

impl Command for Commands {
    const MAX_LENGTH: usize = 4; // Most commands are 1-4 bytes.

    fn to_bytes<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], SerializationError> {
        // A single check for the max required length simplifies the match arms.
        if buffer.len() < Self::MAX_LENGTH {
            return Err(SerializationError::BufferTooSmall);
        }

        match *self {
            // --- CV ---
            Self::SetCv { port, value } => {
                let value_bytes = value.to_be_bytes();
                buffer[0] = 0x01;
                buffer[1] = port;
                buffer[2] = value_bytes[0];
                buffer[3] = value_bytes[1];
                Ok(&buffer[..4])
            }
            Self::SetCvSlew { port, ms } => {
                let ms_bytes = ms.to_be_bytes();
                buffer[0] = 0x02;
                buffer[1] = port;
                buffer[2] = ms_bytes[0];
                buffer[3] = ms_bytes[1];
                Ok(&buffer[..4])
            }
            Self::SetCvFromFader { device_port, value } => {
                let value_bytes = value.to_be_bytes();
                buffer[0] = 0x06;
                buffer[1] = device_port;
                buffer[2] = value_bytes[0];
                buffer[3] = value_bytes[1];
                Ok(&buffer[..4])
            }

            // --- Trigger ---
            Self::SetTrState { port, state } => {
                buffer[0] = 0x10;
                buffer[1] = port;
                buffer[2] = state as u8;
                Ok(&buffer[..3])
            }
            Self::SetTrToggle { port } => {
                buffer[0] = 0x11;
                buffer[1] = port;
                Ok(&buffer[..2])
            }
            Self::SetTrPulse { port } => {
                buffer[0] = 0x12;
                buffer[1] = port;
                Ok(&buffer[..2])
            }
            Self::SetTrPulseDuration { port, ms } => {
                let ms_bytes = ms.to_be_bytes();
                buffer[0] = 0x13;
                buffer[1] = port;
                buffer[2] = ms_bytes[0];
                buffer[3] = ms_bytes[1];
                Ok(&buffer[..4])
            }

            // --- App/Mode ---
            Self::LoadPreset { preset } => {
                buffer[0] = 0x20;
                buffer[1] = preset;
                Ok(&buffer[..2])
            }
            Self::SavePreset { preset } => {
                buffer[0] = 0x21;
                buffer[1] = preset;
                Ok(&buffer[..2])
            }
            Self::KriaSetStep { track, step, state } => {
                buffer[0] = 0x30;
                buffer[1] = track;
                buffer[2] = step;
                buffer[3] = state;
                Ok(&buffer[..4])
            }
        }
    }
}
