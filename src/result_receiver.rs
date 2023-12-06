//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts
use futures::Future;
use futures::future::{FusedFuture, MaybeDone};
use std::fmt::Debug;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

//-------------------------------------------------------------------------------------------------------------------

/// Represents a result receiver for async tasks. See [`PendingResult::new()`].
#[async_trait::async_trait]
pub trait ResultReceiver: Debug
{
    type Result: Send + Sync + 'static;

    /// Check if the result is ready.
    fn done(&self) -> bool;

    /// Try to get the result.
    /// - Returns `None` if the result is not available.
    /// - Returns `Some(Err)` if the result could not be extracted (e.g. due to a task error OR due to the result already
    ///   having been extracted.
    fn try_get(&mut self) -> Option<Result<Self::Result, ResultError>>;

    /// Consume self to get the result.
    /// - Returns `Err` if the result could not be extracted (e.g. due to a task error OR due to the result already
    ///   having been extracted.
    async fn get(self: Box<Self>) -> Result<Self::Result, ResultError>;
}

//-------------------------------------------------------------------------------------------------------------------

/// Uses a oneshot to receive the result.
#[derive(Debug)]
pub struct OneshotResultReceiver<R: Debug>
{
    done_flag       : Arc<AtomicBool>,
    result_receiver : futures::channel::oneshot::Receiver<R>,
    result_taken    : Option<ResultError>,
}

impl<R> OneshotResultReceiver<R>
where
    R: Debug + Send + Sync + 'static
{
    pub fn new<S, F>(spawner: &S, task: F) -> Self
    where
        S: OneshotSpawner,
        F: std::future::Future<Output = R> + Send + 'static,
    {
        let done_flag = Arc::new(AtomicBool::new(false));
        let done_flag_clone = done_flag.clone();
        let (result_sender, result_receiver) = futures::channel::oneshot::channel();
        let work_task = async move {
                let result = task.await;
                let _ = result_sender.send(result);

                // ORDERING
                // WASM compiles `AtomicBool` as `bool`, however since WASM is fully single-threaded, the ordering
                // guarantee here is preserved.
                done_flag_clone.store(true, Ordering::Release);
            };
        spawner.spawn(work_task);

        Self{ done_flag, result_receiver, result_taken: None }
    }
}

#[async_trait::async_trait]
impl<R> ResultReceiver for OneshotResultReceiver<R>
where
    R: Debug + Send + Sync + 'static,
{
    type Result = R;

    fn done(&self) -> bool
    {
        self.done_flag.load(Ordering::Acquire)
    }

    fn try_get(&mut self) -> Option<Result<Self::Result, ResultError>>
    {
        match self.result_receiver.try_recv()
        {
            Ok(Some(res)) => { self.result_taken = Some(ResultError::Taken); Some(Ok(res)) },
            Err(_)        => { self.result_taken = Some(ResultError::TaskFailure); self.result_taken.map(|e| Err(e)) },
            Ok(None)      => self.result_taken.map(|e| Err(e)),
        }
    }

    async fn get(self: Box<Self>) -> Result<Self::Result, ResultError>
    {
        if let Some(error) = self.result_taken { return Err(error); }
        match self.result_receiver.await
        {
            Ok(res) => Ok(res),
            _       => Err(ResultError::TaskFailure),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Uses a future to receive the result.
#[derive(Debug)]
pub struct SimpleResultReceiver<S: SimpleSpawner<R>, R: Debug>
{
    future_result : MaybeDone<<S as SimpleSpawner<R>>::Future>,
    result_taken  : Option<ResultError>,
}

impl<S, R> SimpleResultReceiver<S, R>
where
    S: SimpleSpawner<R>,
    R: Debug + Send + Sync + 'static,
{
    pub fn new<F>(spawner: &S, task: F) -> Self
    where
        F: std::future::Future<Output = R> + Send + 'static,
    {
        let future_result = futures::future::maybe_done(spawner.spawn(task));

        Self{ future_result, result_taken: None }
    }
}

#[async_trait::async_trait]
impl<S, R> ResultReceiver for SimpleResultReceiver<S, R>
where
    S: SimpleSpawner<R>,
    <S as spawners::SimpleSpawner<R>>::Future: Unpin,
    R: Debug + Send + Sync + 'static,
{
    type Result = R;

    fn done(&self) -> bool
    {
        match &self.future_result
        {
            MaybeDone::Future(fut) => S::is_done(fut),
            MaybeDone::Done(_)     => true,
            MaybeDone::Gone        => true,
        }
    }

    fn try_get(&mut self) -> Option<Result<Self::Result, ResultError>>
    {
        // poll the future once
        let mut pinned_fut = Pin::new(&mut self.future_result);

        if !pinned_fut.is_terminated()
        {
            let noop_waker = futures::task::noop_waker();
            let mut ctx = futures::task::Context::from_waker(&noop_waker);
            let _ = pinned_fut.as_mut().poll(&mut ctx);
        }

        // check output
        match pinned_fut.take_output()
        {
            Some(Ok(res)) => { self.result_taken = Some(ResultError::Taken); Some(Ok(res)) }
            Some(Err(_))  => { self.result_taken = Some(ResultError::TaskFailure); self.result_taken.map(|e| Err(e)) },
            None          => self.result_taken.map(|e| Err(e)),
        }
    }

    async fn get(mut self: Box<Self>) -> Result<Self::Result, ResultError>
    {
        if let Some(error) = self.result_taken { return Err(error); }
        let res = match self.future_result
        {
            MaybeDone::Future(fut) => fut.await,
            MaybeDone::Done(res)   => res,
            MaybeDone::Gone        => return Err(ResultError::Taken),
        };

        res.map_err(|_| ResultError::TaskFailure)
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Result receiver with an immediately-available result.
#[derive(Debug)]
pub struct ImmedateResultReceiver<R: Debug>
{
    result: Option<R>,
}

impl<R: Debug> ImmedateResultReceiver<R>
{
    pub fn new(result: R) -> Self
    {
        Self{ result: Some(result) }
    }
}

#[async_trait::async_trait]
impl<R: Debug + Send + Sync + 'static> ResultReceiver for ImmedateResultReceiver<R>
{
    type Result = R;

    fn done(&self) -> bool
    {
        true
    }

    fn try_get(&mut self) -> Option<Result<Self::Result, ResultError>>
    {
        match self.result.take()
        {
            Some(res) => Some(Ok(res)),
            None      => Some(Err(ResultError::Taken))
        }
    }

    async fn get(mut self: Box<Self>) -> Result<Self::Result, ResultError>
    {
        self.try_get().unwrap_or(Err(ResultError::Taken))
    }
}

//-------------------------------------------------------------------------------------------------------------------
