//  This module holds the PPL system functions for comparing user objects

use crate::{workspace::WorkSpace, symbols::{metadata::{FormalArgument, FunctionArgumentList, ArgumentMechanism, MetaDataTypeName, FunctionClass, FunctionImplementation, FunctionDescription}, datatype::{RootDataType, strongest_datatype, self}, name::Name}, execution::value::Value};


pub fn init(workspace: &WorkSpace) {
    workspace.add_system_function(
        "==", 
        FunctionDescription { 
            name: Name::from_str("is_an_instance_of"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("value"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") },
                FormalArgument { name: Name::from_str("data_type"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("string") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(is_an_instance_of)),
            help_text: String::from("Is an instance of a type") });
        
    workspace.add_system_function(
        "=", 
        FunctionDescription { 
            name: Name::from_str("eq"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("general") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(eq)),
            help_text: String::from("Compare two values for equality") });

    workspace.add_system_function(
        "<", 
        FunctionDescription { 
            name: Name::from_str("less"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("general") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(less)),
            help_text: String::from("Compare two values, true if the first is less than the second") });

    workspace.add_system_function(
        "<=", 
        FunctionDescription { 
            name: Name::from_str("lesseq"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("general") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(lesseq)),
            help_text: String::from("Compare two values, true if the first is less than or equal to the second") });

    workspace.add_system_function(
        ">", 
        FunctionDescription { 
            name: Name::from_str("gr"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("general") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(gr)),
            help_text: String::from("Compare two values, true if the first is greater than the second") });

    workspace.add_system_function(
        ">=", 
        FunctionDescription { 
            name: Name::from_str("greq"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("general") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(greq)),
            help_text: String::from("Compare two values, true if the first is greater than or equal to the second") });

    workspace.add_system_function(
        "#", 
        FunctionDescription { 
            name: Name::from_str("noteq"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("general") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(noteq)),
            help_text: String::from("Compare two values for inequality") });

    workspace.add_system_function(
        "&", 
        FunctionDescription { 
            name: Name::from_str("and"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("bool") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("bool") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(and)),
            help_text: String::from("Logical and of two values") });

    workspace.add_system_function(
        "!", 
        FunctionDescription { 
            name: Name::from_str("or"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue, datatype: MetaDataTypeName::from_str("bool") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("bool") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(or)),
            help_text: String::from("Logical or of two values") });
        
    workspace.add_system_function(
        "and", 
        FunctionDescription { 
            name: Name::from_str("and"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("bool") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("bool") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(and)),
            help_text: String::from("Logical and of two values") });

    workspace.add_system_function(
        "eq", 
        FunctionDescription { 
            name: Name::from_str("eq"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(eq)),
            help_text: String::from("Compare two values for equality") });
        
    workspace.add_system_function(
        "gr", 
        FunctionDescription { 
            name: Name::from_str("gr"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(gr)),
            help_text: String::from("Compare two values, true if the first is greater than the second") });
        
    workspace.add_system_function(
        "greq", 
        FunctionDescription { 
            name: Name::from_str("greq"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(greq)),
            help_text: String::from("Compare two values, true if the first is greater than or equal to the second") });
        
    workspace.add_system_function(
        "instance", 
        FunctionDescription { 
            name: Name::from_str("instance"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("value"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") },
                FormalArgument { name: Name::from_str("data_type"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("string") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(is_an_instance_of)),
            help_text: String::from("Is an instance of a type") });
        
    workspace.add_system_function(
        "less", 
        FunctionDescription { 
            name: Name::from_str("less"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(less)),
            help_text: String::from("Compare two values, true if the first is less than the second") });

    workspace.add_system_function(
        "lesseq", 
        FunctionDescription { 
            name: Name::from_str("lesseq"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(lesseq)),
            help_text: String::from("Compare two values, true if the first is less than or equal to the second") });

    workspace.add_system_function(
        "noteq", 
        FunctionDescription { 
            name: Name::from_str("noteq"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("general") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(noteq)),
            help_text: String::from("Compare two values for inequality") });

    workspace.add_system_function(
        "or", 
        FunctionDescription { 
            name: Name::from_str("or"), 
            arguments: FunctionArgumentList::Fixed(vec![
                FormalArgument { name: Name::from_str("left"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("bool") }, 
                FormalArgument { name: Name::from_str("right"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("bool") }]), 
            local_variables: None,
            return_value: Some(MetaDataTypeName::from_str("bool")), 
            implementation_class: FunctionImplementation::System(FunctionClass::Diadic(or)),
            help_text: String::from("Logical or of two values") });
}



fn and(left: &Value, right: &Value, _workspace: &WorkSpace) -> Result<Value,String> {
    Ok(Value::Bool(left.as_bool()? && right.as_bool()?))
}

fn eq(left: &Value, right: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    match eq_internal(left, right, workspace) {
        Ok(result) => Ok(result),
        Err(_) => Ok(Value::Bool(false)),
    }
}

fn eq_internal(left: &Value, right: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    match (left, right) {
        (Value::Sequence(left_sequence), Value::Sequence(right_sequence)) => return Ok(Value::Bool(left_sequence.compare(right_sequence, workspace)? == 0)),
        (Value::Structure(left_structure), Value::Structure(right_structure)) => return Ok(Value::Bool(left_structure.compare(right_structure, workspace)? == 0)),
        _ => {},
    }
    let value_list = [RootDataType::from_value(left, workspace)?, RootDataType::from_value(right, workspace)?];
    let result_type = strongest_datatype(&value_list, workspace)?;
    match  result_type {
        RootDataType::Bool => Ok(Value::Bool(left.as_bool()? == right.as_bool()?)),
        RootDataType::Char => Ok(Value::Bool(left.as_char()? == right.as_char()?)),
        RootDataType::Dbl => Ok(Value::Bool(left.as_f64()? == right.as_f64()?)),
        RootDataType::Int => Ok(Value::Bool(left.as_i32()? == right.as_i32()?)),
        RootDataType::Real => Ok(Value::Bool(left.as_f32()? == right.as_f32()?)),
        _ => Err(format!("unsupported datatype for comparison")),
    }
}

fn gr(left: &Value, right: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    match (left, right) {
        (Value::Sequence(left_sequence), Value::Sequence(right_sequence)) => {
            return Ok(Value::Bool(left_sequence.compare(right_sequence, workspace)? > 0));
        },
        (Value::Structure(left_structure), Value::Structure(right_structure)) => {
            return Ok(Value::Bool(left_structure.compare(right_structure, workspace)? > 0));
        },
        _ => {},
    }
    let value_list = [RootDataType::from_value(left, workspace)?, RootDataType::from_value(right, workspace)?];
    let result_type = strongest_datatype(&value_list, workspace)?;
    match  result_type {
        RootDataType::Char => Ok(Value::Bool(left.as_char()? > right.as_char()?)),
        RootDataType::Dbl => Ok(Value::Bool(left.as_f64()? > right.as_f64()?)),
        RootDataType::Int => Ok(Value::Bool(left.as_i32()? > right.as_i32()?)),
        RootDataType::Real => Ok(Value::Bool(left.as_f32()? > right.as_f32()?)),
        _ => Err(format!("unsupported datatype for comparison")),
    }
}

fn greq(left: &Value, right: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    Ok(Value::Bool(!less(left, right, workspace)?.as_bool()?))
}

fn is_an_instance_of(v: &Value, data_type: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    let data_type_name = data_type.as_string();
    let target_datatype_reference = workspace.try_get_datatype(&data_type_name);
    if target_datatype_reference.is_some() {
        return Ok(Value::Bool(datatype::is_an_instance_of(v, target_datatype_reference.unwrap().root_data_type(), workspace)?));
    }
    Err(format!("Datatype {} not found", data_type_name))
}

fn less(left: &Value, right: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    match (left, right) {
        (Value::Sequence(left_sequence), Value::Sequence(right_sequence)) => {
            return Ok(Value::Bool(left_sequence.compare(right_sequence, workspace)? < 0));
        },
        (Value::Structure(left_structure), Value::Structure(right_structure)) => {
            return Ok(Value::Bool(left_structure.compare(right_structure, workspace)? < 0));
        },
        _ => {},
    }
    let value_list = [RootDataType::from_value(left, workspace)?, RootDataType::from_value(right, workspace)?];
    let result_type = strongest_datatype(&value_list, workspace)?;
    match  result_type {
        RootDataType::Char => Ok(Value::Bool(left.as_char()? < right.as_char()?)),
        RootDataType::Dbl => Ok(Value::Bool(left.as_f64()? < right.as_f64()?)),
        RootDataType::Int => Ok(Value::Bool(left.as_i32()? < right.as_i32()?)),
        RootDataType::Real => Ok(Value::Bool(left.as_f32()? < right.as_f32()?)),
        _ => Err(format!("unsupported datatype for comparison")),
    }
}

fn lesseq(left: &Value, right: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    Ok(Value::Bool(!gr(left, right, workspace)?.as_bool()?))
}

fn noteq(left: &Value, right: &Value, workspace: &WorkSpace) -> Result<Value,String> {
    Ok(Value::Bool(!eq(left, right, workspace)?.as_bool()?))
}

fn or(left: &Value, right: &Value, _workspace: &WorkSpace) -> Result<Value,String> {
    Ok(Value::Bool(left.as_bool()? || right.as_bool()?))
}



