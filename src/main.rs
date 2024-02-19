use std::io::{self, Write};


use ppl::workspace::WorkSpace;
use ppl::execution::evaluate;

extern crate signal_hook;



fn main() {
    let version_query = format!("print(\"\\n\", version(), \"\n\")");
    let eval_query2 = format!("print(exec(??), \"\n\")");
    let workspace = WorkSpace::new();
    let mut stdout = io::stdout();

    signal_hook::flag::register(signal_hook::consts::SIGINT, workspace.get_execution_sentinal_as_atomicbool()).expect("failed to register ^C handler");


    match evaluate(&version_query, &workspace) {
        Ok(_) => (),
        Err(_) => panic!("Internal error"),
    }
    
    loop {
        write!(stdout, "\n\t").expect("Internal error");
        stdout.flush().expect("internal error");
        match evaluate(&eval_query2, &workspace) {
            Ok(_) => (),
            Err(s) => {
                write!(stdout, "Error: {s}").expect("internal error");
            },
        }
        stdout.flush().expect("internal error");
    }
}



