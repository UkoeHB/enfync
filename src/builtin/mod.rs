/*!
Provides built-in [`Handle`] implementations for native and WASM targets.

Requires feature `builtin`.
*/

//module tree
mod builtin;

#[cfg(not(target_family = "wasm"))]
pub mod native;

#[cfg(any(target_family = "wasm", doc))]
pub mod wasm;

//API exports
pub use crate::builtin::builtin::*;
