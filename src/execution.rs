//  This module holds the main evaluation entry point for the interpreter

use std::rc::Rc;

use crate::{
    workspace::{WorkSpace, debug::DebugOption, GeneralSymbol}, 
    execution::value::Value,
    parser::{ Parser, tree::{ArgumentDescription, Node, OperationNode, ReferenceNode }}, 
    symbols::metadata::{ FunctionClass, FunctionDescription, FunctionImplementation, VariableDescription, SelectorDescription, MetaDataType, FunctionArgumentList, ArgumentMechanism}, sequencer,
    utility::convert_escape_sequences};

use self::value::{structure::SelectorInstance, sequence::SequenceInstance, recursion_detector, SymbolicReference};


pub mod definition;
pub mod functions;
pub mod runtime;
pub mod sentinal;
pub mod system_functions;
pub mod value;






fn assemble_argument_list(f: &Rc<FunctionDescription>, actual_argument_list: &Vec<usize>, workspace: &WorkSpace) -> Result<Vec<Value>,String> {
    let mut actual_argument_values = Vec::new();
    match f.arguments {
        FunctionArgumentList::Fixed(ref formal_args) => {
            if formal_args.len() != actual_argument_list.len() {
                return Err(format!("Incorrect number of arguments to {}", f.name));
            }
            for _i in actual_argument_list {
                actual_argument_values.push(workspace.pop_value());
            }
        },
        FunctionArgumentList::Varying(_) => {
            for _i in actual_argument_list {
                actual_argument_values.push(workspace.pop_value());
            }
        },
    }
    actual_argument_values.reverse();
    Ok(actual_argument_values)
}

fn construct(datatype: &Rc<MetaDataType>, actual_argument_list: &Vec<usize>, workspace: &WorkSpace) -> Result<(),String> {
    //  Assemble the actual argument values

    let mut actual_argument_values = Vec::new();
    for _i in actual_argument_list {
        actual_argument_values.push(workspace.pop_value());
    }
    actual_argument_values.reverse();

    //  And build the item

    workspace.push_value(&self::value::construct(datatype, &actual_argument_values, workspace)?);
    Ok(())
}

pub fn evaluate(s: &str, workspace: &WorkSpace) -> Result<String,String> {
    let result = evaluate_internal(s, workspace);
    

    let mut result_string = match &result {
        Ok(r) => r.clone(),
        Err(e) => e.clone(),
    };

    match workspace.current_invocation() {
        Some(invocation) => {
            match (invocation.get_current_line_number(), invocation.get_current_statement_source(), invocation.get_fib()) {
                (Some(l), Some(s), Some(fib)) => {
                    result_string += format!("\n@{} [{}] {}\t", fib.function_description.name, l, convert_escape_sequences(s.as_str())).as_str();
                },
                (Some(l), Some(s), None) => {
                    result_string += format!("\n@[{}] {}\t", l, s).as_str();
                },
                (_, _, _) => {},
            }
        },
        None => workspace.reset(),
    }

    match workspace.current_invocation() {
        Some(invocation) => {
            match invocation.get_execution_state() {
                runtime::invocation::ExecutionState::NotExecuting | runtime::invocation::ExecutionState::Executing => workspace.reset(),
                runtime::invocation::ExecutionState::Stopped => {},
                runtime::invocation::ExecutionState::Resumed => panic!("internal error"),
            }
        },
        None => {},
    }

    match result {
        Ok(_) => Ok(result_string),
        Err(_) => Err(result_string),
    }
}

pub fn evaluate_identifier_by_value(reference: &SymbolicReference, workspace: &WorkSpace) -> Result<Value,String> {
    let value_stack_size = workspace.get_value_stack_size();
    match reference.as_symbol() {
        GeneralSymbol::Datatype(d) => execute_value(&SequenceInstance::construct_string_sequence(&d.as_string()), workspace)?,
        GeneralSymbol::Function(ref f) => execute_function(f, &vec![], workspace)?,
        GeneralSymbol::Selector(s) => execute_selector_reference(s, workspace)?,
        GeneralSymbol::Variable(v) => execute_variable_reference_by_value(v, workspace)?,
        GeneralSymbol::Unresolved(u) => return Err(format!("{} not found", u)),
    }
    Ok(workspace.try_pop_value(value_stack_size).unwrap_or(Value::Empty))
}

