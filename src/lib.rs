use std::{ffi::OsStr, pin::Pin, process::Stdio, task::Poll};

use tokio::{
    io::{AsyncRead, AsyncWrite, Stdin, Stdout},
    process::{ChildStdin, ChildStdout, Command},
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
    stdout: ChildStdout,
    stdin: ChildStdin,
}

impl MergedChildIO {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        let command = Command::new(program)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("child spawned correctly");

        let stdin = command.stdin.expect("Could not capture stdin");
        let stdout = command.stdout.expect("Could not capture stdout");

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

/// An instance of merged child process stdio used to implement `AsyncRead` and
/// `AsyncWrite`, as required by `serde_transport` for use as a transport for tarpc.
pub struct MergedProcessIO {
    stdin: Stdin,
    stdout: Stdout,
}

impl MergedProcessIO {
    pub fn new() -> Self {
        MergedProcessIO {
            stdin: tokio::io::stdin(),
            stdout: tokio::io::stdout(),
        }
    }
}

impl AsyncRead for MergedProcessIO {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.stdin).poll_read(cx, buf)
    }
}

impl AsyncWrite for MergedProcessIO {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        Pin::new(&mut self.stdout).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.stdout).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.stdout).poll_shutdown(cx)
    }
}
