//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts
use std::fmt::Debug;

//-------------------------------------------------------------------------------------------------------------------

/// Error that can be emitted by a [`PendingResult`].
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
pub struct PendingResult<R>
{
    result_receiver: Option<Box<dyn ResultReceiver<Result = R> + Send + Sync>>,
}

impl<R: Debug + Send + Sync + 'static> PendingResult<R>
{
    /// Make a new pending result.
    pub fn new(receiver: impl ResultReceiver<Result = R> + Send + Sync + 'static) -> Self
    {
        Self{ result_receiver: Some(Box::new(receiver)) }
    }

    /// Make a pending result that is immediately ready.
    pub fn make_ready(result: R) -> Self
    {
        Self{ result_receiver: Some(Box::new(ImmedateResultReceiver::new(result))) }
    }

    /// Check if the result is available.
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

    /// Check if work is done (the result may be unavailable if it was already extracted).
    /// - This is robust for checking if a result-less task has completed (i.e. `PendingResult<()>`).
    pub fn done(&self) -> bool
    {
        if self.has_result() || self.result_receiver.is_none() { return true; }
        false
    }

    /// Extract the result if available (non-blocking).
    ///
    /// Returns `None` if the result is still pending.
    pub fn try_extract(&mut self) -> Option<Result<R, ResultError>>
    {
        // check if result is pending
        if !self.has_result() && self.result_receiver.is_some() { return None; }

        // extract result
        match &mut self.result_receiver
        {
            Some(receiver) => receiver.try_get(),
            None           => Some(Err(ResultError::Taken)),
        }
    }

    /// Extract the result (async).
    ///
    /// This method is not cancellation-safe.
    pub async fn extract(&mut self) -> Result<R, ResultError>
    {
        // consume the result receiver
        let Some(receiver) = self.result_receiver.take() else { return Err(ResultError::Taken); };

        // await result
        receiver.get().await
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Only available on non-WASM targets.
#[cfg(not(target_family = "wasm"))]
pub mod blocking
{
    /// Extract a pending result while blocking the current thread.
    ///
    /// Not available on WASM targets.
    pub fn extract<R>(mut pending_result: super::PendingResult<R>) -> Result<R, super::ResultError>
    where
        R: Send + Sync + std::fmt::Debug + 'static
    {
        futures::executor::block_on(async move { pending_result.extract().await })
    }
}

//-------------------------------------------------------------------------------------------------------------------
