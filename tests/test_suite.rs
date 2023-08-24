//local shortcuts

//third-party shortcuts
use enfync::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

fn basic_extract<Recv, H>()
where
    Recv: ResultReceiver<Result = i32>,
    H: Into<<Recv as ResultReceiver>::Spawner> + Default,
{
    // make task
    dbg!("test: basic_extract");
    let val = 10;
    let task = async move { dbg!("task ran"); val };

    // spawn task
    let mut pending_result = PendingResult::<Recv>::new(&H::default().into(), task);

    // wait for task
    let Ok(res) = pending_result.extract() else { panic!(""); };
    assert_eq!(res, val);
}

//-------------------------------------------------------------------------------------------------------------------

fn basic_try_extract<Recv, H>()
where
    Recv: ResultReceiver<Result = i32>,
    H: Into<<Recv as ResultReceiver>::Spawner> + Default,
{
    // make task
    dbg!("test: basic_try_extract");
    let val = 10;
    let task = async move { dbg!("task ran"); val };

    // spawn task
    let mut pending_result = PendingResult::<Recv>::new(&H::default().into(), task);

    // wait for async machinery
    std::thread::sleep(std::time::Duration::from_millis(15));

    // wait for task
    let Some(Ok(res)) = pending_result.try_extract() else { panic!(""); };
    assert_eq!(res, val);
}

//-------------------------------------------------------------------------------------------------------------------

fn test_suite_impl<Recv, H>()
where
    Recv: ResultReceiver<Result = i32>,
    H: Into<<Recv as ResultReceiver>::Spawner> + Default,
{
    basic_extract::<Recv, H>();
    basic_try_extract::<Recv, H>();
}

//-------------------------------------------------------------------------------------------------------------------

fn test_suite()
{
    dbg!("test suite IO");
    test_suite_impl::<enfync::builtin::IOReceiver::<i32>, enfync::builtin::IOHandle>();
    dbg!("test suite CPU");
    test_suite_impl::<enfync::builtin::CPUReceiver::<i32>, enfync::builtin::CPUHandle>();

    dbg!("test suite MIXED");
    //todo
}

//-------------------------------------------------------------------------------------------------------------------

#[test]
fn test_core_native()
{
    test_suite();
}

//-------------------------------------------------------------------------------------------------------------------

#[cfg(wasm)]
#[wasm_bindgen_test]
fn test_core_wasm()
{
    test_suite();
}

//-------------------------------------------------------------------------------------------------------------------
