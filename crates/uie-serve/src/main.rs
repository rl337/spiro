use std::env;

use std::path::PathBuf;
use uie_ipc::{InferenceEngineHost, UnixListenerChannel};

fn main() -> uie_core::Result<()> {
    tracing_subscriber::fmt::init();

    let socket = socket_path();
    let inj = uie_iv::serve_injector().map_err(|e| uie_core::Error::Runtime(e.to_string()))?;
    tracing::info!(
        profile = inj.acceptance_profile().id(),
        loaders = ?uie_iv::serve_loader_mode_from_env(),
        "uie-serve composition"
    );
    let host = InferenceEngineHost::new(inj.loaders());

    tracing::info!(socket = %socket.display(), "listening");
    let server = UnixListenerChannel::bind(&socket, host)?;
    server.serve_forever()
}

fn socket_path() -> PathBuf {
    if let Ok(p) = env::var("UIE_SOCKET") {
        return PathBuf::from(p);
    }
    // Per-process path under TMPDIR (not a shared world-writable default).
    let base = env::var("TMPDIR")
        .map(PathBuf::from)
        .or_else(|_| env::var("XDG_RUNTIME_DIR").map(PathBuf::from))
        .unwrap_or_else(|_| PathBuf::from("/tmp"));
    base.join(format!("uie-serve-{}.sock", std::process::id()))
}
