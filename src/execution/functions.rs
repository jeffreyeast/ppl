//  This module holds support for function invocation

use std::cell::RefCell;
use std::rc::Rc;

use crate::{execution::value::{Value, Cell},
            symbols::{SymbolTable, 
                      metadata::{FunctionArgumentList, FunctionDescription, FunctionImplementation, VariableDescription, ArgumentMechanism}, name::Name},
            workspace::{WorkSpace, debug::DebugOption}};

use super::runtime::statement::Statement;




#[derive(Debug)]
pub struct FunctionInvocationBlock {
    pub function_description: Rc<FunctionDescription>,
    pub variable_symbol_table: RefCell<SymbolTable<VariableDescription>>,
}


impl FunctionInvocationBlock {
    pub fn new (function_description: Rc<FunctionDescription>) -> FunctionInvocationBlock {
        FunctionInvocationBlock { 
            function_description: function_description.clone(),
            variable_symbol_table: RefCell::new(SymbolTable::new()), }
    }
}



pub fn prepare_udf(f: &Rc<FunctionDescription>, actual_argument_values: &Vec<Value>, workspace: &WorkSpace) -> Result<Value,String> {
    if let FunctionImplementation::User(body) = &f.implementation_class {
        if body.executable.get_statement_count() < 1 {
            return Ok(Value::Empty);
        }
        if let FunctionArgumentList::Fixed(formal_args) = &f.arguments {
            workspace.start_user_function(f);
            let invocation = workspace.current_invocation().unwrap();
            let fib = invocation.get_fib().unwrap();
            let mut symbol_table = fib.variable_symbol_table.borrow_mut();

            //  Load the function name in as a locazl variable

            symbol_table.add(f.name.clone(), VariableDescription { cell: Cell::new(Value::Empty)});

            //  Load and initialize the formal parameters

            for i in 0..actual_argument_values.len() {
                match (&formal_args[i].mechanism, &actual_argument_values[i]) {
                    (ArgumentMechanism::ByReference, Value::Symbol(symbol)) => {
                        match symbol.as_symbol() {
                            crate::workspace::GeneralSymbol::Variable(v) => {
                                symbol_table.add_by_reference(formal_args[i].name.clone(), v.clone());
                            },
                            _ => {
                                symbol_table.add(
                                formal_args[i].name.clone(), 
                                VariableDescription { cell: Cell::new(actual_argument_values[i].clone())});
                            },
                        }
                    },
                    (_, _) => {
                        symbol_table.add(
                            formal_args[i].name.clone(), 
                            VariableDescription { cell: Cell::new(actual_argument_values[i].clone())});
                    },
                }
            }

            //  Load any local variables

            if let Some(local_variables) = &f.local_variables {
                for i in 0..local_variables.len() {
                    symbol_table.add(local_variables[i].clone(), VariableDescription { cell: Cell::new(Value::Empty)});
                }
            }

            //  Load statement labels

            for (label_name, line_number) in &body.labels {
                symbol_table.add(Name::from_string(&label_name), VariableDescription { cell: Cell::new(Value::Int(*line_number as i32)) });
            }

            {
                workspace.get_execution_sentinal_mut().clear_stop_requested();
            }

            if workspace.debug_options.borrow().is_set(&DebugOption::Parse) {
                dbg!(&invocation);
            }
        
            return Ok(Value::Empty)
        } else {
            return Err(format!("Variable length argument lists are not supported for user function {}", f.name.as_string()))
        }
    }
    panic!("internal error");
}

pub fn execute_udf(_f: &Rc<FunctionDescription>, _workspace: &WorkSpace) -> Result<(),String> {
    /* let invocation = workspace.current_invocation().unwrap();
    let fib = invocation.get_fib().unwrap();

    match start_user_function(workspace) {
        Ok(value) => {
            let final_value: Value;
            if let Some(v) = fib.variable_symbol_table.borrow().try_get(f.name.as_str()) {
                final_value = v.cell.borrow().as_ref_to_value().clone();
            } else {
                final_value = value.unwrap_or(Value::Empty);
            }

            workspace.push_value(&final_value);
            return Ok(());
        },
        Err(e) => {
            return Err(e);
        },
    } */
    Ok(())
}

pub fn stop(name: &Value, lines: &[Value], workspace: &WorkSpace) -> Result<(),String> {
    statement_internal(name, lines, |s| { s.set_stop(true); }, workspace)
}

pub fn trace(name: &Value, lines: &[Value], workspace: &WorkSpace) -> Result<(),String> {
    statement_internal(name, lines, |s| { s.set_trace(true); }, workspace)
}

pub fn unstop(name: &Value, lines: &[Value], workspace: &WorkSpace) -> Result<(),String> {
    statement_internal(name, lines, |s| { s.set_stop(false); }, workspace)
}

pub fn untrace(name: &Value, lines: &[Value], workspace: &WorkSpace) -> Result<(),String> {
    statement_internal(name, lines, |s| { s.set_trace(false); }, workspace)
}

fn statement_internal(name: &Value, lines: &[Value], setting: fn(statement: Rc<Statement>), workspace: &WorkSpace) -> Result<(),String> {
    let opt_f = workspace.try_get_function(name.as_string().as_str());
    match opt_f {
        Some(f) => {
            match &f.implementation_class {
                FunctionImplementation::System(_) => Err(format!("System functions cannot be stopped")),
                FunctionImplementation::User(body) => {
                    if lines.len() == 0 {
                        body.executable.process_on_each_statement(setting);
                    } else {
                        for line in lines {
                            match body.executable.get_statement_by_line_number(line.as_line_number()?) {
                                Some(statement) => setting(statement),
                                None => return Err(format!("Invalid line number {}", line)),
                            }
                        }
                    }
                    Ok(())
                }
            }
        },
        None => Err(format!("Function {} not found", name))
    }
}
