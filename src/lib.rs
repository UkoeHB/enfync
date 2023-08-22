//documentation
#![doc = include_str!("../README.md")]

//module tree
mod pending_result;
mod pending_result_defaults;
mod result_receiver;
mod runtime_handle_defaults;
mod spawners;

#[cfg(not(wasm))]
mod spawner_impls_native;

#[cfg(wasm)]
mod spawner_impls_wasm;

//API exports
pub use crate::pending_result::*;
pub use crate::pending_result_defaults::*;
pub use crate::result_receiver::*;
pub use crate::runtime_handle_defaults::*;
pub use crate::spawners::*;

#[cfg(not(wasm))]
pub use crate::spawner_impls_native::*;

#[cfg(wasm)]
pub use crate::spawner_impls_wasm::*;