pub fn evaluate_internal(s: &str, workspace: &WorkSpace) -> Result<String,String> {
    prepare(s, workspace)?;
    match sequencer::start_execution(workspace) {
        Ok(r) => {
            recursion_detector::Cycle::start();
            Ok(format!("{}", r.unwrap_or(Value::Empty)))
        },
        Err(e) => {
            Err(format!("{}", e))
        },
    }
}

pub fn evaluate_with_diverted_stdout(s: &str, workspace: &WorkSpace) -> Result<String,String> {
    workspace.enable_alternate_print_destination();
    let result = evaluate_internal(s, workspace);
    match result {
        Ok(_) => {
            let result = workspace.alternate_print_destination_as_string();
            workspace.disable_alternate_print_destination();
            Ok(result)
        },
        Err(e) => {
            workspace.disable_alternate_print_destination();
            workspace.reset();
            Err(e)
        },
    }
}

pub fn execute(node: &Node, workspace: &WorkSpace) -> Result<(),String>{
    recursion_detector::Cycle::start();

    match node {
        Node::Definition(d) => definition::execute_definition(d, workspace)?,
        Node::ExecReturn => execute_exec_return(workspace)?,
        Node::FunctionReturn => execute_function_return(workspace)?,
        Node::Index(_) => execute_selection_by_reference(workspace)?,
        Node::Noop => {},
        Node::Operation(op) =>  execute_operation(op, workspace)?,
        Node::IdentifierByValue(r) => execute_identifier_by_value(r, workspace)?,
        Node::IdentifierByReference(r) => execute_identifier_by_reference(r, workspace)?,
        Node::ResolveParameter(d) => execute_resolve_parameter(d, workspace)?,
        Node::StatementEnd(_) => execute_statement_end(workspace)?,
        Node::StatementLabel(_) => {},
        Node::Value(v) => execute_value(v, workspace)?,
    }

    Ok(())
}

fn execute_exec_return(workspace: &WorkSpace) -> Result<(),String> {
    match workspace.get_last_statement_value() {
        Some(value) => workspace.push_value(&value),
        None => workspace.push_value(&Value::Empty),
    }
    workspace.end_invocation();
    Ok(())
}

fn execute_function(f: &Rc<FunctionDescription>, actual_argument_list: &Vec<usize>, workspace: &WorkSpace) -> Result<(),String> {

    //  Assemble the actual argument values, then invoke the function

    let actual_argument_values = assemble_argument_list(&f, actual_argument_list, workspace)?;
    runtime::debug::display_function(f, &actual_argument_values, workspace);
    match &f.implementation_class {
        FunctionImplementation::System(function_class) => {
            match &function_class {
                FunctionClass::Diadic(func) => {
                    workspace.push_value(&func(&actual_argument_values[0], &actual_argument_values[1], workspace)?);
                },
                FunctionClass::Monadic(func) => {
                    workspace.push_value(&func(&actual_argument_values[0], workspace)?);
                },
                FunctionClass::Nullary(func) => {
                    workspace.push_value(&func(workspace)?);
                },
                FunctionClass::Triadic(func) => {
                    workspace.push_value(&func(&actual_argument_values[0], &actual_argument_values[1], &actual_argument_values[2], workspace)?);
                },
                FunctionClass::Varying(func) => {
                    workspace.push_value(&func(&actual_argument_values, workspace)?);
                },
                FunctionClass::NullValuedDiadic(func) => {
                    func(&actual_argument_values[0], &actual_argument_values[1], workspace)?;
                },
                FunctionClass::NullValuedMonadic(func) => {
                    func(&actual_argument_values[0], workspace)?;
                },
                FunctionClass::NullValuedNullary(func) => {
                    func(workspace)?;
                },
                FunctionClass::NullValuedTriadic(func) => {
                    func(&actual_argument_values[0], &actual_argument_values[1], &actual_argument_values[2], workspace)?;
                },
                FunctionClass::NullValuedVarying(func) => {
                    func(&actual_argument_values, workspace)?;
                },
            }
        },
        FunctionImplementation::User(_) => {
            if workspace.debug_options.borrow().is_set(&DebugOption::StackUsage) {
                println!("\t\tCalling execute_user_function\t{}", workspace.get_stack_size());
            }
                    execute_user_function(f, &actual_argument_values, workspace)?;
        },
    }

    Ok(())
}

