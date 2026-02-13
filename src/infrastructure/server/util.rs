use std::process::Stdio;
use std::pin::Pin;
use std::task::{Context, Poll};
use derive_new::new;
use log::warn;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;
use tokio::process::{Child, Command};
use tokio_stream::{Stream, StreamExt};

/// Wrapper struct that keeps the child process alive for the lifetime of the stream
/// and ensures proper cleanup when the stream is dropped.
pub struct ChildProcessStream {
    child: Child,
    stream: Pin<Box<dyn Stream<Item = String> + Send + Sync>>,
}

impl ChildProcessStream {
    fn new(stream: Pin<Box<dyn Stream<Item = String> + Send + Sync>>, child: Child) -> Self {
        Self { child, stream }
    }
}

impl Stream for ChildProcessStream {
    type Item = String;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.stream.as_mut().poll_next(cx)
    }
}

impl Drop for ChildProcessStream {
    fn drop(&mut self) {
        // Attempt to kill the child process when the stream is dropped
        if let Err(e) = self.child.start_kill() {
            warn!("Failed to kill child process during stream cleanup: {}", e);
        }
    }
}

#[derive(new)]
pub struct SystemCommandExecutor;

impl SystemCommandExecutor {
    
    pub async fn execute(&self, cmd: &str, args: &[&str]) -> bool {
        let status = Command::new(cmd)
            .args(args)
            .status()
            .await;

        match status {
            Ok(s) => s.success(),
            Err(_) => false,
        }
    }
    
    pub async fn capture_output(&self, cmd: &str, args: &[&str]) -> Result<String, std::io::Error> {
        let output = Command::new(cmd)
            .args(args)
            .output()
            .await?;

        let stderr_output = String::from_utf8_lossy(&output.stderr);
        if !stderr_output.is_empty() {
            warn!("[{} stderr]: {}", cmd, stderr_output);
        }

        if output.status.success() {
            let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
            let error_msg = String::from_utf8_lossy(&output.stderr);

            if !error_msg.is_empty() {
                combined.push_str("\n--- Error ---\n");
                combined.push_str(&error_msg);
            }
            Ok(combined)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, stderr_output.to_string()))
        }
    }

    pub async fn capture_output_follow(&self, cmd: &str, args: &[&str]) -> Result<Box<dyn Stream<Item = String> + Send>, std::io::Error> {
        let mut child = Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let stdout_stream = LinesStream::new(BufReader::new(stdout).lines())
            .map(|res| res.unwrap_or_else(|e| format!("stdout error: {}", e)));
        let stderr_stream = LinesStream::new(BufReader::new(stderr).lines())
            .map(|res| res.unwrap_or_else(|e| format!("stderr error: {}", e)));

        let combined_stream = stdout_stream.merge(stderr_stream);

        Ok(Box::new(ChildProcessStream::new(Box::pin(combined_stream), child)))
    }
}