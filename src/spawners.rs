//local shortcuts

//third-party shortcuts

//standard shortcuts
use std::fmt::Debug;

//-------------------------------------------------------------------------------------------------------------------

pub trait OneshotSpawner: Debug + Send + Sync + 'static
{
    fn spawn<F>(&self, task: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static;
}

//-------------------------------------------------------------------------------------------------------------------

pub trait SimpleSpawner<R>: Debug + Send + Sync + 'static
{
    type Error: Debug + Send + 'static;
    type Future: std::future::Future<Output = Result<R, Self::Error>> + Debug + Send + 'static;

    fn spawn<F>(&self, task: F) -> Self::Future
    where
        F: std::future::Future<Output = R> + Send + 'static;

    fn is_done(f: &Self::Future) -> bool;
}

impl<Ss: SimpleSpawner::<()>> OneshotSpawner for Ss
{
    fn spawn<F>(&self, task: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.spawn(task);  //detach future (assume oneshots use a oneshot channel)
    }
}

//-------------------------------------------------------------------------------------------------------------------
