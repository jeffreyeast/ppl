use crate::{tests::tests::run, workspace::WorkSpace};


#[test]
fn chapter11() {
    let workspace = WorkSpace::new();

    // Page 19

    assert_eq!("", run(
        r#"$remain(a,b)
        remain_a-b*int(a/b)
        $"#, &workspace));
        assert_eq!("2.", run("remain(5.0,3.0)", &workspace));
        run(r#"binary("\", remain)"#, &workspace);
        assert_eq!("26.3", run(r#"iformat("2z.1d", (34.2+547.1)\37.0)"#, &workspace));
}
