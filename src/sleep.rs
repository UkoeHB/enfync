//local shortcuts

//third-party shortcuts

//standard shortcuts
use std::time::Duration;

//-------------------------------------------------------------------------------------------------------------------

/// Sleep for the specified duration.
///
/// On native builds, this should be executed from within a tokio runtime, otherwise it will block!
pub async fn sleep(duration: Duration)
{
    #[cfg(not(target_family = "wasm"))]
    {
        // try tokio sleep
        if let Ok(_) = tokio::runtime::Handle::try_current()
        {
            tokio::time::sleep(duration).await;
            return;
        }

        // fallback
        std::thread::sleep(duration);
        return;
    }

    #[cfg(target_family = "wasm")]
    {
        wasmtimer::tokio::sleep(duration).await;
        return;
    }
}

//-------------------------------------------------------------------------------------------------------------------