fn execute_function_return(workspace: &WorkSpace) -> Result<(),String> {
    let invocation = workspace.current_invocation().unwrap();
    let fib = invocation.get_fib().unwrap();
    if let Some(v) = workspace.try_get_variable(fib.function_description.name.as_str()) {
        workspace.push_value(&*v.cell.borrow().as_ref_to_value());
    } else {
        workspace.push_value(&Value::Empty);
    }
    workspace.end_invocation();
    Ok(())
}

fn execute_identifier_by_reference(r: &ReferenceNode, workspace: &WorkSpace) -> Result<(),String> {
    execute_value(&Value::from_reference(r, workspace)?, workspace)
}

fn execute_operation(op: &OperationNode, workspace: &WorkSpace) -> Result<(),String> {

    //  Look for a function that's compatible with the argument list

    let candidates = workspace.try_get_functions(&op.get_name());
    match  candidates.len() {
        0 => Err(format!("Function '{}' not found", op.get_name())),
        _ => {
            for candidate in &candidates {
                match candidate {
                    crate::workspace::GeneralSymbol::Datatype(d) => {
                        //  This is a constructor call
                        return construct(d, op.get_actual_argument_list(), workspace);
                    },
                    crate::workspace::GeneralSymbol::Function(f) => {
                        if f.is_compatible_function(op.get_actual_argument_list().len()) {
                            return execute_function(f, op.get_actual_argument_list(), workspace);
                        }
                    },
                    crate::workspace::GeneralSymbol::Selector(s) => {
                        return execute_selector(s, op.get_actual_argument_list(), workspace);
                    },
                    _ => panic!("internal error"),
                }
            }
            Err(format!("Wrong number of arguments to function {}", op.get_name()))
        },
    }
}

fn execute_identifier_by_value(r: &ReferenceNode, workspace: &WorkSpace) -> Result<(),String> {
    workspace.push_value(&Value::from_reference(r, workspace)?);
    Ok(())
}

fn execute_resolve_parameter(arg: &ArgumentDescription, workspace: &WorkSpace) -> Result<(), String> {
    let symbols = workspace.try_get_functions(arg.function_name.as_str());
    for symbol in symbols {
        match &symbol {
            GeneralSymbol::Datatype(_) => (),
            GeneralSymbol::Function(f) => {
                if f.is_compatible_function(arg.argument_count) {
                    match f.arguments {
                        FunctionArgumentList::Fixed(ref formal_args) => {
                            if arg.argument_number >= formal_args.len() {
                                return Err(format!("Incorrect number of arguments to {}", f.name));
                            }
                            let formal_arg = &formal_args[arg.argument_number];
                            match formal_arg.mechanism {
                                ArgumentMechanism::ByReference | ArgumentMechanism::ByReferenceCreateIfNeeded => (),
                                ArgumentMechanism::ByValue => {
                                    let formal_arg_datatype = workspace.resolve_datatype(&formal_arg.datatype.as_string())?;
                                    workspace.push_value(&formal_arg_datatype.coerce(&workspace.pop_value(), workspace)?);
                                },
                            }
                        },
                        FunctionArgumentList::Varying(ref mechanism) => {
                            match  mechanism {
                                ArgumentMechanism::ByReference | ArgumentMechanism::ByReferenceCreateIfNeeded => (),
                                ArgumentMechanism::ByValue => {
                                    match workspace.pop_value() {
                                        Value::Selector(ref selector) => return Err(format!("{} is not a value", selector)),
                                        Value::Symbol(ref symbol) => workspace.push_value(&evaluate_identifier_by_value(symbol, workspace)?),
                                        Value::ValueByReference(cell_ref) => workspace.push_value(&cell_ref.cell.borrow().as_ref_to_value().clone()),
                                        Value::LogicalLink(link) => workspace.push_value(&link.as_ref_to_value().clone()),
                                        value => workspace.push_value(&value),
                                    }
                                },
                            }
                        },
                    }
                    break;
                }
            },
            GeneralSymbol::Selector(_) => (),
            _ => panic!("internal error"),
        }
    } 
    Ok(())
}

