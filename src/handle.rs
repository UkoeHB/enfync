//local shortcuts
use crate::*;

//third-party shortcuts

//standard shortcuts
use std::fmt::Debug;

//-------------------------------------------------------------------------------------------------------------------

/// A handle to a runtime.
pub trait Handle
{
    /// Spawn a task on the handle's runtime.
    fn spawn<R, F>(&self, task: F) -> PendingResult<R>
    where
        R: Debug + Send + Sync + 'static,
        F: std::future::Future<Output = R> + Send + 'static;
}

//-------------------------------------------------------------------------------------------------------------------

/// Try to adopt the existing runtime.
///
/// Returns `None` if no runtime is detected.
pub trait TryAdopt: Sized
{
    fn try_adopt() -> Option<Self>;
}

//-------------------------------------------------------------------------------------------------------------------

/// Try to adopt the existing runtime, otherwise fall back to the default runtime.
pub trait AdoptOrDefault: TryAdopt + Default
{
    fn adopt_or_default() -> Self
    {
        if let Some(adoptee) = Self::try_adopt() { return adoptee; }
        Self::default()
    }
}

impl<T: TryAdopt + Default> AdoptOrDefault for T {}

//-------------------------------------------------------------------------------------------------------------------
