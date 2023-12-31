//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Implements `OneshotSpawner` for `wasm` runtimes (spawns wasm tasks).
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

impl From<builtin::wasm::WASMHandle> for WasmIOSpawner { fn from(_: builtin::wasm::WASMHandle) -> Self { WasmIOSpawner{} } }

//-------------------------------------------------------------------------------------------------------------------
