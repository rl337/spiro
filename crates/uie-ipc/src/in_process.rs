use uie_core::error::Result;

use crate::engine::InferenceEngineHost;
use crate::message::IpcMessage;
use crate::transport::ByteChannel;

/// Same framing as socket IPC, but handled in memory (tests, future PyO3 embed).
pub struct InProcessChannel {
    host: InferenceEngineHost,
}

impl InProcessChannel {
    pub fn new(host: InferenceEngineHost) -> Self {
        Self { host }
    }

    pub fn host(&self) -> &InferenceEngineHost {
        &self.host
    }
}

impl ByteChannel for InProcessChannel {
    fn roundtrip(&mut self, request: &IpcMessage) -> Result<IpcMessage> {
        Ok(self.host.handle_message(request.clone()))
    }

    fn send_bytes(&mut self, _frame: &[u8]) -> Result<()> {
        Err(uie_core::Error::Io(
            "in-process channel uses roundtrip(), not send_bytes".into(),
        ))
    }

    fn recv_bytes(&mut self) -> Result<Vec<u8>> {
        Err(uie_core::Error::Io(
            "in-process channel uses roundtrip(), not recv_bytes".into(),
        ))
    }
}