//local shortcuts

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

fn basic_extract<H: enfync::Handle>()
{
    // make task
    dbg!("test: basic_extract");
    let val = 10;
    let task = async move { dbg!("task ran"); val };

    // spawn task
    let mut pending_result = H::default().spawn(task);

    // wait for task
    let Ok(res) = pending_result.extract() else { panic!(""); };
    assert_eq!(res, val);
}

//-------------------------------------------------------------------------------------------------------------------

fn basic_try_extract<H: enfync::Handle>()
{
    // make task
    dbg!("test: basic_try_extract");
    let val = 10;
    let task = async move { dbg!("task ran"); val };

    // spawn task
    let mut pending_result = H::default().spawn(task);

    // wait for async machinery
    std::thread::sleep(std::time::Duration::from_millis(15));

    // wait for task
    let Some(Ok(res)) = pending_result.try_extract() else { panic!(""); };
    assert_eq!(res, val);
}

//-------------------------------------------------------------------------------------------------------------------

fn test_suite_impl<H: enfync::Handle>()
{
    basic_extract::<H>();
    basic_try_extract::<H>();
}

//-------------------------------------------------------------------------------------------------------------------

fn test_suite()
{
    dbg!("test suite IO");
    test_suite_impl::<enfync::builtin::IOHandle>();
    dbg!("test suite CPU");
    test_suite_impl::<enfync::builtin::CPUHandle>();

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
