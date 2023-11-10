use std::{io::{self, Write}};


use ppl::workspace::WorkSpace;
use ppl::execution::evaluate;

extern crate signal_hook;



fn main() {

    let version_query = format!("version()");
    let eval_query2 = format!("exec(??)");
    let workspace = WorkSpace::new();
    let mut stdout = io::stdout();

    signal_hook::flag::register(signal_hook::consts::SIGINT, workspace.get_execution_sentinal_as_atomicbool()).expect("failed to register ^C handler");


    write!(stdout, "{}\n\t",evaluate(&version_query, &workspace).expect("internal error")).expect("internal error");
    stdout.flush().expect("internal error");
    
    loop {
        match evaluate(&eval_query2, &workspace) {
            Ok(result) => {
                write!(stdout, "{}\n\t", result).expect("internal error");
            },
            Err(s) => {
                write!(stdout, "Error: {s}\n\t").expect("internal error");
            },
        }
        stdout.flush().expect("internal error");
    }
}
