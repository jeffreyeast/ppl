//  This module contains the implementation of the builtin system functions

use std::{io::{self, Write, stdout}, process};
use crate::{symbols::{ 
    metadata::{FunctionDescription, FunctionArgumentList, FormalArgument, FunctionImplementation, FunctionClass, MetaDataTypeName, ArgumentMechanism, VariableDescription}, 
    name::Name, datatype::is_assignable_to}, 
    workspace::{WorkSpace, GeneralSymbol}, lexical::LineNumber};
use crate::execution::value::Value;
use super::{evaluate_internal, value::{sequence::SequenceInstance, Cell, recursion_detector}};

mod arithmetic;
mod comparison;
mod debug;
mod metadata;

pub fn init(workspace: &WorkSpace) {
    arithmetic::init(workspace);
    comparison::init(workspace);
    debug::init(workspace);
    metadata::init(workspace);

    workspace.add_system_function(
        "_", 
        FunctionDescription { 
            name: Name::from_str("assign"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("variable"), mechanism: ArgumentMechanism::ByReferenceCreateIfNeeded,  datatype: MetaDataTypeName::from_str("general") },
                FormalArgument { name: Name::from_str("value"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("general")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(assign)),
            help_text: String::from("Assign a value") });
                                                        
    workspace.add_system_function(
        "__", 
        FunctionDescription { 
            name: Name::from_str("noncopy"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("variable"), mechanism: ArgumentMechanism::ByReferenceCreateIfNeeded,  datatype: MetaDataTypeName::from_str("general") },
                FormalArgument { name: Name::from_str("value"), mechanism: ArgumentMechanism::ByReference,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("reference")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(noncopy)),
            help_text: String::from("Assign a reference to a value") });

    workspace.add_system_function(
        "?", 
        FunctionDescription { 
            name: Name::from_str("read_stdin"), 
            arguments: FunctionArgumentList::Fixed(Vec::new()),
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("string")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Nullary(read_stdin)),
            help_text: String::from("Read and return user input from the console") });
        
    workspace.add_system_function(
        "??", 
        FunctionDescription { 
            name: Name::from_str("read_stdin_ppl"), 
            arguments: FunctionArgumentList::Fixed(Vec::new()),
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("string")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Nullary(read_stdin_ppl)),
            help_text: String::from("Read and return user input from the console") });

    workspace.add_system_function(
        "-->", 
        FunctionDescription { 
            name: Name::from_str("goto"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("destination"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("int") }]), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedMonadic(goto)),
            help_text: String::from("Transfer control to the designated statement") });

    workspace.add_system_function(
        "-->", 
        FunctionDescription { 
            name: Name::from_str("cgoto"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("condition"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("bool") }, 
                FormalArgument { name: Name::from_str("destination"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("int") }]), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedDiadic(cgoto)),
            help_text: String::from("If the condition is true, transfer control to the designated statement") });

    workspace.add_system_function(
        "assign", 
        FunctionDescription { 
            name: Name::from_str("assign"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("variable"), mechanism: ArgumentMechanism::ByReferenceCreateIfNeeded,  datatype: MetaDataTypeName::from_str("general") },
                FormalArgument { name: Name::from_str("value"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("general")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(assign)),
            help_text: String::from("Assign a value") });

    workspace.add_system_function(
        "branch", 
        FunctionDescription { 
            name: Name::from_str("branch"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("destination"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("int") } ]), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedMonadic(branch)),
            help_text: String::from("Internal use only: transfer control to the designated node") });
                
    workspace.add_system_function(
        "cbranch", 
        FunctionDescription { 
            name: Name::from_str("cbranch"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("condition"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("bool") }, 
                FormalArgument { name: Name::from_str("destination"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("int") }]), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedDiadic(cbranch)),
            help_text: String::from("Internal use only: ff the condition is true, transfer control to the designated node") });
                        
    workspace.add_system_function(
        "cgoto", 
        FunctionDescription { 
            name: Name::from_str("cgoto"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("condition"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("bool") }, 
                FormalArgument { name: Name::from_str("destination"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("int") }]), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedDiadic(cgoto)),
            help_text: String::from("If the condition is true, transfer control to the designated statement") });
        
    workspace.add_system_function(
        "concat", 
        FunctionDescription { 
            name: Name::from_str("concat"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("v.sequence") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("v.sequence") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("v.sequence")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(concat)),
            help_text: String::from("Catenates two sequences of the same type") });
                                        
    workspace.add_system_function(
        "debug.print", 
        FunctionDescription { 
            name: Name::from_str("debug.print"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("operand"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("reference")), 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedMonadic(debug_print)),
            help_text: String::from("Display internal structure of operand on the console") });
                                                
    workspace.add_system_function(
        "exec", 
        FunctionDescription { 
            name: Name::from_str("exec"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("s"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("string") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("string")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Monadic(exec)),
            help_text: String::from("Evaluate a PPL expression") });
                
    workspace.add_system_function(
        "exit", 
        FunctionDescription { 
            name: Name::from_str("exit"), 
            arguments: FunctionArgumentList::Varying(ArgumentMechanism::ByValue), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::Varying(exit)),
            help_text: String::from("Terminate the interpreter") });
                        
    workspace.add_system_function(
        "feature", 
        FunctionDescription { 
            name: Name::from_str("feature"), 
            arguments: FunctionArgumentList::Fixed(vec![]), 
                local_variables: None,
                return_value: Some(MetaDataTypeName::from_str("string")), 
                implementation_class: FunctionImplementation::System(FunctionClass::Nullary(show_features)),
                help_text: String::from(r#"Display optional feature settings"#) });
                                
    workspace.add_system_function(
        "feature", 
        FunctionDescription { 
            name: Name::from_str("feature"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("operation"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("string") }, 
                FormalArgument { name: Name::from_str("option"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("string") }]), 
                local_variables: None,
                return_value: Some(MetaDataTypeName::from_str("string")), 
                implementation_class: FunctionImplementation::System(FunctionClass::Diadic(feature)),
                help_text: String::from(r#"{"SET" | "CLEAR"} feature options {"StringEscapes"}"#) });
                                        
    workspace.add_system_function(
        "format", 
        FunctionDescription { 
            name: Name::from_str("format"), 
            arguments: FunctionArgumentList::Varying(ArgumentMechanism::ByValue), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedVarying(pformat)),
            help_text: String::from("Print formatted numbers to the console") });
                    
    workspace.add_system_function(
        "goto", 
        FunctionDescription { 
            name: Name::from_str("goto"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("destination"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("int") }]), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedMonadic(goto)),
            help_text: String::from("Transfer control to the designated statement") });
                                        
    workspace.add_system_function(
        "iformat", 
        FunctionDescription { 
            name: Name::from_str("iformat"), 
            arguments: FunctionArgumentList::Varying(ArgumentMechanism::ByValue), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("string")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Varying(iformat)),
            help_text: String::from("Convert number(s) to string using a format specification") });
                                                                
    workspace.add_system_function(
        "noncopy", 
        FunctionDescription { 
            name: Name::from_str("noncopy"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("variable"), mechanism: ArgumentMechanism::ByReferenceCreateIfNeeded,  datatype: MetaDataTypeName::from_str("general") },
                FormalArgument { name: Name::from_str("value"), mechanism: ArgumentMechanism::ByReference,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("reference")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(noncopy)),
            help_text: String::from("Assign a reference to a value") });
                                        
    workspace.add_system_function(
        "print", 
        FunctionDescription { 
            name: Name::from_str("print"), 
            arguments: FunctionArgumentList::Varying(ArgumentMechanism::ByValue), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedVarying(print)),
            help_text: String::from("Print to the console") });
}



