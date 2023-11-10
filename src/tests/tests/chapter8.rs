use crate::{tests::tests::run, workspace::WorkSpace};


#[test]
fn chapter8() {
    let workspace = WorkSpace::new();
   
    //  Page 16
   
    assert_eq!("", run(
        r#"$prime(n); i
         prime_false
         (n<4)-->%7
         (n=2*n/2)-->%0
         i_3
         (n=i*n/i)-->%0
         i_i+2
         ((i^2)<=n)-->%5
         prime_true
        $"#, &workspace));
    assert_eq!("true", run("prime(5)", &workspace));
    assert_eq!("true", run("prime(7)", &workspace));
    assert_eq!("true", run("prime(13)", &workspace));
    assert_eq!("false", run("prime(6)", &workspace));
    assert_eq!("false", run("prime(14)", &workspace));
    assert_eq!("", run("erase(prime)", &workspace));
    assert_eq!("", run(
        r#"$prime(n); i
         prime_false
         (n<4)-->%7
         (n=2*n/2)-->%0
         i_3
         loop: (n=i*n/i)-->%0
         i_i+2
         ((i^2)<=n)-->loop
         prime_true
        $"#, &workspace));
    assert_eq!("true", run("prime(5)", &workspace));
    assert_eq!("true", run("prime(7)", &workspace));
    assert_eq!("true", run("prime(13)", &workspace));
    assert_eq!("false", run("prime(6)", &workspace));
    assert_eq!("false", run("prime(14)", &workspace));

    // Page 17

}