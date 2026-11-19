#![cfg(unix)]

use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;

use uie_core::error::{Error, Result};

use crate::engine::InferenceEngineHost;
use crate::message::IpcMessage;
use crate::transport::{ByteChannel, FramedTransport};

pub struct UnixSocketChannel {
    stream: UnixStream,
}

impl UnixSocketChannel {
    pub fn connect(path: impl AsRef<Path>) -> Result<Self> {
        let stream = UnixStream::connect(path).map_err(|e| Error::Io(e.to_string()))?;
        Ok(Self { stream })
    }
}

impl ByteChannel for UnixSocketChannel {
    fn roundtrip(&mut self, request: &IpcMessage) -> Result<IpcMessage> {
        self.send_message(request)?;
        self.recv_message()
    }

    fn send_bytes(&mut self, frame: &[u8]) -> Result<()> {
        FramedTransport::write_frame(&mut self.stream, frame)
    }

    fn recv_bytes(&mut self) -> Result<Vec<u8>> {
        FramedTransport::read_frame(&mut self.stream)
    }
}

pub struct UnixListenerChannel {
    listener: UnixListener,
    host: InferenceEngineHost,
}

impl UnixListenerChannel {
    pub fn bind(path: impl AsRef<Path>, host: InferenceEngineHost) -> Result<Self> {
        let p = path.as_ref();
        if p.exists() {
            std::fs::remove_file(p).map_err(|e| Error::Io(e.to_string()))?;
        }
        let listener = UnixListener::bind(p).map_err(|e| Error::Io(e.to_string()))?;
        Ok(Self { listener, host })
    }

    pub fn serve_one(&self) -> Result<()> {
        let (mut stream, _) = self
            .listener
            .accept()
            .map_err(|e| Error::Io(e.to_string()))?;
        loop {
            let frame = match FramedTransport::read_frame(&mut stream) {
                Ok(f) => f,
                Err(_) => break,
            };
            let msg = crate::codec::decode_frame(&frame)?;
            let reply = self.host.handle_message(msg);
            let out = crate::codec::encode_frame(&reply)?;
            FramedTransport::write_frame(&mut stream, &out)?;
        }
        Ok(())
    }

    pub fn serve_forever(&self) -> Result<()> {
        loop {
            self.serve_one()?;
        }
    }
}