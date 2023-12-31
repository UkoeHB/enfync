//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts
use std::fmt::Debug;

//-------------------------------------------------------------------------------------------------------------------

/// Implements `SimpleSpawner` for `tokio` runtimes (spawns tasks on a tokio runtime).
#[derive(Debug)]
pub struct TokioSpawner<R>
{
    handle: tokio::runtime::Handle,
    _phantom: std::marker::PhantomData<R>,
}

impl<R: Debug + Send + Sync + 'static> SimpleSpawner<R> for TokioSpawner<R>
{
    type Error = tokio::task::JoinError;
    type Future = tokio::task::JoinHandle<R>;

    fn spawn<F>(&self, task: F) -> Self::Future
    where
        F: std::future::Future<Output = R> + Send + 'static,
    {
        self.handle.spawn(task)
    }

    fn is_done(f: &Self::Future) -> bool
    {
        f.is_finished()
    }
}

impl<R: Send + 'static> From<tokio::runtime::Runtime> for TokioSpawner<R> {
    fn from(runtime: tokio::runtime::Runtime) -> Self {
        Self::from(runtime.handle().clone())
    }
}

impl<R: Send + 'static> From<tokio::runtime::Handle> for TokioSpawner<R> {
    fn from(handle: tokio::runtime::Handle) -> Self {
        TokioSpawner::<R>{ handle, _phantom: std::marker::PhantomData::<R>::default() }
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Implements `OneshotSpawner` for `std` runtimes (spawns new std threads).
#[derive(Debug, Clone, Default)]
pub struct StdSpawner;

impl OneshotSpawner for StdSpawner
{
    fn spawn<F>(&self, task: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        std::thread::spawn(move || futures::executor::block_on(async move { task.await }));
    }
}

impl From<builtin::native::CPUHandle> for StdSpawner { fn from(_: builtin::native::CPUHandle) -> Self { StdSpawner{} } }

//-------------------------------------------------------------------------------------------------------------------
