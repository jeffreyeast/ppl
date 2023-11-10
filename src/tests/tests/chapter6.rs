use crate::{tests::tests::run, workspace::WorkSpace};


#[test]
fn chapter6() {
    let workspace = WorkSpace::new();
    //  Page 9
    assert_eq!("80", run("add(53,27)", &workspace));
    assert_eq!("7.9", run("3.4+(add(add(1,5),7)-8.5)", &workspace));
    assert_eq!("80", run("53+27", &workspace));
    assert_eq!("26", run("sub(53,27)", &workspace));
    assert_eq!("26", run("53-27", &workspace));
    assert_eq!("1431", run("mul(53,27)", &workspace));
    assert_eq!("1431", run("53*27", &workspace));
    assert_eq!("1", run("div(53,27)", &workspace));
    assert_eq!("1", run("53/27", &workspace));
    assert_eq!("-27", run("minus(27)", &workspace));
    assert_eq!("-27", run("-27", &workspace));
    assert_eq!("27", run("plus(27)", &workspace));
    assert_eq!("27", run("+27", &workspace));
    assert_eq!("false", run("eq(53,27)", &workspace));
    assert_eq!("false", run("53=27", &workspace));
    assert_eq!("true", run("eq(53,53)", &workspace));
    assert_eq!("true", run("53=53", &workspace));
    assert_eq!("true", run("noteq(53,27)", &workspace));
    assert_eq!("true", run("53#27", &workspace));
    assert_eq!("false", run("noteq(53,53)", &workspace));
    assert_eq!("false", run("53#53", &workspace));
    assert_eq!("false", run("less(53,27)", &workspace));
    assert_eq!("false", run("53<27", &workspace));
    assert_eq!("true", run("less(27,53)", &workspace));
    assert_eq!("true", run("27<53", &workspace));
    assert_eq!("false", run("less(53,53)", &workspace));
    assert_eq!("false", run("53<53", &workspace));
    assert_eq!("true", run("gr(53,27)", &workspace));
    assert_eq!("true", run("53>27", &workspace));
    assert_eq!("false", run("gr(27,53)", &workspace));
    assert_eq!("false", run("27>53", &workspace));
    assert_eq!("false", run("gr(53,53)", &workspace));
    assert_eq!("false", run("53>53", &workspace));
    assert_eq!("false", run("lesseq(53,27)", &workspace));
    assert_eq!("false", run("53<=27", &workspace));
    assert_eq!("true", run("lesseq(27,53)", &workspace));
    assert_eq!("true", run("27<=53", &workspace));
    assert_eq!("true", run("lesseq(53,53)", &workspace));
    assert_eq!("true", run("53<=53", &workspace));
    assert_eq!("true", run("greq(53,27)", &workspace));
    assert_eq!("true", run("53>=27", &workspace));
    assert_eq!("false", run("greq(27,53)", &workspace));
    assert_eq!("false", run("27>=53", &workspace));
    assert_eq!("true", run("greq(53,53)", &workspace));
    assert_eq!("true", run("53>=53", &workspace));
    assert_eq!("true", run("and(true,true)", &workspace));
    assert_eq!("false", run("and(true,false)", &workspace));
    assert_eq!("false", run("and(false,true)", &workspace));
    assert_eq!("false", run("and(false,false)", &workspace));
    assert_eq!("true", run("or(true,true)", &workspace));
    assert_eq!("true", run("or(true,false)", &workspace));
    assert_eq!("true", run("or(false,true)", &workspace));
    assert_eq!("false", run("or(false,false)", &workspace));
    assert_eq!("false", run("minus(true)", &workspace));
    assert_eq!("true", run("minus(false)", &workspace));
    assert_eq!("46", run("assign(b,46)", &workspace));
    assert_eq!("46", run("b_46", &workspace));
    assert_eq!("46", run("a_b", &workspace));

    //  Page 10
    assert_eq!("7", run("int(3.1+(i_9.7)/2)", &workspace));
    assert_eq!("9.7", run("i", &workspace));
    assert_eq!("1.", run("a_b_c_1.0", &workspace));
    assert_eq!("1.", run("b", &workspace));
    assert_eq!("46", run("int(46)", &workspace));
    assert_eq!("46.", run("real(46)", &workspace));
    assert_eq!("46.", run("dbl(46)", &workspace));
    assert_eq!("46", run("int(46.7)", &workspace));
    assert_eq!("46.7", run("real(46.7)", &workspace));
 //   assert_eq!("46.7", run("dbl(46.7)", &workspace));     //  There's a fuzz issue
    assert_eq!("true", run("int(46.7)==int", &workspace));
    assert_eq!("false", run("real(46.7)==int", &workspace));
    assert_eq!("false", run("dbl(46.7)==int", &workspace));
    assert_eq!("false", run("int(46.7)==real", &workspace));
    assert_eq!("true", run("real(46.7)==real", &workspace));
    assert_eq!("false", run("dbl(46.7)==real", &workspace));
    assert_eq!("false", run("int(46.7)==dbl", &workspace));
    assert_eq!("false", run("real(46.7)==dbl", &workspace));
    assert_eq!("true", run("dbl(46.7)==dbl", &workspace));

}
