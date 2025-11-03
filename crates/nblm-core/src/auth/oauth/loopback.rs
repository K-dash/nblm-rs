use std::net::{SocketAddr, TcpListener};

use crate::error::{Error, Result};

const LOOPBACK_ADDR: &str = "127.0.0.1";

/// Listener bound to a loopback address with a concrete port.
pub struct LoopbackListener {
    listener: TcpListener,
    addr: SocketAddr,
}

impl LoopbackListener {
    /// Bind a loopback listener. If `preferred_port` is provided and available it will be used,
    /// otherwise the OS will allocate an available port.
    pub fn bind(preferred_port: Option<u16>) -> Result<Self> {
        if let Some(port) = preferred_port {
            if let Ok(listener) = TcpListener::bind((LOOPBACK_ADDR, port)) {
                return Self::finalize(listener);
            }
        }

        let listener = TcpListener::bind((LOOPBACK_ADDR, 0)).map_err(|err| {
            Error::TokenProvider(format!("failed to bind loopback listener: {err}"))
        })?;
        Self::finalize(listener)
    }

    fn finalize(listener: TcpListener) -> Result<Self> {
        listener
            .set_nonblocking(true)
            .map_err(|err| Error::TokenProvider(format!("failed to configure listener: {err}")))?;
        let addr = listener.local_addr().map_err(|err| {
            Error::TokenProvider(format!("failed to get listener address: {err}"))
        })?;
        Ok(Self { listener, addr })
    }

    /// Return the bound port.
    pub fn port(&self) -> u16 {
        self.addr.port()
    }

    /// Consume the listener and return the underlying std listener.
    pub fn into_std(self) -> TcpListener {
        self.listener
    }
}

/// Build a loopback redirect URI for the given port.
pub fn build_redirect_uri(port: u16) -> String {
    format!("http://{LOOPBACK_ADDR}:{port}")
}

/// Convenience wrapper around [LoopbackListener::bind].
pub fn bind_loopback_listener(preferred_port: Option<u16>) -> Result<LoopbackListener> {
    LoopbackListener::bind(preferred_port)
}
