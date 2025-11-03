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

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;

    #[test]
    fn test_bind_loopback_listener_without_preferred_port() {
        let listener = LoopbackListener::bind(None).unwrap();

        // Should bind to loopback address
        assert!(listener.addr.ip().is_loopback());

        // Should have a valid port assigned by OS
        let port = listener.port();
        assert!(port > 0);

        // Should be able to get the underlying listener
        let std_listener = listener.into_std();
        assert!(std_listener.local_addr().is_ok());
    }

    #[test]
    fn test_bind_loopback_listener_with_available_preferred_port() {
        // First, find an available port
        let temp_listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let available_port = temp_listener.local_addr().unwrap().port();
        drop(temp_listener); // Release the port

        // Now bind with the preferred port
        let listener = LoopbackListener::bind(Some(available_port)).unwrap();

        // Should use the preferred port
        assert_eq!(listener.port(), available_port);
        assert!(listener.addr.ip().is_loopback());
    }

    #[test]
    fn test_bind_loopback_listener_with_unavailable_preferred_port() {
        // Bind to a port to make it unavailable
        let _occupier = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let occupied_port = _occupier.local_addr().unwrap().port();

        // Try to bind with the occupied port - should fallback to OS-assigned port
        let listener = LoopbackListener::bind(Some(occupied_port)).unwrap();

        // Should get a different port from OS
        assert_ne!(listener.port(), occupied_port);
        assert!(listener.port() > 0);
    }

    #[test]
    fn test_loopback_listener_is_nonblocking() {
        let listener = LoopbackListener::bind(None).unwrap();
        let std_listener = listener.into_std();

        // Accept should return WouldBlock error immediately for non-blocking
        match std_listener.accept() {
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Expected behavior for non-blocking listener
            }
            _ => panic!("Expected WouldBlock error for non-blocking listener"),
        }

        drop(std_listener);
    }

    #[test]
    fn test_loopback_listener_can_accept_connections() {
        let listener = LoopbackListener::bind(None).unwrap();
        let port = listener.port();
        let std_listener = listener.into_std();

        // Set back to blocking for this test
        std_listener.set_nonblocking(false).unwrap();

        // Spawn a thread to connect
        let handle = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(10));
            TcpStream::connect(("127.0.0.1", port))
        });

        // Accept connection
        let result = std_listener.accept();
        assert!(result.is_ok());

        // Clean up
        handle.join().unwrap().unwrap();
    }

    #[test]
    fn test_build_redirect_uri() {
        let uri = build_redirect_uri(8080);
        assert_eq!(uri, "http://127.0.0.1:8080");

        let uri2 = build_redirect_uri(4317);
        assert_eq!(uri2, "http://127.0.0.1:4317");
    }

    #[test]
    fn test_bind_loopback_listener_convenience_function() {
        // Test the convenience wrapper function
        let listener = bind_loopback_listener(None).unwrap();
        assert!(listener.port() > 0);

        // Test with preferred port
        let temp_listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let available_port = temp_listener.local_addr().unwrap().port();
        drop(temp_listener);

        let listener = bind_loopback_listener(Some(available_port)).unwrap();
        assert_eq!(listener.port(), available_port);
    }
}
