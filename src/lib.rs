use std::{pin::Pin, task::Poll};

use tokio::{
    io::{AsyncRead, AsyncWrite},
    process::Child,
};

/// This is the service definition. It looks a lot like a trait definition.
/// It defines one RPC, hello, which takes one arg, name, and returns a String.
#[tarpc::service]
pub trait Plugin {
    async fn hello(name: String) -> String;
}

/// An instance of merged child process stdio used to implement `AsyncRead` and
/// `AsyncWrite`, as required by `serde_transport` for use as a transport for tarpc.
pub struct MergedChildIO {
    inner: Child,
}

impl MergedChildIO {
    pub fn new(child: Child) -> Self {
        MergedChildIO { inner: child }
    }
}

impl AsyncRead for MergedChildIO {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let stdout = self.inner.stdout.as_mut().expect("stdout");
        AsyncRead::poll_read(Pin::new(stdout), cx, buf)
    }
}

impl AsyncWrite for MergedChildIO {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        let stdin = self.inner.stdin.as_mut().expect("stdin");
        AsyncWrite::poll_write(Pin::new(stdin), cx, buf)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let stdin = self.inner.stdin.as_mut().expect("stdin");
        AsyncWrite::poll_flush(Pin::new(stdin), cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let stdin = self.inner.stdin.as_mut().expect("stdin");
        AsyncWrite::poll_shutdown(Pin::new(stdin), cx)
    }
}
