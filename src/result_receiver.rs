//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts
use futures::future::MaybeDone;
use std::fmt::Debug;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

//-------------------------------------------------------------------------------------------------------------------

#[async_trait::async_trait]
pub trait ResultReceiver: Debug
{
    type Result: Send + Sync + 'static;

    /// Check if the result is ready.
    fn done(&self) -> bool;

    /// Try to get a result.
    /// Return `None` if the result is not available.
    /// Return `Some(Err)` if the result could not be extracted (e.g. due to an error OR due to the result already
    /// having been extracted.
    fn try_get(&mut self) -> Option<Result<Self::Result, ResultError>>;

    /// Get the result.
    /// Return `Err` if the result could not be extracted (e.g. due to an error OR due to the result already
    /// having been extracted.
    async fn get(self: Box<Self>) -> Result<Self::Result, ResultError>;
}

//-------------------------------------------------------------------------------------------------------------------

/// Oneshot result receiver uses a oneshot to receive the result.
#[derive(Debug)]
pub struct OneshotResultReceiver<R: Debug>
{
    done_flag: Arc<AtomicBool>,
    oneshot: futures::channel::oneshot::Receiver<R>,
    result_taken: bool,
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
        match self.oneshot.try_recv()
        {
            Ok(Some(res)) => { self.result_taken = true; Some(Ok(res)) },
            Err(_)        => { self.result_taken = true; Some(Err(ResultError::TaskFailure)) },
            Ok(None)      => match self.result_taken { true => Some(Err(ResultError::Taken)), false => None },
        }
    }

    async fn get(self: Box<Self>) -> Result<Self::Result, ResultError>
    {
        match self.oneshot.await
        {
            Ok(res) => Ok(res),
            _       => Err(ResultError::TaskFailure),
        }
    }
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

        Self{ done_flag, oneshot: result_receiver, result_taken: false }
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Simple result receiver uses a future to receive the result.
#[derive(Debug)]
pub struct SimpleResultReceiver<S: SimpleSpawner<R>, R: Debug>
{
    future_result: MaybeDone<<S as SimpleSpawner<R>>::Future>,
    result_taken: bool,
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
            MaybeDone::Future(fut) => S::is_terminated(fut),
            MaybeDone::Done(_)     => true,
            MaybeDone::Gone        => true,
        }
    }

    fn try_get(&mut self) -> Option<Result<Self::Result, ResultError>>
    {
        let pinned_fut = Pin::new(&mut self.future_result);
        
        match pinned_fut.take_output()
        {
            Some(Ok(res)) => { self.result_taken = true; Some(Ok(res)) }
            Some(Err(_))  => { self.result_taken = true; Some(Err(ResultError::TaskFailure)) }
            None          => match self.result_taken { true => Some(Err(ResultError::Taken)), false => None }
        }
    }

    async fn get(mut self: Box<Self>) -> Result<Self::Result, ResultError>
    {
        let res =
            match self.future_result
            {
                MaybeDone::Future(fut) => fut.await,
                MaybeDone::Done(res)   => res,
                MaybeDone::Gone        => return Err(ResultError::Taken),
            };

        res.map_err(|_| ResultError::TaskFailure)
    }
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

        Self{ future_result, result_taken: false }
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Immediate result receiver.
#[derive(Debug)]
pub struct ImmedateResultReceiver<R: Debug>
{
    result: Option<R>,
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

impl<R: Debug> ImmedateResultReceiver<R>
{
    pub fn new(result: R) -> Self
    {
        Self{ result: Some(result) }
    }
}

//-------------------------------------------------------------------------------------------------------------------
