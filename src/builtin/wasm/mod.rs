/*!
Provides built-in [`Handle`](super::Handle) implementations for WASM targets.
*/

//module tree
mod handle_impls;
mod spawner_impls;

//API exports
pub use crate::builtin::wasm::handle_impls::*;
pub use crate::builtin::wasm::spawner_impls::*;
