use std::{ffi::OsStr, path::PathBuf, pin::Pin, task::Poll};

use async_std::process::{Command, Stdio};
use tarpc::{serde_transport, tokio_serde::formats::Bincode};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::{
    codec::LengthDelimitedCodec,
    compat::{FuturesAsyncReadCompatExt, FuturesAsyncWriteCompatExt},
};

#[tarpc::service]
pub trait Plugin {
    async fn current_dir() -> PathBuf;
    async fn output(output: String);
}

impl PluginClient {
    /// Initialize an RPC client to communicate with the plugin server
    pub fn init() -> Self {
        let merged_io = MergedProcessIO::new();

        let codec_builder = LengthDelimitedCodec::builder();
        let framed = codec_builder.new_framed(merged_io);
        let transport = serde_transport::new(framed, Bincode::default());
        Self::new(Default::default(), transport).spawn()
    }
}

/// An instance of merged child process stdio used to implement `AsyncRead` and
/// `AsyncWrite`, as required by `serde_transport` for use as a transport for tarpc.
struct MergedProcessIO {
    stdout: tokio_util::compat::Compat<async_std::io::Stdout>,
    stdin: tokio_util::compat::Compat<async_std::io::Stdin>,
}

impl MergedProcessIO {
    fn new() -> Self {
        MergedProcessIO {
            stdout: async_std::io::stdout().compat_write(),
            stdin: async_std::io::stdin().compat(),
        }
    }
}

impl AsyncRead for MergedProcessIO {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        Pin::new(&mut self.stdin).poll_read(cx, buf)
    }
}

impl AsyncWrite for MergedProcessIO {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        Pin::new(&mut self.stdout).poll_write(cx, buf)
    }

    #[inline]
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.stdout).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.stdout).poll_shutdown(cx)
    }
}

/// Merged stdin and stdout of a child process to provide `AsyncRead` and
/// `AsyncWrite`, as required by `serde_transport` for use as a transport for tarpc.
pub struct MergedChildIO {
    stdout: tokio_util::compat::Compat<async_std::process::ChildStdout>,
    stdin: tokio_util::compat::Compat<async_std::process::ChildStdin>,
}

impl MergedChildIO {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        let command = Command::new(program)
            // Allow the parent process to interface via stdin/stdout for IPC
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("child spawned correctly");

        // Make async_std's futures-io::AsyncRead/Write stdio tokio-compatible
        // using tokio-util::compat
        let stdin = command
            .stdin
            .expect("Could not capture stdin")
            .compat_write();
        let stdout = command.stdout.expect("Could not capture stdout").compat();

        Self { stdin, stdout }
    }
}

impl AsyncRead for MergedChildIO {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.stdout).poll_read(cx, buf)
    }
}

impl AsyncWrite for MergedChildIO {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        Pin::new(&mut self.stdin).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.stdin).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.stdin).poll_shutdown(cx)
    }
}
