//documentation
#![doc = include_str!("../README.md")]

//module tree
pub mod builtin;
mod pending_result;
mod result_receiver;
mod runtime_handles;
mod spawners;

//API exports
pub use crate::pending_result::*;
pub use crate::result_receiver::*;
pub use crate::runtime_handles::*;
pub use crate::spawners::*;

