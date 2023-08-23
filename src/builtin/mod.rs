//module tree
mod pending_results;
mod runtime_handles;

#[cfg(not(wasm))]
mod spawner_impls_native;

#[cfg(wasm)]
mod spawner_impls_wasm;

//API exports
pub use crate::builtin::pending_results::*;
pub use crate::builtin::runtime_handles::*;

#[cfg(not(wasm))]
pub use crate::builtin::spawner_impls_native::*;

#[cfg(wasm)]
pub use crate::builtin::spawner_impls_wasm::*;
