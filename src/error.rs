use std::error::Error;
use std::fmt;
use std::fmt::Display;

use crate::Frame;

/// This enum contains all error messages this library can return. Most API functions will generally return a `Result<(),GGRSError>`.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum GGRSError {
    /// When the prediction threshold has been reached, we cannot accept more inputs from the local player.
    PredictionThreshold,
    /// You made an invalid request, usually by using wrong parameters for function calls.
    InvalidRequest { info: String },
    /// In a `SyncTestSession`, this error is returned if checksums of resimulated frames do not match up with the original checksum.
    MismatchedChecksum { frame: Frame },
    /// A problem occured during creation of the UDP socket.
    SocketCreationFailed,
    /// The Session is not synchronized yet. Please start the session and wait a few ms to let the clients synchronize.
    NotSynchronized,
    /// The player you are trying to disconnect is already disconnected.
    PlayerDisconnected,
    /// The spectator got so far behind the host that catching up is impossible.
    SpectatorTooFarBehind,
    /// Something went wrong while decoding received input data.
    DecodingError,
}

impl Display for GGRSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GGRSError::PredictionThreshold => {
                write!(
                    f,
                    "Prediction threshold is reached, cannot proceed without catching up."
                )
            }
            GGRSError::InvalidRequest { info } => {
                write!(f, "Invalid Request: {}", info)
            }
            GGRSError::NotSynchronized => {
                write!(
                    f,
                    "The session is not yet synchronized with all remote sessions."
                )
            }
            GGRSError::MismatchedChecksum { frame } => {
                write!(
                    f,
                    "Detected checksum mismatch during rollback on frame {}.",
                    frame
                )
            }
            GGRSError::SocketCreationFailed => {
                write!(f, "UPD Socket creation failed.")
            }
            GGRSError::PlayerDisconnected => {
                write!(
                    f,
                    "The player you are trying to disconnect is already disconnected."
                )
            }
            GGRSError::SpectatorTooFarBehind => {
                write!(
                    f,
                    "The spectator got so far behind the host that catching up is impossible."
                )
            }
            &GGRSError::DecodingError => {
                write!(
                    f,
                    "Something went wrong while decoding received input data."
                )
            }
        }
    }
}

impl Error for GGRSError {}
