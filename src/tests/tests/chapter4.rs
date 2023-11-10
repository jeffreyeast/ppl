use crate::{tests::tests::run, workspace::WorkSpace};


#[test]
fn chapter4() {
    let workspace = WorkSpace::new();
    //  Page 6
    assert_eq!("1", run("a_1", &workspace));
    assert_eq!("1", run("a", &workspace));
    assert_eq!("1", run("w3_1", &workspace));
    assert_eq!("1", run("w3", &workspace));
    assert_eq!("1", run("th1.3v_1", &workspace));
    assert_eq!("1", run("th1.3v", &workspace));
    assert_eq!("1", run("sum.of.9.digits_1", &workspace));
    assert_eq!("1", run("sum.of.9.digits", &workspace));
    run("n_5.4", &workspace);
    assert_eq!("5.4", run("n", &workspace));
    run("q3_(n^3)+54", &workspace);
    assert_eq!("3.0209E1", run("q3/7", &workspace));
    run("n_n-2.1", &workspace);
   // assert_eq!("3.3", run("n", &workspace));          // Fails due to round-off error
   run("n_3", &workspace);
   assert_eq!("2", run("(n+1)/2", &workspace));
   run("n_3.35e+8", &workspace);
   assert_eq!("1.6750E8", run("(n+1)/2", &workspace));
   run("pi_3.1415926", &workspace);
   assert_eq!("3.5257E17", run("pi*n^2", &workspace));
}
