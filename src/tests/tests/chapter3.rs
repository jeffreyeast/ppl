use crate::{tests::tests::run, workspace::WorkSpace};


    #[test]
    fn chapter3() {
        let workspace = WorkSpace::new();
        // Page 3
        assert_eq!("5", run("3+2", &workspace));
        assert_eq!("1200", run("25*(46+2)", &workspace));
        assert_eq!("1.9635E1", run("3.14159*(2.5^2)", &workspace));
        // Page 4
        assert_eq!("1152", run("3*3*3+5*5*5", &workspace));
        assert_eq!("1152", run("3*(3*(3+(5*5*5)))", &workspace));
        assert_eq!("152", run("(3*3*3)+(5*5*5)", &workspace));
        assert_eq!("2.", run("10.0/3+2", &workspace));
        assert_eq!("2.", run("10.0/(3+2)", &workspace));
        assert_eq!("5.333333", run("(10.0/3)+2", &workspace));
        assert_eq!("true", run("31==int", &workspace));
        assert_eq!("true", run("0==int", &workspace));
        assert_eq!("true", run("987654321==int", &workspace));
        assert_eq!("true", run("000130==int", &workspace));
        assert_eq!("true", run("3.14159==real", &workspace));
        assert_eq!("true", run(".52==real", &workspace));
        assert_eq!("true", run("0.==real", &workspace));
        assert_eq!("true", run("1.347e28==real", &workspace));
        assert_eq!("true", run("52e-7==real", &workspace));
        assert_eq!("false", run("52e-7==dbl", &workspace));
        assert_eq!("true", run("1.4d0==dbl", &workspace));
        assert_eq!("true", run("9.327863415==dbl", &workspace));
        assert_eq!("true", run("83.5216d27==dbl", &workspace));
        assert_eq!("false", run("83.5216d27==real", &workspace));
        assert_eq!("true", run("1d-10==dbl", &workspace));
        // Page 5
        assert_eq!("1", run("3/2", &workspace));
        assert_eq!("1.5", run("3/2.", &workspace));
        assert_eq!("1.5", run("3./2", &workspace));
        assert_eq!("6.9", run("3+4-.1", &workspace));
    }
