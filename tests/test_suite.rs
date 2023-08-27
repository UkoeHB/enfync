//local shortcuts

//third-party shortcuts
#[cfg(not(target_family = "wasm"))]
use enfync::HandleTrait;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn print_dbg(s: &str)
{
    #[cfg(not(target_family = "wasm"))]
    dbg!(s);

    #[cfg(target_family = "wasm")]
    web_sys::console::log_1(&s.into());
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

async fn basic_try_extract<H: enfync::HandleTrait>()
{
    // make task
    print_dbg("test: basic_try_extract");
    let val = 10;
    let task = async move { print_dbg("task ran"); val };

    // spawn task
    print_dbg("test: basic_try_extract... spawning");
    let mut pending_result = H::default().spawn(task);

    // wait for async machinery
    print_dbg("test: basic_try_extract... sleeping");
    enfync::sleep(std::time::Duration::from_millis(15)).await;

    // wait for task
    print_dbg("test: basic_try_extract... extracting");
    let Some(Ok(res)) = pending_result.try_extract() else { panic!(""); };
    assert_eq!(res, val);
}

//-------------------------------------------------------------------------------------------------------------------

async fn basic_nesting<H: enfync::HandleTrait>()
{
    // make task
    print_dbg("test: basic_nesting");
    let val = 10;
    let task = async move { print_dbg("task ran"); val };

    // spawn task
    print_dbg("test: basic_nesting... spawning");
    let mut pending_result = H::default().spawn(task);

    // make new task waiting for other task
    print_dbg("test: basic_nesting... new task");
    let mut pending_result = H::default().spawn(async move { pending_result.extract().await });

    print_dbg("test: basic_nesting... waiting");
    let Ok(Ok(res)) = pending_result.extract().await else { panic!(""); };
    assert_eq!(res, val);
}

//-------------------------------------------------------------------------------------------------------------------

async fn test_suite_impl<H: enfync::HandleTrait>()
{
    basic_try_extract::<H>().await;
    basic_nesting::<H>().await;
}

//-------------------------------------------------------------------------------------------------------------------

async fn test_suite()
{
    print_dbg("test suite");
    test_suite_impl::<enfync::builtin::Handle>().await;
}

//-------------------------------------------------------------------------------------------------------------------

#[cfg(not(target_family = "wasm"))]
#[test]
fn test_core_native()
{
    enfync::builtin::Handle::default().spawn( async { test_suite().await; });
}

//-------------------------------------------------------------------------------------------------------------------

#[cfg(target_family = "wasm")]
#[wasm_bindgen_test::wasm_bindgen_test]
async fn test_core_wasm()
{
    web_sys::console::log_1(&"wasm test running".into());
    test_suite().await;
}

//-------------------------------------------------------------------------------------------------------------------