fn execute_selection_by_reference(workspace: &WorkSpace) -> Result<(), String> {
    let mut index = workspace.pop_value();
    if let Value::Symbol(symbol) = &index {
        index = evaluate_identifier_by_value(symbol, workspace)?;
    }
    let base_value = workspace.pop_value();
    execute_selection_by_reference_internal(&base_value, &index, workspace)
}

fn execute_selection_by_reference_internal(base_value: &Value, index: &Value, workspace: &WorkSpace) -> Result<(), String> {
    match &base_value {
        Value::Sequence(ref seq) => {
            return seq.access_cell_by_reference(index.as_i32()?, workspace);
        },
        Value::Structure(ref structure) => {
            if let Value::Selector(ref selector) = index {
                return structure.access_field_by_reference(selector, workspace);
            } else {
                return Err(format!("{} is not a field selector", index));
            }
        },
        Value::Symbol(symbol) => {
            match symbol.as_symbol() {
                GeneralSymbol::Variable(v) => {
                    match &*v.cell.borrow().as_ref_to_value() {
                        Value::Sequence(ref seq) => {
                            return seq.access_cell_by_reference(index.as_i32()?, workspace);
                        },
                        Value::Structure(ref structure) => {
                            if let Value::Selector(ref selector) = index {
                                return structure.access_field_by_reference(selector, workspace);
                            } else {
                                if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                                    dbg!(&index);
                                }
                            return Err(format!("{} is not a field selector", index));
                            }
                        },
                        _ => {},
                    }  
                },
                GeneralSymbol::Unresolved(u) => return Err(format!("{} not found", u)),
                _ => {},
            }
            if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                dbg!(&base_value);
            }
            Err(format!("{} is not indexable", base_value))
        },
        Value::ValueByReference(v) => {
            execute_selection_by_reference_internal(&*v.cell.borrow().as_ref_to_value(), index, workspace)
        },
        Value::LogicalLink(link) => {
            execute_selection_by_reference_internal(&*link.as_ref_to_value(), index, workspace)
        },
        _ => { 
            if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                dbg!(&base_value);
            }
             Err(format!("{} is not indexable", base_value))
        },
    }
}

