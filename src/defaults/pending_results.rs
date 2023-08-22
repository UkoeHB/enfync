//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[cfg(not(wasm))]
mod envmod
{
    use crate::*;
    pub(super) type IOReceiver<R>  = SimpleResultReceiver<defaults::TokioSpawner<R>, R>;
    pub(super) type CPUReceiver<R> = OneshotResultReceiver<defaults::StdSpawner, R>;
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[cfg(wasm)]
mod envmod
{
    use crate::*;
    pub(super) type IOReceiver<R>  = OneshotResultReceiver<defaults::WasmIOSpawner, R>;
    /// note: We use the WASM IO spawner here because implementing a CPU spawner on WASM currently can only be done
    ///       with web workers, which are very inefficient and impose many restrictions on the interface (such as
    ///       requiring everything to implement `Serialize/Deserialize`, and needing explicitly-defined channels since
    ///       there is no shared memory).
    pub(super) type CPUReceiver<R> = OneshotResultReceiver<defaults::WasmIOSpawner, R>;
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Default IO pending result uses default IO receiver (native: tokio, wasm: local spawn).
pub type IOPendingResult<R> = PendingResult<envmod::IOReceiver<R>>;

/// Default CPU pending result uses default CPU receiver (native: std::thread, wasm: local spawn).
pub type CPUPendingResult<R> = PendingResult<envmod::CPUReceiver<R>>;

//-------------------------------------------------------------------------------------------------------------------