fn assign(receiver: &Value, value: &Value, workspace: &WorkSpace) -> Result<Value, String> {
    match receiver {
        Value::Symbol(symbol) => {
            match symbol.as_symbol() {
                GeneralSymbol::Variable(v) => v.cell.borrow().set_value(value)?,
                GeneralSymbol::Unresolved(u) => {
                    Cell::validate_value(value)?;
                    workspace.add_variable(u.as_string().as_str(), VariableDescription { cell: Cell::new(value.clone()) });
                },
                _ => return Err(format!("Only variables can be the target of assign")),
            }
        },
        Value::ValueByReference(receiver) => {
            receiver.cell.borrow().set_value(&workspace.resolve_datatype(&receiver.datatype.as_string())?.coerce(value, workspace)?)?;
        },
        _ => {
            dbg!(receiver);
            dbg!(value);
            panic!("internal error");
        },
    }
    Ok(value.clone())
}

fn branch(destination: &Value, workspace: &WorkSpace) -> Result<(),String>{
    let node_index = destination.as_usize()?;
    let invocation = workspace.current_invocation().unwrap();
    invocation.set_pending_goto_index(Some(node_index));
    Ok(())
}

fn cbranch(condition: &Value, destination: &Value, workspace: &WorkSpace) -> Result<(),String> {
    if condition.as_bool()? {
        branch (destination, workspace)        
    } else {
        Ok(())
    }
}

fn cgoto(condition: &Value, destination: &Value, workspace: &WorkSpace) -> Result<(),String> {
    if condition.as_bool()? {
        goto (destination, workspace)        
    } else {
        workspace.push_value(&Value::Bool(false));
        Ok(())
    }
}

