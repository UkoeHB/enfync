//local shortcuts
use crate::{defaults, *};

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Implements `OneshotSpawner` for `wasm` runtimes (spawn on local thread).
/// If no other type implements `From<enfync::defaults::IOHandle>`, this is the default IO spawner on WASM builds.
/// If no other type implements `From<enfync::defaults::CPUHandle>`, this is the default CPU spawner on WASM builds.
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

impl From<defaults::IOHandle>  for WasmIOSpawner { fn from(_: defaults::IOHandle)  -> Self { WasmIOSpawner{} } }
impl From<defaults::CPUHandle> for WasmIOSpawner { fn from(_: defaults::CPUHandle) -> Self { WasmIOSpawner{} } }

//-------------------------------------------------------------------------------------------------------------------
