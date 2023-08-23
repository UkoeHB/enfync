//local shortcuts

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[test]
fn pending_result_native_io()
{
    // make cpu-oriented task
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
fn pending_result_native_cpu()
{
    // make cpu-oriented task
    dbg!("task");
    let val = 10;
    let task = async move { dbg!("task ran"); val };

    // spawn task
    let mut pending_result = enfync::builtin::CPUPendingResult::new(
            &enfync::builtin::CPUHandle::default().into(),
            task
        );

    // wait for task
    let Ok(res) = pending_result.extract() else { panic!(""); };
    assert_eq!(res, val);
}

//-------------------------------------------------------------------------------------------------------------------
