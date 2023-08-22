//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts
use futures::future::FusedFuture;


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

    /// Make a result receiver with an immediately-available result.
    fn immediate(spawner: &Self::Spawner, result: Self::Result) -> Self;

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
        let (result_sender, result_receiver) = futures::channel::oneshot::channel();
        let work_task = async move {
                let result = task.await;
                let _ = result_sender.send(Some(result));
            };
        spawner.spawn(work_task);

        Self{ oneshot: result_receiver, _phantom: std::marker::PhantomData::<Self::Spawner>::default() }
    }

    fn immediate(_spawner: &Self::Spawner, result: Self::Result) -> Self
    {
        let (result_sender, result_receiver) = futures::channel::oneshot::channel();
        let _ = result_sender.send(Some(result));

        Self{ oneshot: result_receiver, _phantom: std::marker::PhantomData::<Self::Spawner>::default() }
    }

    fn done(&self) -> bool
    {
        self.oneshot.is_terminated()
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

    fn immediate(spawner: &Self::Spawner, result: Self::Result) -> Self
    {
        let future_result = spawner.spawn(futures::future::ready(result));

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
