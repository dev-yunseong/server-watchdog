use std::pin::Pin;
use std::task::{Context, Poll};
use log::warn;
use tokio::process::Child;
use tokio_stream::Stream;

/// Wrapper struct that keeps the child process alive for the lifetime of the stream
/// and ensures proper cleanup when the stream is dropped.
pub struct ChildProcessStream {
    child: Child,
    stream: Pin<Box<dyn Stream<Item = String> + Send + Sync>>,
}

impl ChildProcessStream {
    pub(crate) fn new(stream: Pin<Box<dyn Stream<Item = String> + Send + Sync>>, child: Child) -> Self {
        Self { child, stream }
    }

    /// Converts this stream into a boxed trait object.
    /// This is a convenience method to avoid verbose casting at call sites.
    pub fn into_box(self) -> Box<dyn Stream<Item = String> + Send + Sync> {
        Box::new(self)
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
