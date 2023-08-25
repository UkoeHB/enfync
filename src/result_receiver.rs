//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts
use std::fmt::Debug;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

//-------------------------------------------------------------------------------------------------------------------

#[async_trait::async_trait]
pub trait ResultReceiver: Debug
{
    type Result: Send + 'static;

    /// Check if the result is ready.
    fn done(&self) -> bool;

    /// Get the result.
    /// Return `None` if the result could not be extracted (e.g. due to an error).
    async fn get(self: Box<Self>) -> Option<Self::Result>;
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct OneshotResultReceiver<S, R: Debug>
{
    done_flag: Arc<AtomicBool>,
    oneshot: futures::channel::oneshot::Receiver<Option<R>>,
    _phantom: std::marker::PhantomData<S>,
}

#[async_trait::async_trait]
impl<S, R> ResultReceiver for OneshotResultReceiver<S, R>
where
    S: Debug + Send + 'static,
    R: Debug + Send + 'static
{
    type Result = R;

    fn done(&self) -> bool
    {
        self.done_flag.load(Ordering::Acquire)
    }

    async fn get(self: Box<Self>) -> Option<Self::Result>
    {
        self.oneshot.await.unwrap_or(None)
    }
}

impl<S, R> OneshotResultReceiver<S, R>
where
    S: Debug + OneshotSpawner,
    R: Debug + Send + 'static
{
    pub fn new<F>(spawner: &S, task: F) -> Self
    where
        F: std::future::Future<Output = R> + Send + 'static,
    {
        let done_flag = Arc::new(AtomicBool::new(false));
        let done_flag_clone = done_flag.clone();
        let (result_sender, result_receiver) = futures::channel::oneshot::channel();
        let work_task = async move {
                let result = task.await;
                let _ = result_sender.send(Some(result));
                done_flag_clone.store(true, Ordering::Release);
            };
        spawner.spawn(work_task);

        Self{ done_flag, oneshot: result_receiver, _phantom: std::marker::PhantomData::<S>::default() }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct SimpleResultReceiver<S: SimpleSpawner<R>, R: Debug>
{
    future_result: <S as SimpleSpawner<R>>::Future,
}

#[async_trait::async_trait]
impl<S, R> ResultReceiver for SimpleResultReceiver<S, R>
where
    S: SimpleSpawner<R>,
    R: Debug + Send + 'static,
{
    type Result = R;

    fn done(&self) -> bool
    {
        S::is_terminated(&self.future_result)
    }

    async fn get(self: Box<Self>) -> Option<Self::Result>
    {
        let Ok(result) = self.future_result.await else { return None; };
        Some(result)
    }
}

impl<S, R> SimpleResultReceiver<S, R>
where
    S: SimpleSpawner<R>,
    R: Debug + Send + 'static,
{
    pub fn new<F>(spawner: &S, task: F) -> Self
    where
        F: std::future::Future<Output = R> + Send + 'static,
    {
        let future_result = spawner.spawn(task);

        Self{ future_result }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ImmedateResultReceiver<R: Debug>
{
    result: R,
}

#[async_trait::async_trait]
impl<R: Debug + Send + 'static> ResultReceiver for ImmedateResultReceiver<R>
{
    type Result = R;

    fn done(&self) -> bool
    {
        true
    }

    async fn get(self: Box<Self>) -> Option<Self::Result>
    {
        Some(self.result)
    }
}

impl<R: Debug> ImmedateResultReceiver<R>
{
    pub fn new<F>(task: F) -> Self
    where
        F: std::future::Future<Output = R> + Send + 'static,
    {
        let result = futures::executor::block_on(task);

        Self{ result }
    }
}

//-------------------------------------------------------------------------------------------------------------------
