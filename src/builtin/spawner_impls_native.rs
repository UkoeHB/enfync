//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts
use std::fmt::Debug;

//-------------------------------------------------------------------------------------------------------------------

/// Implements `SimpleSpawner` for `tokio` runtimes (spawn on tokio runtime).
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

    fn is_terminated(f: &Self::Future) -> bool
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
