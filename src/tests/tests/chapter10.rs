use crate::{tests::tests::run, workspace::WorkSpace};


#[test]
fn chapter10() {
    let workspace = WorkSpace::new();

    // Page 19

    assert_eq!("", run(
        r#"$ftest
        ftest_iformat("3z.4d", 7.41) 
        ftest_concat(ftest,"\n")
        ftest_concat(ftest,iformat("3d.4z", 7.41)) 
        ftest_concat(ftest,"\n")
        ftest_concat(ftest,iformat("e1z2d.4d", 7.41)) 
        $"#, &workspace));
        assert_eq!(r#"  7.4100
007.41  
 74.1000e-01"#, run("ftest", &workspace));
        assert_eq!("100000",run(r#"iformat("ff", 100000)"#, &workspace));
        assert_eq!("100000.",run(r#"iformat("ff", 100000.)"#, &workspace));
        assert_eq!("   1",run(r#"iformat("3z1d", 1.)"#, &workspace));
        assert_eq!(" 100",run(r#"iformat("3z1d", 100.)"#, &workspace));
        assert_eq!("-100",run(r#"iformat("3z1d", -100)"#, &workspace));
        assert_eq!(" 123",run(r#"iformat("3z1d", 123.456)"#, &workspace));
        assert_eq!("-123",run(r#"iformat("3z1d", -123.456)"#, &workspace));
        assert_eq!(" 124",run(r#"iformat("3z1d", 123.556)"#, &workspace));
        assert_eq!("-123",run(r#"iformat("3z1d", -123.556)"#, &workspace));
        assert_eq!(" 123.46",run(r#"iformat("3z1d.2d", 123.456)"#, &workspace));
        assert_eq!("-123.46",run(r#"iformat("3z1d.2d", -123.456)"#, &workspace));
        assert_eq!(" 123.56",run(r#"iformat("3z1d.2d", 123.556)"#, &workspace));
        assert_eq!("-123.55",run(r#"iformat("3z1d.2d", -123.556)"#, &workspace));
        assert_eq!(" 123.43",run(r#"iformat("3z1d.2d", 123.432)"#, &workspace));
        assert_eq!("-123.43",run(r#"iformat("3z1d.2d", -123.432)"#, &workspace));
        assert_eq!(" 123.46e 00",run(r#"iformat("e3z1d.2d", 123.456)"#, &workspace));
        assert_eq!("-123.46e 00",run(r#"iformat("e3z1d.2d", -123.456)"#, &workspace));
        assert_eq!("1.23e 02",run(r#"iformat("e1d.2d", 123.456)"#, &workspace));
        assert_eq!("12.35e 01",run(r#"iformat("e1z1d.2d", 123.456)"#, &workspace));
        assert_eq!("-1.23e 02",run(r#"iformat("e1z1d.2d", -123.456)"#, &workspace));
}
