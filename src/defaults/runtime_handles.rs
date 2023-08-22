//local shortcuts

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[cfg(not(wasm))]
mod envmod
{
    use crate::*;

    /// Default IO runtime handle (tokio).
    /// If you access this via `::default()`, you will get a handle to a statically-initialized tokio runtime.
    #[derive(Clone, Debug)]
    pub struct IOHandle(pub tokio::runtime::Handle);

    impl Default for IOHandle
    {
        fn default() -> IOHandle
        {
            static RUNTIME: std::sync::OnceLock<tokio::runtime::Runtime> = std::sync::OnceLock::new();

            let runtime = RUNTIME.get_or_init(
                    || {
                        tokio::runtime::Runtime::new().expect("unable to get default tokio runtime")
                    }
                );
            IOHandle(runtime.handle().clone())
        }
    }

    impl TryAdopt for IOHandle
    {
        fn try_adopt() -> Option<IOHandle>
        {
            let Ok(handle) = tokio::runtime::Handle::try_current() else { return None; };
            Some(IOHandle::from(handle))
        }
    }

    impl From<IOHandle> for tokio::runtime::Handle
    { fn from(handle: IOHandle) -> Self { handle.0 } }

    impl From<tokio::runtime::Handle> for IOHandle
    { fn from(handle: tokio::runtime::Handle) -> Self { Self(handle) } }

    /// Default CPU runtime handle (unspecified)
    #[derive(Default)]
    pub struct CPUHandle;

    impl TryAdopt for CPUHandle { fn try_adopt() -> Option<CPUHandle> { Some(CPUHandle) } }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[cfg(wasm)]
mod envmod
{
    use crate::*;

    /// Default IO runtime handle (unspecified)
    #[derive(Clone, Debug, Default)]
    pub struct IOHandle;

    impl TryAdopt for IOHandle { fn try_adopt() -> Option<IOHandle> { Some(IOHandle) } }

    /// Default CPU runtime handle (unspecified)
    #[derive(Clone, Debug, Default)]
    pub struct CPUHandle;

    impl TryAdopt for CPUHandle { fn try_adopt() -> Option<CPUHandle> { Some(CPUHandle) } }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Default IO runtime handle (native: tokio, wasm: empty).
pub type IOHandle = envmod::IOHandle;

/// Default CPU runtime handle (native: empty, wasm: empty).
pub type CPUHandle = envmod::CPUHandle;

//-------------------------------------------------------------------------------------------------------------------
