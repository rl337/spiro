use std::env;
use std::path::PathBuf;

use uie_ipc::{InferenceEngineHost, UnixListenerChannel};
use uie_stub_models::StubModelLoader;
use uie_core::model::ModelLoader;

fn main() -> uie_core::Result<()> {
    tracing_subscriber::fmt::init();

    let socket = socket_path();
    let host =
        InferenceEngineHost::new(vec![Box::new(StubModelLoader::default()) as Box<dyn ModelLoader>]);

    tracing::info!(%socket, "listening");
    let server = UnixListenerChannel::bind(&socket, host)?;
    server.serve_forever()
}

fn socket_path() -> PathBuf {
    env::var("UIE_SOCKET")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp/uie.sock"))
}