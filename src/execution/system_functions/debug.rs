use crate::{workspace::WorkSpace, symbols::{metadata::{FunctionDescription, FunctionArgumentList, FunctionImplementation, FunctionClass, FormalArgument, ArgumentMechanism, MetaDataTypeName}, name::Name}, execution::value::{Value, sequence::SequenceInstance}};





pub fn init(workspace: &WorkSpace) {
    workspace.add_system_function(
        "break", 
        FunctionDescription { 
            name: Name::from_str("break"),
            arguments: FunctionArgumentList::Fixed(Vec::new()),
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedNullary(breakpoint)),
            help_text: String::from("Interrupts the currently executing function") });
                        
    workspace.add_system_function(
        "debug", 
        FunctionDescription { 
            name: Name::from_str("debug"), 
            arguments: FunctionArgumentList::Fixed(vec![]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("string")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Nullary(show_debug)),
            help_text: String::from(r#"Display debug options"#) });
                                
    workspace.add_system_function(
        "debug", 
        FunctionDescription { 
            name: Name::from_str("debug"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("operation"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("string") }, 
                FormalArgument { name: Name::from_str("option"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("string") }]), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedDiadic(debug)),
            help_text: String::from(r#"{"SET" | "CLEAR"} debug options {"LEX" | "PARSE"}"#) });
                                
    workspace.add_system_function(
        "stack.usage", 
        FunctionDescription { 
            name: Name::from_str("stack.usage"), 
            arguments: FunctionArgumentList::Fixed(vec![]),
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("int")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Nullary(stack_usage)),
            help_text: String::from(r#"Returns the amount of stack space consumed by the function, in kilobytes"#) });
                                                    
    workspace.add_system_function(
        "stop", 
        FunctionDescription { 
            name: Name::from_str("stop"), 
            arguments: FunctionArgumentList::Varying(ArgumentMechanism::ByReference), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedVarying(stop)),
            help_text: String::from("Sets breakpoints at the specified lines in the designated function.") });
                                
    workspace.add_system_function(
        "trace", 
        FunctionDescription { 
            name: Name::from_str("trace"), 
            arguments: FunctionArgumentList::Varying(ArgumentMechanism::ByReference), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedVarying(trace)),
            help_text: String::from("Trace execution of the specified function.") });
                                
    workspace.add_system_function(
        "unstop", 
        FunctionDescription { 
            name: Name::from_str("unstop"), 
            arguments: FunctionArgumentList::Varying(ArgumentMechanism::ByReference), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedVarying(unstop)),
            help_text: String::from("Removes breakpoints from the designated function") });
                                        
    workspace.add_system_function(
        "untrace", 
        FunctionDescription { 
            name: Name::from_str("untrace"), 
            arguments: FunctionArgumentList::Varying(ArgumentMechanism::ByReference), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedVarying(untrace)),
            help_text: String::from("Cease tracing execution of the specified function.") });
}


fn breakpoint(_workspace: &WorkSpace) -> Result<(),String> {
    Err(format!("Breakpoint"))
}

fn debug(operation: &Value, option: &Value, workspace: &WorkSpace) -> Result<(),String> {
    match operation.as_string().to_ascii_lowercase().as_str() {
        "on" | "set" => workspace.debug_options.borrow_mut().set_str(option.as_string().as_str())?,
        "off" | "clear" => workspace.debug_options.borrow_mut().clear_str(option.as_string().as_str())?,
        _ => return Err(format!("{} is not a valid debug operation", operation.as_string())),
    }
    Ok(())
}

fn show_debug(workspace: &WorkSpace) -> Result<Value,String> {
    let options = workspace.debug_options.borrow().show();
    Ok(SequenceInstance::construct_string_sequence(options.as_str()))
}

fn stack_usage(workspace: &WorkSpace) -> Result<Value,String> {
    Ok(Value::Int((workspace.get_stack_size() / 1024) as i32))
}

fn stop(args: &Vec<Value>, workspace: &WorkSpace) -> Result<(),String> {
    if args.len() == 0 {
        return Err(format!("Stop requires a function name"));
    }
    crate::execution::functions::stop(&args[0], &args[1..], workspace)
}

fn trace(args: &Vec<Value>, workspace: &WorkSpace) -> Result<(),String> {
    if args.len() == 0 {
        return Err(format!("Trace requires a function name"));
    }
    crate::execution::functions::trace(&args[0], &args[1..], workspace)
}
fn unstop(args: &Vec<Value>, workspace: &WorkSpace) -> Result<(),String> {
    if args.len() == 0 {
        return Err(format!("Unstop requires a function name"));
    }
    crate::execution::functions::unstop(&args[0], &args[1..], workspace)
}

fn untrace(args: &Vec<Value>, workspace: &WorkSpace) -> Result<(),String> {
    if args.len() == 0 {
        return Err(format!("Untrace requires a function name"));
    }
    crate::execution::functions::untrace(&args[0], &args[1..], workspace)
}



