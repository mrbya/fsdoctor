use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// Cancellation token checked by long-running hashing operations.
#[derive(Debug, Clone, Default)]
pub struct CancelToken {
    /// Shared cancellation flag.
    cancelled: Arc<AtomicBool>,
}

impl CancelToken {
    /// Requests cancellation.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    /// Returns whether cancellation was requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}
