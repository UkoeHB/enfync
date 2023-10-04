//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts
use std::fmt::Debug;
use std::future::Future;

//-------------------------------------------------------------------------------------------------------------------

/// Built-in IO runtime handle (spawns wasm tasks).
#[derive(Clone, Debug, Default)]
pub struct WASMHandle;

impl Handle for WASMHandle
{
    fn spawn<R, F>(&self, task: F) -> PendingResult<R>
    where
        R: Debug + Send + Sync + 'static,
        F: Future<Output = R> + Send + 'static
    {
        let result_receiver = OneshotResultReceiver::new(&builtin::wasm::WasmIOSpawner{}, task);
        PendingResult::new(result_receiver)
    }
}

impl TryAdopt for WASMHandle { fn try_adopt() -> Option<WASMHandle> { Some(WASMHandle) } }

//-------------------------------------------------------------------------------------------------------------------
