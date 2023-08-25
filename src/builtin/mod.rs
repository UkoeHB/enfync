//module tree
#[cfg(not(wasm))]
mod handle_impls_native;
#[cfg(not(wasm))]
mod spawner_impls_native;

#[cfg(wasm)]
mod handle_impls_wasm;
#[cfg(wasm)]
mod spawner_impls_wasm;

//API exports
#[cfg(not(wasm))]
pub use crate::builtin::handle_impls_native::*;
#[cfg(not(wasm))]
pub use crate::builtin::spawner_impls_native::*;

#[cfg(wasm)]
pub use crate::builtin::handle_impls_wasm::*;
#[cfg(wasm)]
pub use crate::builtin::spawner_impls_wasm::*;
