//local shortcuts

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Implements `OneshotSpawner` for `wasm` runtimes (spawn on local thread).
/// If no other type implements `From<DefaultIOHandle>`, this is the default IO spawner on WASM builds.
/// If no other type implements `From<DefaultCPUHandle>`, this is the default CPU spawner on WASM builds.
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

impl From<DefaultIOHandle> for WasmIOSpawner {
    fn from(_: DefaultIOHandle) -> Self {
        WasmIOSpawner{}
    }
}

impl From<&DefaultIOHandle> for WasmIOSpawner {
    fn from(_: &DefaultIOHandle) -> Self {
        WasmIOSpawner{}
    }
}

impl From<DefaultCPUHandle> for WasmIOSpawner {
    fn from(_: DefaultCPUHandle) -> Self {
        WasmIOSpawner{}
    }
}

impl From<&DefaultCPUHandle> for WasmIOSpawner {
    fn from(_: &DefaultCPUHandle) -> Self {
        WasmIOSpawner{}
    }
}

//-------------------------------------------------------------------------------------------------------------------
