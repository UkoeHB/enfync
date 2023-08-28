//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts
use std::fmt::Debug;
use std::future::Future;

//-------------------------------------------------------------------------------------------------------------------

/// Built-in IO runtime handle (spawns tokio tasks).
/// If you access this via `::default()`, you will get a handle to a statically-initialized tokio runtime.
#[derive(Clone, Debug)]
pub struct TokioHandle(pub tokio::runtime::Handle);

impl HandleTrait for TokioHandle
{
    fn spawn<R, F>(&self, task: F) -> PendingResult<R>
    where
        R: Debug + Send + Sync + 'static,
        F: Future<Output = R> + Send + 'static
    {
        let spawner = builtin::native::TokioSpawner::<R>::from(self.0.clone());
        let result_receiver = SimpleResultReceiver::new(&spawner, task);
        PendingResult::new(result_receiver)
    }
}

impl Default for TokioHandle
{
    fn default() -> TokioHandle
    {
        static RUNTIME: std::sync::OnceLock<tokio::runtime::Runtime> = std::sync::OnceLock::new();

        let runtime = RUNTIME.get_or_init(
                || {
                    tokio::runtime::Runtime::new().expect("unable to make default tokio runtime")
                }
            );
            TokioHandle(runtime.handle().clone())
    }
}

impl TryAdopt for TokioHandle
{
    fn try_adopt() -> Option<TokioHandle>
    {
        let Ok(handle) = tokio::runtime::Handle::try_current() else { return None; };
        Some(TokioHandle::from(handle))
    }
}

impl From<TokioHandle> for tokio::runtime::Handle
{ fn from(handle: TokioHandle) -> Self { handle.0 } }

impl From<tokio::runtime::Handle> for TokioHandle
{ fn from(handle: tokio::runtime::Handle) -> Self { Self(handle) } }

//-------------------------------------------------------------------------------------------------------------------

/// Built-in CPU runtime handle (std threads)
#[derive(Default)]
pub struct CPUHandle;

impl HandleTrait for CPUHandle
{
    fn spawn<R, F>(&self, task: F) -> PendingResult<R>
    where
        R: Debug + Send + Sync + 'static,
        F: Future<Output = R> + Send + 'static
    {
        let result_receiver = OneshotResultReceiver::new(&builtin::native::StdSpawner{}, task);
        PendingResult::new(result_receiver)
    }
}

impl TryAdopt for CPUHandle { fn try_adopt() -> Option<CPUHandle> { Some(CPUHandle) } }

//-------------------------------------------------------------------------------------------------------------------
