//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

//-------------------------------------------------------------------------------------------------------------------

#[async_trait::async_trait]
pub trait ResultReceiver
{
    type Spawner;
    type Result: Send + 'static;

    /// Make a new result receiver.
    fn new<F>(spawner: &Self::Spawner, task: F) -> Self
    where
        F: std::future::Future<Output = Self::Result> + Send + 'static;

    /// Check if the result is ready.
    fn done(&self) -> bool;

    /// Get the result.
    /// Return `None` if the result could not be extracted (e.g. due to an error).
    async fn get(mut self) -> Option<Self::Result>;
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct OneshotResultReceiver<S, R>
{
    done_flag: Arc<AtomicBool>,
    oneshot: futures::channel::oneshot::Receiver<Option<R>>,
    _phantom: std::marker::PhantomData<S>,
}

#[async_trait::async_trait]
impl<S, R> ResultReceiver for OneshotResultReceiver<S, R>
where
    S: OneshotSpawner,
    R: Send + 'static
{
    type Spawner = S;
    type Result = R;

    fn new<F>(spawner: &Self::Spawner, task: F) -> Self
    where
        F: std::future::Future<Output = Self::Result> + Send + 'static,
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

        Self{ done_flag, oneshot: result_receiver, _phantom: std::marker::PhantomData::<Self::Spawner>::default() }
    }

    fn done(&self) -> bool
    {
        self.done_flag.load(Ordering::Acquire)
    }

    async fn get(mut self) -> Option<Self::Result>
    {
        self.oneshot.await.unwrap_or(None)
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct SimpleResultReceiver<S: SimpleSpawner<R>, R>
{
    future_result: <S as SimpleSpawner<R>>::Future,
}

#[async_trait::async_trait]
impl<S, R> ResultReceiver for SimpleResultReceiver<S, R>
where
    S: SimpleSpawner<R>,
    R: Send + 'static,
{
    type Spawner = S;
    type Result = R;

    fn new<F>(spawner: &Self::Spawner, task: F) -> Self
    where
        F: std::future::Future<Output = Self::Result> + Send + 'static,
    {
        let future_result = spawner.spawn(task);

        Self{ future_result }
    }

    fn done(&self) -> bool
    {
        Self::Spawner::is_terminated(&self.future_result)
    }

    async fn get(mut self) -> Option<Self::Result>
    {
        let Ok(result) = self.future_result.await else { return None; };
        Some(result)
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ImmedateResultReceiver<R>
{
    result: R,
}

#[async_trait::async_trait]
impl<R> ResultReceiver for ImmedateResultReceiver<R>
{
    type Spawner = ();
    type Result = R;

    fn new<F>(_: &(), task: F) -> Self
    where
        F: std::future::Future<Output = Self::Result> + Send + 'static,
    {
        let result = futures::executor::block_on(task);

        Self{ result }
    }

    fn done(&self) -> bool
    {
        true
    }

    async fn get(mut self) -> Option<Self::Result>
    {
        Some(self.result)
    }
}

//-------------------------------------------------------------------------------------------------------------------
