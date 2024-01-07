//features
#![cfg_attr(docsrs, feature(doc_cfg))]

//documentation
#![doc = include_str!("../README.md")]
#![allow(unused_imports)]
use crate as enfync;

//module tree
mod handle;
mod pending_result;
mod result_receiver;
mod sleep;
mod spawners;

#[cfg(feature = "builtin")]
#[cfg_attr(docsrs, doc(cfg(feature = "builtin")))]
pub mod builtin;

//API exports
pub use crate::handle::*;
pub use crate::pending_result::*;
pub use crate::result_receiver::*;
pub use crate::sleep::*;
pub use crate::spawners::*;
