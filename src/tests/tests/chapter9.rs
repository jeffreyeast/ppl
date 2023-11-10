use crate::{tests::tests::run, workspace::WorkSpace};


#[test]
fn chapter9() {
    let workspace = WorkSpace::new();

    // Page 19

    assert_eq!("", run(
        r#"$max(a,b)
        (a>b)-->its.a
        max_b
        -->0
        its.a: max_a
        $"#, &workspace));
   assert_eq!("6", run("max(1,6)", &workspace));
   assert_eq!("6", run("max(6,1)", &workspace));
   assert_eq!("5.2", run("max(1,5.2)", &workspace));
   assert_eq!("5.2", run("max(5.2,1.0)", &workspace));
   assert_eq!("dog", run("max(\"dog\",\"cat\")", &workspace));
   assert_eq!("dog", run("max(\"cat\",\"dog\")", &workspace));

   // Page 23

   assert_eq!("", run(
    r#"$ssum(n,p)
    ssum_0
    again: ssum_ssum+n^p
    ((n_n-1)>0)-->again
    $"#, &workspace));
    assert_eq!("14", run("ssum(3,2)", &workspace));
    assert_eq!("30", run("ssum(4,2)", &workspace));


}
