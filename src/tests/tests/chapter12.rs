use crate::{tests::tests::{run, run_divert_stdout}, workspace::WorkSpace};


#[test]
fn chapter12() {
    let workspace = WorkSpace::new();

        // Page 29

        run("$complex=[rp:real, ip:real]", &workspace);

        // Page 30

        assert_eq!("[rp:3.7, ip:59.]",run(r#"x_complex(3.7,59.0)"#, &workspace));
        assert_eq!("3.7",run(r#"r_rp(x)"#, &workspace));
        assert_eq!("72.",run(r#"13+x[ip]"#, &workspace));
        assert_eq!("true",run(r#"x==complex"#, &workspace));
        assert_eq!("false",run(r#"3.427==complex"#, &workspace));
        assert_eq!("true",run(r#"3==int"#, &workspace));
        assert_eq!("false",run(r#"4.529==int"#, &workspace));
        assert_eq!("1.1937E2",run(r#"v_93+(45.2/6)*3.5"#, &workspace));
        assert_eq!("true",run(r#"v==real"#, &workspace));
        run("$row=[1:]real", &workspace);

        // Page 31

        assert_eq!("[3.56, -14.295, 41.4075]",run(r#"x_row(3.56, -28.59/2, 3*55.21/4)"#, &workspace));
        assert_eq!("3.56",run(r#"x[1]"#, &workspace));
        assert_eq!("3",run(r#"i_3"#, &workspace));
        assert_eq!("-14.295",run(r#"q_x[i-1]"#, &workspace));
        assert_eq!("1.3556E1",run(r#"(q+x[i])/2"#, &workspace));
        run("erase(row)", &workspace);
        run("$row=[-10:]real", &workspace);
        assert_eq!("[3.2, -1.295, 33.9075]",run(r#"x_row(3.2, -2.59/2, 3*45.21/4)"#, &workspace));
        assert_eq!("true",run(r#"x==row"#, &workspace));
        assert_eq!("false",run(r#"x==complex"#, &workspace));

        // Page 32

        run("erase(row)", &workspace);
        run("$row=[1:4]real", &workspace);
        assert_eq!("[1., 3.6, -5., 12.2]",run(r#"x_row(1, 3.6, -5.,2*6.1)"#, &workspace));
        run("$arith2=int!real!dbl", &workspace);
        assert_eq!("true",run(r#"(3.276*54)==arith2"#, &workspace));
        assert_eq!("false",run(r#"x==arith2"#, &workspace));
        assert_eq!("true",run(r#"(x[3]/x[2])==arith2"#, &workspace));
        run("$vector=[1:]arith2", &workspace);
        assert_eq!("[3, 7.9, 0.22]",run(r#"x_vector(3, 7.9, 1.54/7)"#, &workspace));
        assert_eq!("3",run(r#"x[1]"#, &workspace));
        assert_eq!("true",run(r#"x[1]==int"#, &workspace));
        assert_eq!("false",run(r#"x[1]==real"#, &workspace));
        assert_eq!("true",run(r#"x[1]==arith2"#, &workspace));
        assert_eq!("true",run(r#"x[3]==real"#, &workspace));
        assert_eq!("true",run(r#"x[3]==arith2"#, &workspace));

        // Page 33

        assert_eq!(r#"[3, Q, [4.1, abcde]]"#,run(r#"x_[3, 'Q, [4.1, "abcde"]]"#, &workspace));
        assert_eq!("[true]", run("y_[true]", &workspace));
        assert_eq!("[]", run("z_[]", &workspace));
        assert_eq!("[3, 7.9, 0.22]", run("x_vector(3, 7.9, 1.54/7)", &workspace));
        assert_eq!("[3, 7.9, 0.22]", run("x", &workspace));
        assert_eq!("[rp:5.7, ip:-32.4]", run("complex(5.7, -32.4)", &workspace));
        run("$array=[1:]vector", &workspace);

        // Page 34

        assert_eq!("[[3, 7.9, 0.22], [3, 2, 1]]", run("a_array(x,vector(3,2,1))", &workspace));
        assert_eq!("1", run("q_1", &workspace));
        assert_eq!("[3, 2, 1]", run("a[q+1]", &workspace));
        assert_eq!("[3, 7.9, 0.22]", run("x", &workspace));
        assert_eq!("8.8540E-26", run("x[2]_88.54e-27", &workspace));
        assert_eq!("8.8540E-26", run("x[2]", &workspace));
        assert_eq!("[3, 8.8540E-26, 0.22]", run("x", &workspace));
        assert_eq!("[[3, 7.9, 0.22], [3, 2, 1]]", run("a", &workspace));
        assert_eq!("[3, 2, 1]", run("a[1]_a[2]", &workspace));
        assert_eq!("[[3, 2, 1], [3, 2, 1]]", run("a", &workspace));
        assert_eq!("[[11, 12, 13], [21, 22, 23], [31, 32, 33]]", run("x_array(vector(11,12,13),vector(21,22,23),vector(31,32,33))", &workspace));
        assert_eq!("[[11, 12, 13], [21, 22, 23], [31, 32, 33]]", run("x", &workspace));
        assert_eq!("13", run("(x[1])[3]", &workspace));
        assert_eq!("13", run("x[1][3]", &workspace));
        assert_eq!("32", run("x[3][2]", &workspace));
        assert_eq!("32", run("x[3,2]", &workspace));

        // Page 35

        run("$num=arith2 ! complex", &workspace);
        run("$numseq=[1:]num", &workspace);
        assert_eq!("[3.42, [rp:8.1, ip:-33.], -5]", run("x_numseq(3.42,complex(8.1,-33.0),-5)", &workspace));
        assert_eq!("[3.42, [rp:8.1, ip:-33.], -5]", run("x", &workspace));
        assert_eq!("-33.", run("x[2][ip]", &workspace));
        assert_eq!("-33.", run("x[2,ip]", &workspace));
        assert_eq!("8.1", run("rp(x[2])", &workspace));
        assert_eq!("G", run(r#""STRING CONSTANT"[6]"#, &workspace));
        assert_eq!("STRING", run(r#"x_"STRING""#, &workspace));
        assert_eq!("STRING", run(r#"x"#, &workspace));
        assert_eq!("U", run(r#"x[4]_'U"#, &workspace));
        assert_eq!("STRUNG", run(r#"x"#, &workspace));
        assert_eq!("true", run(r#"x==string"#, &workspace));
        assert_eq!("false", run(r#"x[4]==string"#, &workspace));
        assert_eq!("true", run(r#"x[4]==char"#, &workspace));

        // Page 36

        run(r#"$form = uf ! bf ! atom"#, &workspace);
        run(r#"$bf = [lo:form, op:char, ro:form]"#, &workspace);
        run(r#"$uf = [op:char, ro:form]"#, &workspace);
        run(r#"$atom = string ! char ! real ! int ! dbl"#, &workspace);
        assert_eq!("[lo:x, op:+, ro:3]", run(r#"f_bf('x, '+, 3)"#, &workspace));
        assert_eq!("[op:-, ro:[lo:x, op:*, ro:[lo:x, op:+, ro:3]]]", run(r#"g_uf('-, bf('x, '*, f))"#, &workspace));
        run(r#"$deriv(f:general, x:general)
        (f==atom)-->a
        (f==bf)-->b
        (f==uf)-->u
        a: (f=x)--> %7
        deriv_0
        -->%0
        deriv_1
        -->%0
        b: (f[op]='+)-->pl
        (f[op]='-)-->mi
        (f[op]='*)-->ti
        (f[op]='/)-->di
        pl: deriv_bf(deriv(f[lo],x),'+,deriv(f[ro],x))
        -->0
        mi: deriv_bf(deriv(f[lo],x),'-,deriv(f[ro],x))
        -->0
        ti: deriv_bf(bf(f[lo], '*, deriv(f[ro],x)),'+, bf(deriv(f[lo],x),'*,f[ro]))
        -->0
        di: deriv_bf(bf(bf(deriv(f[lo],x),'*,f[ro]),'-,bf(f[lo],'*,deriv(f[ro],x))),'/,bf(f[ro],'*,f[ro]))
        -->0
        u: deriv_uf(f[op],deriv(f[ro],x))
        $"#, &workspace);
        run(r#"$pf(f:general)
        (f==uf)-->5
        (f==bf)-->7
        print(f)
        -->0
        print('()
        -->9
        print('()
        pf(f[lo])
        print(f[op])
        pf(f[ro])
        print('))
        $"#, &workspace);

        // Page 37

        assert_eq!("[lo:1, op:+, ro:0]", run("deriv(f,'x)", &workspace));
        assert_eq!("[op:-, ro:[lo:[lo:x, op:*, ro:[lo:1, op:+, ro:0]], op:+, ro:[lo:1, op:*, ro:[lo:x, op:+, ro:3]]]]", run("deriv(g,'x)", &workspace));

        assert_eq!("3string932700.q", run_divert_stdout(r#"print(3, "string", 93.27e+4, 'q)"#, &workspace));
     
        // Page 38

        assert_eq!("(1+0)", run_divert_stdout("pf(deriv(f,'x))", &workspace));
        assert_eq!("(-(x*(x+3)))", run_divert_stdout("pf(g)", &workspace));
        assert_eq!("(-((x*(1+0))+(1*(x+3))))", run_divert_stdout("pf(deriv(g,'x))", &workspace));
        run(r#"$cadd(a:general, b:general)
        (a==complex)-->7
        (b==complex)-->5
        cadd_add(a,b)
        -->0
        cadd_complex(add(a,b[rp]),b[lp])
        -->0
        (b==complex)-->10
        cadd_complex(add(a[rp],b),a[ip])
        -->0
        cadd_complex(add(a[rp],b[rp]),add(a[ip],b[ip]))
        $"#, &workspace);
        run(r#"binary("+", cadd)"#, &workspace);
        assert_eq!("[rp:8.37, ip:-2.1]", run("x_complex(8.37, -2.1)", &workspace));
        assert_eq!("[rp:-11400000., ip:-32.85]", run("y_complex(-1.14e+7, -32.85)", &workspace));
        assert_eq!("[rp:-11399992., ip:-3.4950E1]", run("x+y", &workspace));
        assert_eq!("[rp:11.37, ip:-2.1]", run("x+3", &workspace));
        assert_eq!("[rp:-11399991., ip:-3.4950E1]", run("x+y+1.47", &workspace));
        assert_eq!("49", run("42+7", &workspace));

    }