fn execute_selector(s: &Rc<SelectorDescription>, actual_argument_list: &Vec<usize>, workspace: &WorkSpace) -> Result<(),String> {
    if actual_argument_list.len() != 1 {
        return Err(format!("Selectors can only operate on one structure"));
    }

    //  Assemble the actual argument values

    let mut actual_argument_value = workspace.pop_value();
    if let Value::Symbol(symbol) = actual_argument_value {
        actual_argument_value = evaluate_identifier_by_value(&symbol, workspace)?;
    }

    loop {
        match &actual_argument_value {
            Value::Symbol(symbol) => actual_argument_value = evaluate_identifier_by_value(&symbol, workspace)?,
            Value::Structure(ref structure) => {
                for target in s.structures.borrow().iter() {
                    if target.as_string() == structure.as_string() {
                        for member in structure.as_values() {
                            if member.as_string() == s.member.as_string() {
                                workspace.push_value(&member.as_value());
                                return Ok(());
                            }
                        }
                    }
                }
                return Err(format!("{} does not have a {} field", structure.as_string(), s.member.as_string()));
            },
            Value::ValueByReference(cell_reference) => {
                if let Value::Structure(structure) =  &*cell_reference.cell.borrow().as_ref_to_value() {
                    for target in s.structures.borrow().iter() {
                        if target.as_string() == structure.as_string() {
                            for member in structure.as_values() {
                                if member.as_string() == s.member.as_string() {
                                    workspace.push_value(&member.as_reference());
                                    return Ok(());
                                }
                            }
                        }
                    }
                    return Err(format!("{} does not have a {} field", structure.as_string(), s.member.as_string()));
                } else {
                    if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                        dbg!(&actual_argument_value);
                    }
                        return Err(format!("Selectors can only target a structure"))
                }
            },
            _ => {
                if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                    dbg!(&actual_argument_value);
                }
                return Err(format!("Selectors can only target a structure"))
            },
        }
    }
}

fn execute_selector_reference(selector: &Rc<SelectorDescription>, workspace: &WorkSpace) -> Result<(),String> {
    execute_value(&Value::Selector(SelectorInstance::from_description(&selector)), workspace)
}

pub fn execute_statement_end(workspace: &WorkSpace) -> Result<(),String> {
    let invocation = workspace.current_invocation().unwrap();
    match workspace.try_pop_value(invocation.get_base_stack_size()) {
        Some(value) => {
            match value {
                Value::Selector(sel) => return Err(format!("{} is not a value", sel.member)),
                Value::Symbol(symbol) => {
                    if let GeneralSymbol::Function(_) = symbol.as_symbol() {
                        // We're going to evaluate the function, and so doing, need to re-execute this node
                        invocation.repeat();
                    }
                    workspace.set_last_statement_value(&evaluate_identifier_by_value(&symbol, workspace)?);
                },
                Value::ValueByReference(cell_reference) => workspace.set_last_statement_value(&*cell_reference.cell.borrow().as_ref_to_value()),
                Value::LogicalLink(link) => workspace.set_last_statement_value(&*link.as_ref_to_value()),
             _ => workspace.set_last_statement_value(&value),
            }
        },
        None => {},
    }
    if let Some(index) = invocation.get_pending_goto_index() {
        invocation.set_next_node(index);
        invocation.set_pending_goto_index(None);
    }
    Ok(())
}

fn execute_user_function(f: &Rc<FunctionDescription>, actual_argument_values: &Vec<Value>, workspace: &WorkSpace) -> Result<(),String> {
    functions::prepare_udf (f, actual_argument_values, workspace)?;
    functions::execute_udf (f, workspace)?;
    Ok(())
}

fn execute_value(value: &Value, workspace: &WorkSpace) -> Result<(),String> {
    workspace.push_value(value);
    Ok(())
}

fn execute_variable_reference_by_value(v: &Rc<VariableDescription>, workspace: &WorkSpace) -> Result<(),String> {
    execute_value(&*v.cell.borrow().as_ref_to_value(), workspace)
}

pub fn prepare(s: &str, workspace: &WorkSpace) -> Result<(),String> {
    let parser = Parser::new(s, workspace);
    let executable = parser.parse(s)?;
    if workspace.debug_options.borrow().is_set(&DebugOption::Parse) {
        dbg!(&executable);
    }
    sequencer::prepare_execution(&executable, workspace);
    Ok(())
}

pub fn prepare_as_exec(s: &str, workspace: &WorkSpace) -> Result<(),String> {
    let parser = Parser::new(s, workspace);
    let executable = parser.parse_as_exec(s)?;
    if workspace.debug_options.borrow().is_set(&DebugOption::Parse) {
        dbg!(&executable);
    }
    sequencer::prepare_execution(&executable, workspace);
    Ok(())
}
