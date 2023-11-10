use crate::{tests::tests::run, workspace::WorkSpace};


#[test]
fn chapter13() {
    let workspace = WorkSpace::new();

        // Page 41

        run("$foo=[1:3]general", &workspace);
        assert_eq!("[1, 2, 3]",run(r#"y_foo(1,2,3)"#, &workspace));
        assert_eq!("[1, [1, ..., 3], 3]",run(r#"y[2]__y"#, &workspace));
        assert_eq!("[1, [1, ..., 3], 3]",run(r#"y"#, &workspace));
        assert_eq!("99",run(r#"y[2]_99"#, &workspace));
        assert_eq!("[1, 99, 3]",run(r#"y"#, &workspace));
        run(r#"$pntr=[val:general]"#, &workspace);
        run(r#"$mkref($x)
        mkref_pntr(0)
        mkref[val]__x
        $"#, &workspace);
        run(r#"x_1"#, &workspace);
        assert_eq!("[val:1]",run(r#"r_mkref(x)"#, &workspace));
        assert_eq!("[val:1]",run(r#"r"#, &workspace));
        assert_eq!("1",run(r#"x"#, &workspace));

        // Page 42

        run(r#"$cbv(a,b,c)
        a_3.4
        b[3]_5
        c_c+1
        $"#, &workspace);
        run("x_1.0", &workspace);
        run("y_[1,2,3,4]", &workspace);
        run("z_5", &workspace);
        run("cbv(x,y,z)", &workspace);
        assert_eq!("1.",run(r#"x"#, &workspace));
        assert_eq!("[1, 2, 3, 4]",run(r#"y"#, &workspace));
        assert_eq!("5",run(r#"z"#, &workspace));
        run(r#"$cbr($a,$b,$c)
        a_3.4
        b[3]_5
        c_c+1
        $"#, &workspace);
        run("cbr(x,y,z)", &workspace);
        assert_eq!("3.4",run(r#"x"#, &workspace));
        assert_eq!("[1, 2, 5, 4]",run(r#"y"#, &workspace));
        assert_eq!("6",run(r#"z"#, &workspace));
        run(r#"$bar
        bar_1976
        $"#, &workspace);
        assert_eq!("1976",run(r#"bar"#, &workspace));
        assert_eq!("",run(r#"erase(bar)"#, &workspace));
        assert_eq!("",run(r#"bar"#, &workspace));
        run(r#"$bar
        bar_1976
        $"#, &workspace);
        assert_eq!("1976",run(r#"bar"#, &workspace));
        assert_eq!("",run(r#"erase($bar)"#, &workspace));
        assert_eq!("",run(r#"bar"#, &workspace));

        // Page 43

        run("x_1.0", &workspace);
        run("y_[1,2,3,4]", &workspace);
        run("z_5", &workspace);
        assert_eq!("1.",run(r#"x"#, &workspace));
        assert_eq!("[1, 2, 3, 4]",run(r#"y"#, &workspace));
        assert_eq!("5",run(r#"z"#, &workspace));
        assert_eq!("",run(r#"erase(x,y)"#, &workspace));
        assert_eq!("",run(r#"x"#, &workspace));
        assert_eq!("",run(r#"y"#, &workspace));
        assert_eq!("5",run(r#"z"#, &workspace));
        run("x_1.0", &workspace);
        run("y_[1,2,3,4]", &workspace);
        assert_eq!("1.",run(r#"x"#, &workspace));
        assert_eq!("[1, 2, 3, 4]",run(r#"y"#, &workspace));
        assert_eq!("",run(r#"erase"#, &workspace));
        assert_eq!("",run(r#"x"#, &workspace));
        assert_eq!("",run(r#"y"#, &workspace));
        assert_eq!("",run(r#"z"#, &workspace));


}