fn concat(left: &Value, right: &Value, _workspace: &WorkSpace) -> Result<Value,String> {
    if let Value::Sequence(left_sequence) = left {
        if let Value::Sequence(right_sequence) = right {
            if left_sequence.as_datatype() == right_sequence.as_datatype() {
                SequenceInstance::concat(left_sequence, right_sequence)
            } else {
                Err(format!("Both sequences must be of the same type"))
            }
        } else {
            Err(format!("{} is not a variadic sequence", right))
        }
    } else {
        Err(format!("{} is not a variadic sequence", left))
    }
}

fn debug_print(operand: &Value, _workspace: &WorkSpace) -> Result<(),String> {
    recursion_detector::Cycle::start();
    dbg!(operand);
    recursion_detector::Cycle::start();

    Ok(())
}


fn exec(s: &Value, workspace: &WorkSpace) -> Result<Value,String>{
    recursion_detector::Cycle::start();
    Ok(SequenceInstance::construct_string_sequence(&evaluate_internal(&s.as_string(), workspace)?))
}

fn exit(args: &Vec<Value>, _workspace: &WorkSpace) -> Result<Value,String> {
    let mut exit_code = 0;
    if args.len() >= 1 {
        exit_code = args[0].as_i32()?;
    }
    process::exit(exit_code);
    //Ok(Value::Empty)
}

fn feature(operation: &Value, option: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    match operation.as_string().to_ascii_lowercase().as_str() {
        "on" | "set" => workspace.features.borrow_mut().set_str(option.as_string().as_str())?,
        "off" | "clear" => workspace.features.borrow_mut().clear_str(option.as_string().as_str())?,
        _ => return Err(format!("{} is not a valid feature operation", operation.as_string())),
    }
    Ok(SequenceInstance::construct_string_sequence(""))
}

fn goto(destination: &Value, workspace: &WorkSpace) -> Result<(),String> {
    let line_number = destination.as_usize()? as LineNumber;

    //  We have a bit of a conundrum here...in that gotos in immediate mode are used to resume stopped
    //  functions. They're also used in functions to alter control flow. For now, we can simply trash the
    //  frames between us and the target function, but this won't work if we allow gotos to alter control flow
    //  in immediate mode.

    loop {
        match workspace.current_invocation() {
            Some(invocation) => {
                if invocation.get_fib().is_some() {
                    if let Some(statement) = invocation.as_executable().get_statement_by_line_number(line_number) {
                        invocation.set_pending_goto_index(Some(statement.as_first_node_index()));
                    } else {
                        if let Some(frln) = invocation.as_executable().get_function_return_line_number() {
                            if let Some(statement) = invocation.as_executable().get_statement_by_line_number(frln) {
                                invocation.set_pending_goto_index(Some(statement.as_first_node_index()));
                            } else {
                                panic!("internal error");
                            }
                        } else {
                            panic!("internal error");
                        }
                    }
                    match invocation.get_execution_state() {
                        super::runtime::invocation::ExecutionState::NotExecuting => panic!("internal error"),
                        super::runtime::invocation::ExecutionState::Stopped => {
                            invocation.set_execution_state(super::runtime::invocation::ExecutionState::Resumed);
                            invocation.set_next_node(invocation.get_pending_goto_index().unwrap());
                            invocation.set_pending_goto_index(None);
                        },
                        super::runtime::invocation::ExecutionState::Resumed => panic!("internal error"),
                        super::runtime::invocation::ExecutionState::Executing => {
                            workspace.push_value(&Value::Int(line_number as i32));
                        },
                    }
                    
                    return Ok(());
                }
                workspace.end_invocation();
            },
            None => return Err(format!("--> can only be used in immediate mode to resume a stopped function")),
        }
    }
}

fn iformat(args: &Vec<Value>, workspace: &WorkSpace) -> Result<Value,String> {
    Ok(SequenceInstance::construct_string_sequence(&Value::format(args, workspace)?))
}

