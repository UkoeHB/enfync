//documentation
#![doc = include_str!("../README.md")]

//module tree
pub mod builtin;
mod handle;
mod pending_result;
mod result_receiver;
mod sleep;
mod spawners;

//API exports
pub use crate::handle::*;
pub use crate::pending_result::*;
pub use crate::result_receiver::*;
pub use crate::sleep::*;
pub use crate::spawners::*;

