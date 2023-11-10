use crate::{tests::tests::run, workspace::WorkSpace};


#[test]
fn chapter5() {
    let workspace = WorkSpace::new();
    //  Page 7
    run("c_'Q", &workspace);
    assert_eq!("Q", run("c", &workspace));
    assert_eq!("I AM A CHARACTER STRING", run(r#""I AM A CHARACTER STRING""#, &workspace));
    assert_eq!(r#"HE SAID "FOO"."#, run(r#"qq_"HE SAID ""FOO"".""#, &workspace));
    assert_eq!("1", run("a_1", &workspace));
    assert_eq!("1", run("b_1", &workspace));
    assert_eq!("2", run("c_2", &workspace));
    assert_eq!("2", run("d_2", &workspace));
    assert_eq!("0", run("e_0", &workspace));
    assert_eq!("true", run("a=b", &workspace));
    assert_eq!("false", run("a=c", &workspace));
    assert_eq!("true", run("a<c", &workspace));
    assert_eq!("false", run("a<b", &workspace));
    assert_eq!("true", run("a<=b", &workspace));
    assert_eq!("true", run("a<=c", &workspace));
    assert_eq!("true", run("c>a", &workspace));
    assert_eq!("false", run("b>a", &workspace));
    assert_eq!("true", run("c>=a", &workspace));
    assert_eq!("true", run("c>=d", &workspace));
    assert_eq!("true", run("a#c", &workspace));
    assert_eq!("false", run("a#b", &workspace));

    // Page 8
    assert_eq!("true", run("true&true", &workspace));
    assert_eq!("false", run("true&false", &workspace));
    assert_eq!("false", run("false&true", &workspace));
    assert_eq!("false", run("false&false", &workspace));
    assert_eq!("true", run("true!true", &workspace));
    assert_eq!("true", run("true!false", &workspace));
    assert_eq!("true", run("false!true", &workspace));
    assert_eq!("false", run("false!false", &workspace));
    assert_eq!("false", run("-true", &workspace));
    assert_eq!("true", run("-false", &workspace));
}
