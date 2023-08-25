//local shortcuts
use crate::{builtin, *};

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Implements `OneshotSpawner` for `wasm` runtimes (spawn on local thread).
#[derive(Debug, Clone, Default)]
pub struct WasmIOSpawner;

impl OneshotSpawner for WasmIOSpawner
{
    fn spawn<F>(&self, task: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        wasm_bindgen_futures::spawn_local(
                async move {
                        task.await;
                    }
            );
    }
}

impl From<builtin::IOHandle>  for WasmIOSpawner { fn from(_: builtin::IOHandle)  -> Self { WasmIOSpawner{} } }
impl From<builtin::CPUHandle> for WasmIOSpawner { fn from(_: builtin::CPUHandle) -> Self { WasmIOSpawner{} } }

//-------------------------------------------------------------------------------------------------------------------
