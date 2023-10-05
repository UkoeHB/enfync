//module tree
mod builtin;

#[cfg(not(target_family = "wasm"))]
pub mod native;

#[cfg(target_family = "wasm")]
pub mod wasm;

//API exports
pub use crate::builtin::builtin::*;
