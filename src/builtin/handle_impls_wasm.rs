//local shortcuts
use crate::{*, builtin::*};

//third-party shortcuts

//standard shortcuts
use std::fmt::Debug;
use std::future::Future;

//-------------------------------------------------------------------------------------------------------------------

/// Built-in IO runtime handle (wasm task)
#[derive(Clone, Debug, Default)]
pub struct IOHandle;

impl Handle for IOHandle
{
    fn spawn<R, F>(&self, task: F) -> PendingResult<R>
    where
        R: Debug + Send + Sync + 'static,
        F: Future<Output = R> + Send + 'static
    {
        let result_receiver = OneshotResultReceiver::new(&WasmIOSpawner{}, task);
        PendingResult::new(result_receiver)
    }
}

impl TryAdopt for IOHandle { fn try_adopt() -> Option<IOHandle> { Some(IOHandle) } }

//-------------------------------------------------------------------------------------------------------------------

/// Built-in CPU runtime handle (wasm task)
/// note: We use the WASM IO spawner here because implementing a CPU spawner on WASM currently can only be done
///       with web workers, which are very inefficient and impose many restrictions on the interface (such as
///       requiring everything to implement `Serialize/Deserialize`, and needing explicitly-defined channels since
///       there is no shared memory).
pub type CPUHandle = IOHandle;

//-------------------------------------------------------------------------------------------------------------------
