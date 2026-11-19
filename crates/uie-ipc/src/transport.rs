use uie_core::error::{Error, Result};

use crate::codec::{decode_frame, encode_frame, MAX_FRAME_LEN};
use crate::message::IpcMessage;

/// Pluggable byte conduit: same framing for in-process and Unix socket transports.
pub trait ByteChannel: Send {
    /// Send one framed request and receive one framed response.
    fn roundtrip(&mut self, request: &IpcMessage) -> Result<IpcMessage>;

    fn send_message(&mut self, msg: &IpcMessage) -> Result<()> {
        self.send_bytes(&encode_frame(msg)?)
    }

    fn recv_message(&mut self) -> Result<IpcMessage> {
        decode_frame(&self.recv_bytes()?)
    }

    fn send_bytes(&mut self, frame: &[u8]) -> Result<()>;

    fn recv_bytes(&mut self) -> Result<Vec<u8>>;
}

/// Helper for transports that already expose raw byte I/O.
pub struct FramedTransport;

impl FramedTransport {
    pub fn write_frame(w: &mut impl std::io::Write, frame: &[u8]) -> Result<()> {
        if frame.len() < 4 {
            return Err(Error::Io("invalid frame".into()));
        }
        let len = u32::from_le_bytes(frame[..4].try_into().unwrap()) as usize;
        if 4 + len != frame.len() || len > MAX_FRAME_LEN {
            return Err(Error::Io("invalid frame bounds".into()));
        }
        w.write_all(frame).map_err(|e| Error::Io(e.to_string()))
    }

    pub fn read_frame(r: &mut impl std::io::Read) -> Result<Vec<u8>> {
        let mut len_buf = [0u8; 4];
        r.read_exact(&mut len_buf)
            .map_err(|e| Error::Io(e.to_string()))?;
        let len = u32::from_le_bytes(len_buf) as usize;
        if len > MAX_FRAME_LEN {
            return Err(Error::Io("frame too large".into()));
        }
        let mut body = vec![0u8; len];
        r.read_exact(&mut body)
            .map_err(|e| Error::Io(e.to_string()))?;
        let mut frame = Vec::with_capacity(4 + len);
        frame.extend_from_slice(&len_buf);
        frame.extend_from_slice(&body);
        Ok(frame)
    }
}
