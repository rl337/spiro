//! Byte-channel API for driving [`uie_core::InferenceRuntime`] from other processes or languages.

mod codec;
mod engine;
mod message;
mod transport;

#[cfg(unix)]
mod unix;

mod in_process;

pub use codec::{decode_frame, encode_frame};
pub use engine::InferenceEngineHost;
pub use in_process::InProcessChannel;
pub use message::{InferRequest, InferResponse, IpcMessage, TensorPayload, PROTOCOL_VERSION};
pub use transport::{ByteChannel, FramedTransport};

#[cfg(unix)]
pub use unix::{UnixListenerChannel, UnixSocketChannel};