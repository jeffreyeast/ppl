use crate::{tests::tests::{run, run_divert_stdout}, workspace::WorkSpace};


#[test]
fn extensions() {
    let workspace = WorkSpace::new();

    //  if

    assert_eq!("", run(
        r#"$f1(n)
        if n=1
        print("n=1")
        else
        print("n!=1")
        $"#, &workspace));
    assert_eq!("n=1", run_divert_stdout("f1(1)", &workspace));
    assert_eq!("n!=1", run_divert_stdout("f1(2)", &workspace));
    assert_eq!("", run(
        r#"$f2(n)
        if n=1
        {
        print("n=1")
        }
        else
        {
        print("n!=1")
        }
        $"#, &workspace));
    assert_eq!("n=1", run_divert_stdout("f2(1)", &workspace));
    assert_eq!("n!=1", run_divert_stdout("f2(2)", &workspace));
    assert_eq!("", run(
        r#"$f3(n)
        if n=1
        print("n=1")
        print("n!=1")
        $"#, &workspace));
    assert_eq!("n=1n!=1", run_divert_stdout("f3(1)", &workspace));
    assert_eq!("n!=1", run_divert_stdout("f3(2)", &workspace));
      
}