//local shortcuts
use crate::builtin::*;

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[cfg(not(target_family = "wasm"))]
pub type Handle = native::TokioHandle;

#[cfg(target_family = "wasm")]
pub type Handle = wasm::WASMHandle;

//-------------------------------------------------------------------------------------------------------------------
