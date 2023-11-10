//  This module holds the PPL system function implementations for arithmetic

use crate::{workspace::WorkSpace, symbols::{metadata::{FunctionDescription, FormalArgument, ArgumentMechanism, MetaDataTypeName, FunctionImplementation, FunctionClass, FunctionArgumentList}, name::Name, datatype::{RootDataType, strongest_datatype}}, execution::value::Value};


pub fn init(workspace: &WorkSpace) {
    workspace.add_system_function(
        "+", 
        FunctionDescription { 
            name: Name::from_str("add"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("arith") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("ARITH")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(add)),
            help_text: String::from("Add two values") });

    workspace.add_system_function(
        "+", 
        FunctionDescription { 
            name: Name::from_str("plus"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("operand"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("ARITH")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Monadic(plus)),
            help_text: String::from("Noop") });
        
    workspace.add_system_function(
        "-", 
        FunctionDescription { 
            name: Name::from_str("subtract"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("arith")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(subtract)),
            help_text: String::from("Subtract two values") });

    workspace.add_system_function(
        "-", 
        FunctionDescription { 
            name: Name::from_str("minus"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("operand"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("arith")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Monadic(minus)),
            help_text: String::from("Negate a value") });
                
    workspace.add_system_function(
        "*", 
        FunctionDescription { 
            name: Name::from_str("multiply"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("arith") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("arith")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(multiply)),
            help_text: String::from("Multiply two values") });
                
    workspace.add_system_function(
        "/", 
        FunctionDescription { 
            name: Name::from_str("divide"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("arith") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("arith")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(divide)),
            help_text: String::from("Divide two values") });

    workspace.add_system_function(
        "^", 
        FunctionDescription { 
            name: Name::from_str("power"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("arith") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("ARITH")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(power)),
            help_text: String::from("Raise a value to a power") });
    
        
    workspace.add_system_function(
        "add", 
        FunctionDescription { 
            name: Name::from_str("add"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("arith") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("ARITH")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(add)),
            help_text: String::from("Add two values") });
                        
    workspace.add_system_function(
        "div", 
        FunctionDescription { 
            name: Name::from_str("div"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("arith")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(divide)),
            help_text: String::from("divide two values") });
                                
    workspace.add_system_function(
        "minus", 
        FunctionDescription { 
            name: Name::from_str("minus"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("operand"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("arith")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Monadic(minus)),
            help_text: String::from("Negate a value") });
                        
    workspace.add_system_function(
        "mul", 
        FunctionDescription { 
            name: Name::from_str("mul"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("arith")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(multiply)),
            help_text: String::from("Multiply two values") });
                
    workspace.add_system_function(
        "plus", 
        FunctionDescription { 
            name: Name::from_str("plus"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("operand"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("arith")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Monadic(plus)),
            help_text: String::from("Noop") });

    workspace.add_system_function(
        "power", 
        FunctionDescription { 
            name: Name::from_str("power"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("ARITH")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(power)),
            help_text: String::from("Raise a value to a power") });

    workspace.add_system_function(
        "sub", 
        FunctionDescription { 
            name: Name::from_str("sub"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("arith")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(subtract)),
            help_text: String::from("Subtract two values") });
}




fn add(left: &Value, right: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    let value_list = [RootDataType::from_value(left, workspace)?, RootDataType::from_value(right, workspace)?];
    let result_type = strongest_datatype(&value_list, workspace)?;
    match  result_type {
        RootDataType::Int => Ok(Value::Int(left.as_i32()? + right.as_i32()?)),
        RootDataType::Real => Ok(Value::Real(left.as_f32()? + right.as_f32()?)),
        RootDataType::Dbl => Ok(Value::Double(left.as_f64()? + right.as_f64()?)),
        _ => Err(format!("unsupported datatype for arithmetic")),
    }
}


fn divide(left: &Value, right: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    let value_list = [RootDataType::from_value(left, workspace)?, RootDataType::from_value(right, workspace)?];
    let result_type = strongest_datatype(&value_list, workspace)?;
    match  result_type {
        RootDataType::Int => {
            let r = right.as_i32()?;
            let l = left.as_i32()?;
            if r == 0 {
                if l > 0 { 
                    Ok(Value::Int(i32::MAX))
                } else {
                    Ok(Value::Int(i32::MIN))
                }
            } else {
                Ok(Value::Int(l / r))
            }
        },
        RootDataType::Real => Ok(Value::Real(left.as_f32()? / right.as_f32()?)),
        RootDataType::Dbl => Ok(Value::Double(left.as_f64()? / right.as_f64()?)),
        _ => Err(format!("unsupported datatype for arithmetic")),
    }
}

fn multiply(left: &Value, right: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    let value_list = [RootDataType::from_value(left, workspace)?, RootDataType::from_value(right, workspace)?];
    let result_type = strongest_datatype(&value_list, workspace)?;
    match  result_type {
        RootDataType::Int => Ok(Value::Int(left.as_i32()? * right.as_i32()?)),
        RootDataType::Real => Ok(Value::Real(left.as_f32()? * right.as_f32()?)),
        RootDataType::Dbl => Ok(Value::Double(left.as_f64()? * right.as_f64()?)),
        _ => Err(format!("unsupported datatype for arithmetic")),
    }
}

fn minus(operand: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    let value_list = [RootDataType::from_value(operand, workspace)?];
    let result_type = strongest_datatype(&value_list, workspace)?;
    match  result_type {
        RootDataType::Bool => Ok(Value::Bool(!operand.as_bool()?)),
        RootDataType::Int => Ok(Value::Int(-operand.as_i32()?)),
        RootDataType::Real => Ok(Value::Real(-operand.as_f32()?)),
        RootDataType::Dbl => Ok(Value::Double(-operand.as_f64()?)),
        _ => Err(format!("unsupported datatype for arithmetic")),
    }
}

fn plus(operand: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    let value_list = [RootDataType::from_value(operand, workspace)?];
    let result_type = strongest_datatype(&value_list, workspace)?;
    match  result_type {
        RootDataType::Int => Ok(Value::Int(operand.as_i32()?)),
        RootDataType::Real => Ok(Value::Real(operand.as_f32()?)),
        RootDataType::Dbl => Ok(Value::Double(operand.as_f64()?)),
        _ => Err(format!("unsupported datatype for arithmetic")),
    }
}

fn power(left: &Value, right: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    let value_list = [RootDataType::from_value(left, workspace)?, RootDataType::from_value(right, workspace)?];
    let result_type = strongest_datatype(&value_list, workspace)?;
    match  result_type {
        RootDataType::Int => Ok(Value::Int(left.as_i32()?.pow(right.as_i32()? as u32))),
        RootDataType::Real => Ok(Value::Real(left.as_f32()?.powf(right.as_f32()?))),
        RootDataType::Dbl => Ok(Value::Double(left.as_f64()?.powf(right.as_f64()?))),
        _ => Err(format!("unsupported datatype for arithmetic")),
    }
}

fn subtract(left: &Value, right: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    let value_list = [RootDataType::from_value(left, workspace)?, RootDataType::from_value(right, workspace)?];
    let result_type = strongest_datatype(&value_list, workspace)?;
    match  result_type {
        RootDataType::Int => Ok(Value::Int(left.as_i32()? - right.as_i32()?)),
        RootDataType::Real => Ok(Value::Real(left.as_f32()? - right.as_f32()?)),
        RootDataType::Dbl => Ok(Value::Double(left.as_f64()? - right.as_f64()?)),
        _ => Err(format!("unsupported datatype for arithmetic")),
    }
}



