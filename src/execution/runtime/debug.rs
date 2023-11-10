//  This module holds support routines for debugging execution

use std::rc::Rc;

use crate::{workspace::{WorkSpace, debug::DebugOption}, symbols::metadata::FunctionDescription, execution::value::Value, utility::convert_escape_sequences};


pub fn display_function(f: &Rc<FunctionDescription>, actual_argument_values: &Vec<Value>, workspace: &WorkSpace) {
    if workspace.debug_options.borrow().is_set(&DebugOption::BuiltinFunctions) {
        print!("Invoking: {}(", f.name);
        let mut separator = "";
        for arg in actual_argument_values {
            let normalized_arg = convert_escape_sequences(format!("{}", arg).as_str());
            print!("{}{}", separator, normalized_arg);
            separator = ", ";
        }
        println!(")");
    }
}