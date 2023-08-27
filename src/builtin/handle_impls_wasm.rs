//local shortcuts
use crate::{*, builtin::*};

//third-party shortcuts

//standard shortcuts
use std::fmt::Debug;
use std::future::Future;

//-------------------------------------------------------------------------------------------------------------------

/// Built-in IO runtime handle (spawns wasm tasks).
#[derive(Clone, Debug, Default)]
pub struct Handle;

impl HandleTrait for Handle
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

impl TryAdopt for Handle { fn try_adopt() -> Option<Handle> { Some(Handle) } }

//-------------------------------------------------------------------------------------------------------------------
