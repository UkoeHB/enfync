//module tree
#[cfg(not(target_family = "wasm"))]
mod handle_impls_native;
#[cfg(not(target_family = "wasm"))]
mod spawner_impls_native;

#[cfg(target_family = "wasm")]
mod handle_impls_wasm;
#[cfg(target_family = "wasm")]
mod spawner_impls_wasm;

//API exports
#[cfg(not(target_family = "wasm"))]
pub use crate::builtin::handle_impls_native::*;
#[cfg(not(target_family = "wasm"))]
pub use crate::builtin::spawner_impls_native::*;

#[cfg(target_family = "wasm")]
pub use crate::builtin::handle_impls_wasm::*;
#[cfg(target_family = "wasm")]
pub use crate::builtin::spawner_impls_wasm::*;
