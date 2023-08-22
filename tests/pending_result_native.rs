//local shortcuts

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[test]
fn pending_result_native_io()
{
    // make cpu-oriented task
    dbg!("task");
    let task = async { dbg!("task ran"); };

    // spawn task
    let mut pending_result = enfync::defaults::IOPendingResult::<()>::new(
            &enfync::defaults::IOHandle::default().into(),
            task
        );

    // wait for task
    let enfync::Result::Ok(_) = pending_result.extract() else { panic!(""); };
}

//-------------------------------------------------------------------------------------------------------------------

#[test]
fn pending_result_native_cpu()
{
    // make cpu-oriented task
    dbg!("task");
    let task = async { dbg!("task ran"); };

    // spawn task
    let mut pending_result = enfync::defaults::CPUPendingResult::<()>::new(
            &enfync::defaults::CPUHandle::default().into(),
            task
        );

    // wait for task
    let enfync::Result::Ok(_) = pending_result.extract() else { panic!(""); };
}

//-------------------------------------------------------------------------------------------------------------------
