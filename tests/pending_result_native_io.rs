//local shortcuts

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[test]
fn io_basic_extract()
{
    // make task
    dbg!("task");
    let val = 10;
    let task = async move { dbg!("task ran"); val };

    // spawn task
    let mut pending_result = enfync::builtin::IOPendingResult::new(
            &enfync::builtin::IOHandle::default().into(),
            task
        );

    // wait for task
    let Ok(res) = pending_result.extract() else { panic!(""); };
    assert_eq!(res, val);
}

//-------------------------------------------------------------------------------------------------------------------

#[test]
fn io_basic_try_extract()
{
    // make task
    dbg!("task");
    let val = 10;
    let task = async move { dbg!("task ran"); val };

    // spawn task
    let mut pending_result = enfync::builtin::IOPendingResult::new(
            &enfync::builtin::IOHandle::default().into(),
            task
        );

    // wait for async machinery
    std::thread::sleep(std::time::Duration::from_millis(15));

    // wait for task
    let Some(Ok(res)) = pending_result.try_extract() else { panic!(""); };
    assert_eq!(res, val);
}

//-------------------------------------------------------------------------------------------------------------------
