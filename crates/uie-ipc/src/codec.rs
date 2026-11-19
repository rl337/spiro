use uie_core::error::{Error, Result};

use crate::message::IpcMessage;

/// Maximum frame size (64 MiB) to bound malicious clients.
pub const MAX_FRAME_LEN: usize = 64 * 1024 * 1024;

pub fn encode_frame(msg: &IpcMessage) -> Result<Vec<u8>> {
    let body = serde_json::to_vec(msg).map_err(|e| Error::Io(e.to_string()))?;
    if body.len() > MAX_FRAME_LEN {
        return Err(Error::Io("frame too large to encode".into()));
    }
    let mut frame = Vec::with_capacity(4 + body.len());
    frame.extend_from_slice(&(body.len() as u32).to_le_bytes());
    frame.extend_from_slice(&body);
    Ok(frame)
}

pub fn decode_frame(frame: &[u8]) -> Result<IpcMessage> {
    if frame.len() < 4 {
        return Err(Error::Io("frame shorter than length prefix".into()));
    }
    let len = u32::from_le_bytes(frame[..4].try_into().unwrap()) as usize;
    if len > MAX_FRAME_LEN {
        return Err(Error::Io("frame length exceeds limit".into()));
    }
    if frame.len() != 4 + len {
        return Err(Error::Io("frame length mismatch".into()));
    }
    serde_json::from_slice(&frame[4..]).map_err(|e| Error::Io(e.to_string()))
}
