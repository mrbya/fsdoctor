use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
};

use fsdoctor_core::CancelToken;

/// Application state managed by Tauri.
#[derive(Debug, Default)]
pub struct AppState {
    /// Next job identifier.
    next_job_id: AtomicU64,

    /// Active cancellable jobs.
    jobs: Mutex<HashMap<String, CancelToken>>,
}

impl AppState {
    /// Creates and registers a new cancellable job.
    ///
    /// # Errors
    /// Returns an error string if the job registry lock is poisoned.
    pub fn create_job(&self, prefix: &str) -> Result<(String, CancelToken), String> {
        let id = self.next_job_id.fetch_add(1, Ordering::Relaxed);
        let job_id = format!("{prefix}-{id}");
        let token = CancelToken::default();

        self.jobs
            .lock()
            .map_err(|_error| "failed to lock application job state".to_owned())?
            .insert(job_id.clone(), token.clone());

        Ok((job_id, token))
    }

    /// Requests cancellation of a job.
    ///
    /// Returns `true` when a matching active job was found.
    ///
    /// # Errors
    /// Returns an error string if the job registry lock is poisoned.
    pub fn cancel_job(&self, job_id: &str) -> Result<bool, String> {
        let cancelled = self
            .jobs
            .lock()
            .map_err(|_error| "failed to lock application job state".to_owned())?
            .get(job_id)
            .is_some_and(|token| {
                token.cancel();
                true
            });

        Ok(cancelled)
    }

    /// Removes a finished job from the registry.
    ///
    /// # Errors
    /// Returns an error string if the job registry lock is poisoned.
    pub fn remove_job(&self, job_id: &str) -> Result<(), String> {
        self.jobs
            .lock()
            .map_err(|_error| "failed to lock application job state".to_owned())?
            .remove(job_id);

        Ok(())
    }
}
