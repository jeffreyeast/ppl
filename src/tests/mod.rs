#[cfg(test)]
mod tests {

    use crate::{execution::{evaluate, evaluate_with_diverted_stdout}, workspace::WorkSpace};

    mod chapter3;
    mod chapter4;
    mod chapter5;
    mod chapter6;
    mod chapter7;
    mod chapter8;
    mod  chapter9;
    mod chapter10;
    mod chapter11;
    mod chapter12;
    mod chapter13;
    mod extensions;
    mod negative;

    #[test]
    fn simple_echo() {
        let workspace = WorkSpace::new();
        assert_eq!("1", run("1", &workspace));
    }

    fn run(s: &str, workspace: &WorkSpace) -> String {
         match evaluate(s, workspace) {
            Ok(s) => return s,
            Err(s) => {
                println!("{}", s);
                return String::new();
            }
         }
    }

    fn run_divert_stdout(s: &str, workspace: &WorkSpace) -> String {
        match evaluate_with_diverted_stdout(s, workspace) {
            Ok(s) => return s,
            Err(s) => {
                println!("{}", s);
                return String::new();
            }
        }
    }

}