fn noncopy(receiver: &Value, value: &Value, workspace: &WorkSpace) -> Result<Value, String> {
//    dbg!(receiver);
//    dbg!(value);
    match (receiver, value) {
        (Value::Symbol(symbol), Value::Symbol(attachee)) => {
            match (symbol.as_symbol(), attachee.as_symbol()) {
                (GeneralSymbol::Variable(v), GeneralSymbol::Variable(exp)) => {
                    v.cell.borrow_mut().set_reference(exp.cell.borrow().as_contents());
                },
                (GeneralSymbol::Unresolved(u), GeneralSymbol::Variable(exp)) => {
                    workspace.add_variable(u.as_string().as_str(), VariableDescription { cell: Cell::new_by_reference(exp.cell.borrow().as_contents()) });
                },
                (_,GeneralSymbol::Unresolved(_)) => return Err(format!("{} not found", attachee.as_string())),
                _ => {
                    return Err(format!("Only variables can be the target of assign"));
                },
            }
        },
        (Value::Symbol(symbol), Value::ValueByReference(attachee)) => {
            match symbol.as_symbol() {
                GeneralSymbol::Variable(v) => {
                    v.cell.borrow_mut().set_reference(attachee.cell.borrow().as_contents().clone());
                },
                GeneralSymbol::Unresolved(u) => {
                    workspace.add_variable(u.as_string().as_str(), VariableDescription { cell: Cell::new_by_reference(attachee.cell.borrow().as_contents()) });
                },
                _ => {dbg!(symbol.as_symbol()); return Err(format!("Only variables can be the target of assign"));},
            }
        },
        (Value::ValueByReference(receiver), Value::Symbol(attachee)) => {
            match attachee.as_symbol() {
                GeneralSymbol::Variable(v) => {
                    if is_assignable_to(&v.cell.borrow().as_ref_to_value(), &workspace.resolve_datatype(&receiver.datatype.as_string())?, workspace)? {
                        receiver.cell.borrow_mut().set_reference(v.cell.borrow().as_contents().clone());
                    } else {
                        return Err(format!("Incompatible datatype"));
                    }
                },
                GeneralSymbol::Unresolved(_) => return Err(format!("{} not found", attachee.as_string())),
                _ => return Err(format!("Only variables can be the target of assign")),
            }
        },
        (Value::ValueByReference(receiver), Value::ValueByReference(attachee)) => {
            if is_assignable_to(&*attachee.cell.borrow().as_ref_to_value(), &workspace.resolve_datatype(&receiver.datatype.as_string())?, workspace)? {
                receiver.cell.borrow_mut().set_reference(attachee.cell.borrow().as_contents().clone());
            } else {
                return Err(format!("Incompatible datatype"));
            }
        },
        (_,_) => {
            dbg!(receiver);
            dbg!(value);
            panic!("internal error");
        },
    }
    Ok(value.clone())
}

fn pformat(args: &Vec<Value>, workspace: &WorkSpace) -> Result<(),String> {
    print(&vec![iformat(args, workspace)?], workspace)
}

fn print(args: &Vec<Value>, workspace: &WorkSpace) -> Result<(),String> {
    let mut result = String::new();

    recursion_detector::Cycle::start();

    for arg in args {
        result += format!("{}", arg).as_str();
    }

    recursion_detector::Cycle::start();

    match *workspace.get_alternate_print_destinatin() {
        Some(ref mut cursor) => {
            cursor.write(result.as_bytes()).map_err(|e| format!("{}", e))?;
        },
        None => {
            write!(stdout(), "{}", result).map_err(|e| format!("{}", e))?;
            stdout().flush().map_err(|e| format!("{}", e))?;
        },
    }

    Ok(())
}

fn read_stdin(_workspace: &WorkSpace) -> Result<Value, String> {
    let mut result = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut result) {
        Ok(_) => Ok(SequenceInstance::construct_string_sequence(&result)),
        Err(e) => Err(format!("{}", e))
    }
}

fn read_stdin_ppl(_workspace: &WorkSpace) -> Result<Value, String> {
    //  This function implements PPL's interpreter semantics on input: whereby a line starting
    //  with $ (and no equal sign) is a function definition. It reads and appends lines until it
    //  encounters an empty line.

    let mut result = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut function_definition_active = false;
    let mut line_number = 1;

    loop {
        if function_definition_active {
            print!("[{}] ", line_number);
            stdout.flush().map_err(|e| format!("{}", e))?;
        }
        let mut current_line = String::new();
        match stdin.read_line(&mut current_line) {
            Ok(_) => {
                result += current_line.as_str();
                if function_definition_active {
                    if current_line.as_str() == "\r\n" || current_line.trim() == "$" {
                        return Ok(SequenceInstance::construct_string_sequence(&result));
                    } else {
                        line_number += 1;
                    }
                } else if current_line.len() > 1 && &current_line[0..1] == "$" && !current_line.contains('=') {
                    function_definition_active = true;
                } else {
                    return Ok(SequenceInstance::construct_string_sequence(&result));
                }
            },
            Err(e) => return Err(format!("{}", e)),
        }
    }
}


fn show_features(workspace: &WorkSpace) -> Result<Value,String> {
    Ok(SequenceInstance::construct_string_sequence(&workspace.features.borrow().show()))
}

