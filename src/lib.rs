//documentation
#![doc = include_str!("../README.md")]

//module tree
mod pending_result;
mod pending_result_defaults;
mod result_receiver;
mod runtime;
mod runtime_handle_defaults;

#[cfg(not(wasm))]
mod runtime_impl_native;

#[cfg(wasm)]
mod runtime_impl_wasm;

//API exports
pub use crate::pending_result::*;
pub use crate::pending_result_defaults::*;
pub use crate::result_receiver::*;
pub use crate::runtime::*;
pub use crate::runtime_handle_defaults::*;

#[cfg(not(wasm))]
pub use crate::runtime_impl_native::*;

#[cfg(wasm)]
pub use crate::runtime_impl_wasm::*;
