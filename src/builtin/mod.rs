/*!
Provides built-in [`Handle`] implementations for native and WASM targets.
*/

//module tree
mod builtin;

#[cfg(not(target_family = "wasm"))]
#[cfg_attr(docsrs, doc(cfg(not(target_family = "wasm"))))]
pub mod native;

#[cfg(any(target_family = "wasm", doc))]
#[cfg_attr(docsrs, doc(cfg(target_family = "wasm")))]
pub mod wasm;

//API exports
pub use crate::builtin::builtin::*;
