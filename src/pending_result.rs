//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Error that can be emitted when extracting a `PendingResult`.
#[derive(Debug)]
pub enum ResultError
{
    /// Result has already been taken.
    Taken,
    /// The task failed for some reason.
    TaskFailure,
}

//-------------------------------------------------------------------------------------------------------------------

/// The pending result of async work.
#[derive(Debug)]
pub struct PendingResult<Recv: ResultReceiver>
{
    result_receiver: Option<Recv>,
}

impl<'a, Recv: ResultReceiver + 'a> PendingResult<Recv>
{
    /// Make a new pending result.
    pub fn new<F>(spawner: impl Into<&'a Recv::Spawner>, task: F) -> Self
    where
        F: std::future::Future<Output = Recv::Result> + Send + 'static,
    {
        let result_receiver = Recv::new(spawner.into(), task);
        Self{ result_receiver: Some(result_receiver) }
    }

    /// Make a pending result that is immediately ready.
    pub fn immediate(spawner: impl Into<&'a Recv::Spawner>, result: Recv::Result) -> Self
    {
        let result_receiver = Recv::immediate(spawner.into(), result);
        Self{ result_receiver: Some(result_receiver) }
    }

    /// Check if result is available.
    pub fn has_result(&self) -> bool
    {
        match &self.result_receiver
        {
            // has result if done running
            Some(receiver) => receiver.done(),
            // result was already extracted
            None => false
        }
    }

    /// Check if work is done (result may be unavailable if already extracted).
    /// - This is robust for checking if a result-less task has completed (i.e. `PendingResult<()>`).
    pub fn is_done(&self) -> bool
    {
        if self.has_result() || self.result_receiver.is_none() { return true; }
        false
    }

    /// Extract result if available (non-blocking).
    /// Returns `None` if the result is still pending.
    pub fn try_extract(&mut self) -> Option<Result<Recv::Result, ResultError>>
    {
        // check if result is pending
        if !self.has_result() && !self.result_receiver.is_none() { return None; }

        // extract thread result
        Some(self.extract())
    }

    /// Extract result (blocking).
    pub fn extract(&mut self) -> Result<Recv::Result, ResultError>
    {
        futures::executor::block_on(async { self.extract_async().await })
    }

    /// Extract result (async).
    pub async fn extract_async(&mut self) -> Result<Recv::Result, ResultError>
    {
        // consume the result receiver
        let Some(receiver) = self.result_receiver.take() else { return Err(ResultError::Taken); };

        // await thread result
        let Some(res) = receiver.get().await else { return Err(ResultError::TaskFailure); };

        Ok(res)
    }
}

//-------------------------------------------------------------------------------------------------------------------
