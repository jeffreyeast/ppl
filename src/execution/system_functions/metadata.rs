//  This module holds PPL system functions that manipulate metadata

use std::io::{Write, Read};

use crate::{workspace::{WorkSpace, GeneralSymbol}, 
    symbols::{metadata::{FunctionDescription, FunctionArgumentList, FormalArgument, ArgumentMechanism, Metadata, MetaDataTypeName, FunctionImplementation, FunctionClass}, 
    name::Name, datatype::RootDataType}, 
    execution::value::{Value, sequence::SequenceInstance}};



pub fn init(workspace: &WorkSpace) {
    workspace.add_system_function(
        "?", 
        FunctionDescription { 
            name: Name::from_str("display"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("function"), mechanism: ArgumentMechanism::ByReference,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("string")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Monadic(display)),
            help_text: String::from("Display the source text of a function") });
             
    workspace.add_system_function(
        "%", 
        FunctionDescription { 
            name: Name::from_str("relocate"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("line"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("ARITH")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Monadic(relocate)),
            help_text: String::from("Designates a relocatable line number") });
   
    workspace.add_system_function(
        "binary", 
        FunctionDescription { 
            name: Name::from_str("binary"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("operator"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("string") }, 
                FormalArgument { name: Name::from_str("function"), mechanism: ArgumentMechanism::ByReference,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedDiadic(binary)),
            help_text: String::from("Associates the operator with a diadic function") });

    workspace.add_system_function(
        "display", 
        FunctionDescription { 
            name: Name::from_str("display"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("function"), mechanism: ArgumentMechanism::ByReference,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("string")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Monadic(display)),
            help_text: String::from("Display the source text of a function") });
                                                
    workspace.add_system_function(
        "edit", 
        FunctionDescription { 
            name: Name::from_str("edit"), 
            arguments: FunctionArgumentList::Varying(ArgumentMechanism::ByReference), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("general")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Varying(edit)),
            help_text: String::from("Edit a definition") });

    workspace.add_system_function(
        "erase", 
        FunctionDescription { 
            name: Name::from_str("erase"), 
            arguments: FunctionArgumentList::Varying(ArgumentMechanism::ByReference), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedVarying(erase)),
            help_text: String::from("Causes the meanings of the identifiers to be erased from the system. Everything is erased if called with no arguments.") });
                                
    workspace.add_system_function(
        "false", 
        FunctionDescription { 
            name: Name::from_str("false"), 
            arguments: FunctionArgumentList::Fixed(Vec::new()),
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Nullary(false_value)),
            help_text: String::from("Boolean value of false") });
                                        
    workspace.add_system_function(
        "help", 
        FunctionDescription { 
            name: Name::from_str("help"), 
            arguments: FunctionArgumentList::Varying(ArgumentMechanism::ByReference),
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("string")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Varying(help)),
            help_text: String::from(r#"Use HELP("<item>") for more information"#) });
                        
    workspace.add_system_function(
        "l.bound", 
        FunctionDescription { 
            name: Name::from_str("l.bound"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("operand"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("int")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Monadic(lower_bound)),
            help_text: String::from("Returns the lower bound of the operand. Non-sequences are defined to be 1.") });
                                
    workspace.add_system_function(
        "length", 
        FunctionDescription { 
            name: Name::from_str("length"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("operand"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("int")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Monadic(length)),
            help_text: String::from("Returns the number of components of the operand. If the operand is atomic, Length is defined to be one.") });
        
    workspace.add_system_function(
        "make", 
        FunctionDescription { 
            name: Name::from_str("make"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("sequence_type"), mechanism: ArgumentMechanism::ByReference, datatype: MetaDataTypeName::from_str("general") }, 
                FormalArgument { name: Name::from_str("count"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") },
                FormalArgument { name: Name::from_str("value"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("general")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Triadic(make)),
            help_text: String::from("Construct a sequence with {count} copies of {value}") });
                
    workspace.add_system_function(
        "read", 
        FunctionDescription { 
            name: Name::from_str("read"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("filename"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("string") }]), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedMonadic(read)),
            help_text: String::from("Loads the workspace from an ASCII file") });
                                                
    workspace.add_system_function(
        "reset", 
        FunctionDescription { 
            name: Name::from_str("reset"),
            arguments: FunctionArgumentList::Fixed(Vec::new()),
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedNullary(reset)),
            help_text: String::from("Erases all nests of function calls.") });
                
    workspace.add_system_function(
        "symbol.table", 
        FunctionDescription { 
            name: Name::from_str("symbol.table"), 
            arguments: FunctionArgumentList::Varying(ArgumentMechanism::ByReference), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedNullary(dump_symbol_table)),
            help_text: String::from("Dump the symbol table") });
                                        
    workspace.add_system_function(
        "true", 
        FunctionDescription { 
            name: Name::from_str("true"),
            arguments: FunctionArgumentList::Fixed(Vec::new()),
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Nullary(true_value)),
            help_text: String::from("Boolean false value") });
                
    workspace.add_system_function(
        "type", 
        FunctionDescription { 
            name: Name::from_str("type"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("operand"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("string")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Monadic(get_type)),
            help_text: String::from("Returns the data type of the operand") });
                
    workspace.add_system_function(
        "unary", 
        FunctionDescription { 
            name: Name::from_str("unary"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("operator"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("string") }, 
                FormalArgument { name: Name::from_str("function"), mechanism: ArgumentMechanism::ByReference,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedDiadic(unary)),
            help_text: String::from("Associates the operator with a diadic function") });
                                                                
    workspace.add_system_function(
        "version", 
        FunctionDescription { 
            name: Name::from_str("version"),
            arguments: FunctionArgumentList::Fixed(Vec::new()),
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("string")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Nullary(version)),
            help_text: String::from("Return PPL's version string") });
                
    workspace.add_system_function(
        "write", 
        FunctionDescription { 
            name: Name::from_str("write"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("filename"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("string") }]), 
            local_variables: None,
            return_value: None, 
            implementation_class: FunctionImplementation::System(FunctionClass::NullValuedMonadic(write)),
            help_text: String::from("Writes the contents of the workspace to an ASCII file") });
}



fn binary(operator: &Value, function: &Value, workspace: &WorkSpace) -> Result<(),String> {
    if let Some(f) = workspace.try_get_function(function.as_string().as_str()) {
        if let FunctionArgumentList::Fixed(ref args) = f.arguments {
            if args.len() == 2 {
                workspace.add_user_function_by_reference(operator.as_string().as_str(), f.clone());
                return Ok(());
            }
        }
        return Err(format!("{} is not a binary function", function));
    }
    Err(format!("{} not found", function))
}

fn display(function: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    match workspace.try_get_any(function.as_string().as_str()) {
        GeneralSymbol::Unresolved(_) => Err(format!("{} not found", function.as_string())),
        desc => Ok(SequenceInstance::construct_string_sequence(&format!("{}", desc.as_definition()))),
    }
}

fn dump_symbol_table(workspace: &WorkSpace) -> Result<(),String> {
    workspace.dump_symbol_table();
    Ok(())
}

fn edit(args: &Vec<Value>, workspace: &WorkSpace) -> Result<Value,String> {
    let text:String;
    match args.len() {
        0 => text = String::new(),
        1 => {
            let symbol = workspace.try_get_any(&args[0].as_string());
            match &symbol {
                GeneralSymbol::Datatype(dt) => text = dt.as_definition(),
                GeneralSymbol::Function(func) => text = func.as_source(),
                GeneralSymbol::Unresolved(_) => text = args[0].as_string(),
                _ => return Err(format!("{} cannot be editted", args[0])),
            }
        },
        _ => return Err(format!("EDIT takes at most one argument")),
    }

    let temp_directory = std::env::var("TEMP");
    let filename:String;
    match temp_directory {
        Ok(dir) => filename = dir + "\\function.tmp",
        Err(_) => filename = String::from("\\function.tmp"),
    }
    std::fs::File::create(filename.as_str())
        .map_err(|e| e.to_string())?
        .write_all(&text.as_bytes())
        .map_err(|e| e.to_string())?;
    std::process::Command::new("notepad")
        .arg(filename.as_str())
        .output()
        .map_err(|e| e.to_string())?;
    let mut editted_text = String::new();
    std::fs::File::open(filename.as_str())
        .map_err(|e| e.to_string())?
        .read_to_string(&mut editted_text)
        .map_err(|e| e.to_string())?;

    if args.len() == 1 {
        erase(&vec![args[0].clone()], workspace)?;
    }
    super::exec(&SequenceInstance::construct_string_sequence(editted_text.as_str()), workspace)
}

fn erase(args: &Vec<Value>, workspace: &WorkSpace) -> Result<(),String> {
    if args.len() == 0 {
        workspace.remove_all();
    } else {
        for arg in args {
            workspace.remove(&arg.as_string().as_str());
        }
    }
    Ok(())
}

fn false_value(_workspace: &WorkSpace) -> Result<Value, String> {
    Ok(Value::Bool(false))
}

fn get_type(operand: &Value, _workspace: &WorkSpace) -> Result<Value,String> {
    Ok(SequenceInstance::construct_string_sequence(&operand.as_datatype()?))
}

fn help(args: &Vec<Value>, workspace: &WorkSpace) -> Result<Value,String> {
    match args.len() {
        0 => workspace.help_all(),
        _ => workspace.help_one(&args[0].as_string()),
    }
}

fn length(operand: &Value, _workspace: &WorkSpace) -> Result<Value,String> {
    match operand {
        Value::Empty => Ok(Value::Int(0)),
        Value::Structure(s) => Ok(Value::Int(s.length())),
        Value::Sequence(s) => Ok(Value::Int(s.length())),
        Value::Selector(_) => Err(format!("Length is undefined for selectors")),
        _ => Ok(Value::Int(1)),
    }
}

fn lower_bound(operand: &Value, _workspace: &WorkSpace) -> Result<Value,String> {
    match operand {
        Value::Sequence(s) => Ok(Value::Int(s.lower_bound())),
        _ => Ok(Value::Int(1)),
    }
}

fn make(sequence_type: &Value, count: &Value, value: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    match workspace.try_get_datatype(&sequence_type.as_string()) {
        Some(datatype) => {
            if let RootDataType::Sequence(ref seq) = datatype.root_data_type() {
                SequenceInstance::make(seq, count.as_i32()?, value, workspace)
            } else {
                Err(format!("{} is not a sequence", sequence_type))
            }
        },
        None => Err(format!("{} not found", sequence_type)),
    }
}

fn read(filename: &Value, workspace: &WorkSpace) -> Result<(),String> {
    crate::workspace::io::read(&filename.as_string(), workspace)
}

fn relocate(line: &Value, _workspace: &WorkSpace) -> Result<Value,String> {
    Ok(line.clone())
}

fn reset(workspace: &WorkSpace) -> Result<(), String> {
    workspace.reset_function_state();
    Ok(())
}

fn true_value(_workspace: &WorkSpace) -> Result<Value, String> {
    Ok(Value::Bool(true))
}

fn unary(operator: &Value, function: &Value, workspace: &WorkSpace) -> Result<(),String> {
    let symbol_list = workspace.try_get_functions(function.as_string().as_str());
    for symbol in symbol_list {
        match symbol {
            GeneralSymbol::Function(f) => {
                if let FunctionArgumentList::Fixed(ref args) = f.arguments {
                    if args.len() == 1 {
                        workspace.add_user_function_by_reference(operator.as_string().as_str(), f.clone());
                        return Ok(());
                    }
                }
                return Err(format!("{} is not a unary function", f));
            },
            _ => return Err(format!("{} is not a unary function", function)),
        }
    }
    Err(format!("{} not found", function))
}

fn version(_workspace: &WorkSpace) -> Result<Value, String> {
    Ok(SequenceInstance::construct_string_sequence(&String::from("PPL T0.0")))
}

fn write(filename: &Value, workspace: &WorkSpace) -> Result<(),String> {
    crate::workspace::io::write(&filename.as_string(), workspace)
}



