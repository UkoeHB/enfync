/*!
Provides built-in [`Handle`](super::Handle) implementations for native targets.
*/

//module tree
mod handle_impls;
mod spawner_impls;

//API exports
pub use crate::builtin::native::handle_impls::*;
pub use crate::builtin::native::spawner_impls::*;
