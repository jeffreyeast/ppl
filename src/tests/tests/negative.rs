use crate::{tests::tests::run, workspace::WorkSpace};


#[test]
fn negative() {
    let workspace = WorkSpace::new();

    assert_eq!("", run("foo(bar)", &workspace));
    assert_eq!("", run("a_b", &workspace));
    assert_eq!("", run("a_b[1]", &workspace));
    assert_eq!("", run("a[1]_b", &workspace));
    assert_eq!("", run("a_$b", &workspace));
    assert_eq!("", run("a_$b[2]", &workspace));
    assert_eq!("", run("a[1]_$b", &workspace));
    assert_eq!("", run("a[1]_$b[2]", &workspace));

}