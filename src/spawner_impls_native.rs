//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts
use std::fmt::Debug;

//-------------------------------------------------------------------------------------------------------------------

/// Implements `SimpleSpawner` for `tokio` runtimes (spawn on tokio runtime).
/// If no other type implements `From<DefaultIOHandle>`, this is the default IO spawner on native builds.
#[derive(Debug)]
pub struct TokioSpawner<R>
{
    handle: tokio::runtime::Handle,
    _phantom: std::marker::PhantomData<R>,
}

impl<R: Debug + Send + 'static> SimpleSpawner<R> for TokioSpawner<R>
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
        Self::from(&runtime)
    }
}

impl<R: Send + 'static> From<&tokio::runtime::Runtime> for TokioSpawner<R> {
    fn from(runtime: &tokio::runtime::Runtime) -> Self {
        Self::from(runtime.handle().clone())
    }
}

impl<R: Send + 'static> From<tokio::runtime::Handle> for TokioSpawner<R> {
    fn from(handle: tokio::runtime::Handle) -> Self {
        TokioSpawner::<R>{ handle, _phantom: std::marker::PhantomData::<R>::default() }
    }
}

impl<R: Send + 'static> From<&tokio::runtime::Handle> for TokioSpawner<R> {
    fn from(handle: &tokio::runtime::Handle) -> Self {
        Self::from(handle.clone())
    }
}

impl<R: Send + 'static> From<DefaultIOHandle> for TokioSpawner<R> {
    fn from(handle: DefaultIOHandle) -> Self {
        Self::from(tokio::runtime::Handle::from(handle))
    }
}

impl<R: Send + 'static> From<&DefaultIOHandle> for TokioSpawner<R> {
    fn from(handle: &DefaultIOHandle) -> Self {
        Self::from(handle.clone())
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Implements `OneshotSpawner` for `std` runtimes (spawn new thread).
/// If no other type implements `From<DefaultCPUHandle>`, this is the default CPU spawner on native builds.
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

impl From<DefaultCPUHandle> for StdSpawner {
    fn from(_: DefaultCPUHandle) -> Self {
        StdSpawner{}
    }
}

impl From<&DefaultCPUHandle> for StdSpawner {
    fn from(_: &DefaultCPUHandle) -> Self {
        StdSpawner{}
    }
}

//-------------------------------------------------------------------------------------------------------------------