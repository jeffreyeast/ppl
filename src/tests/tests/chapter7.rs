use crate::{tests::tests::run, workspace::WorkSpace};


#[test]
fn chapter7() {
    let workspace = WorkSpace::new();
    //  Page 11
    assert_eq!("", run(
r#"$fact(n); i
fact_1.0
i_n
(i<=0)-->%0
fact_fact*i
i_i-1
-->%3
$"#, &workspace));

    assert_eq!("6.", run("fact(3)", &workspace));
    assert_eq!("3628800.", run("fact(10)", &workspace));
    assert_eq!("6.2045E23", run("25+fact(fact(4))", &workspace));
    assert_eq!("", run(
        r#"$rfact(n)
        rfact_1.0
        (n=0)-->%0
        rfact_n*rfact(n-1)
        $"#, &workspace));
    assert_eq!("6.", run("rfact(3)", &workspace));
    assert_eq!("3628800.", run("rfact(10)", &workspace));
    assert_eq!("6.2045E23", run("25+rfact(rfact(4))", &workspace));
    assert_eq!("3.5569E14", run("rfact(17)", &workspace));
    assert_eq!("", run(
        r#"$dfact(n)
        dfact_dbl(1.0)
        (n=0)-->%0
        dfact_n*dfact(n-1)
        $"#, &workspace));
    assert_eq!("6.", run("dfact(3)", &workspace));
    assert_eq!("3628800.", run("dfact(10)", &workspace));
//    assert_eq!("6.2045D23", run("25+dfact(dfact(4))", &workspace));       //  Fuzz problem
    assert_eq!("355687428096000.", run("dfact(17)", &workspace));
    assert_eq!("", run(
        r#"$ack(x,y)
        (x=0)-->%5
        (y=0)-->%7
        ack_ack(x-1,ack(x,y-1))
        -->%0
        ack_y+1
        -->%0
        ack_ack(x-1,1)
        $"#, &workspace));
        assert_eq!("29", run("ack(3,2)", &workspace));
        assert_eq!("4", run("ack(1,2)", &workspace));
        assert_eq!("11", run("ack(2,4)", &workspace));
    
}