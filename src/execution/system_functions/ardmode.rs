//  Graphics functions from the implementation node

use crate::execution::value::Value;
use crate::graphics::GraphicsStates;
use crate::symbols::metadata::{FormalArgument, ArgumentMechanism, MetaDataTypeName};
use crate::workspace::WorkSpace;
use crate::symbols::{metadata::{FunctionClass, FunctionArgumentList, FunctionImplementation, FunctionDescription}, name::Name};




pub fn init(workspace: &WorkSpace) {
    if workspace.features.borrow().is_set(&crate::workspace::optional_features::Feature::Ardmode) {
        workspace.add_system_function(
            "ardmode", 
            FunctionDescription { 
                name: Name::from_str("ardmode"), 
                arguments: FunctionArgumentList::Fixed(vec![]),
                local_variables: None,
                return_value: None, 
                implementation_class: FunctionImplementation::System(FunctionClass::NullValuedNullary(ardmode)),
                help_text: String::from("Enter graphical output mode") });

        workspace.add_system_function(
            "clearscreen", 
            FunctionDescription { 
                name: Name::from_str("clearscreen"), 
                arguments: FunctionArgumentList::Fixed(vec![]),
                local_variables: None,
                return_value: None, 
                implementation_class: FunctionImplementation::System(FunctionClass::NullValuedNullary(clearscreen)),
                help_text: String::from("Clear the graphical surface") });

        workspace.add_system_function(
            "dottedvec", 
            FunctionDescription { 
                name: Name::from_str("dottedvec"), 
                arguments: FunctionArgumentList::Fixed(vec![
                    FormalArgument { name: Name::from_str("dx"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }, 
                    FormalArgument { name: Name::from_str("dy"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
                local_variables: None,
                return_value: None, 
                implementation_class: FunctionImplementation::System(FunctionClass::NullValuedDiadic(dottedvec)),
                help_text: String::from("Draw a dotted vector through the given displacement") });
        
        workspace.add_system_function(
            "setlinewidth", 
            FunctionDescription { 
                name: Name::from_str("setlinewidth"), 
                arguments: FunctionArgumentList::Fixed(vec![
                    FormalArgument { name: Name::from_str("width"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
                local_variables: None,
                return_value: None, 
                implementation_class: FunctionImplementation::System(FunctionClass::NullValuedMonadic(setlinewidth)),
                help_text: String::from("Sets the width of vectors drawn with dottedvec and solidvec") });
                
        workspace.add_system_function(
            "setpoint", 
            FunctionDescription { 
                name: Name::from_str("setpoint"), 
                arguments: FunctionArgumentList::Fixed(vec![
                    FormalArgument { name: Name::from_str("x"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }, 
                    FormalArgument { name: Name::from_str("y"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
                local_variables: None,
                return_value: None, 
                implementation_class: FunctionImplementation::System(FunctionClass::NullValuedDiadic(setpoint)),
                help_text: String::from("Set the beam to the given coordinates") });

        workspace.add_system_function(
            "solidvec", 
            FunctionDescription { 
                name: Name::from_str("solidvec"), 
                arguments: FunctionArgumentList::Fixed(vec![
                    FormalArgument { name: Name::from_str("dx"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }, 
                    FormalArgument { name: Name::from_str("dy"), mechanism: ArgumentMechanism::ByValue,  datatype: MetaDataTypeName::from_str("arith") }]), 
                local_variables: None,
                return_value: None, 
                implementation_class: FunctionImplementation::System(FunctionClass::NullValuedDiadic(solidvec)),
                help_text: String::from("Draw a solid vector through the given displacement") });
                
        workspace.add_system_function(
            "ttymode", 
            FunctionDescription { 
                name: Name::from_str("ttymode"), 
                arguments: FunctionArgumentList::Fixed(vec![]),
                local_variables: None,
                return_value: None, 
                implementation_class: FunctionImplementation::System(FunctionClass::NullValuedNullary(ttymode)),
                help_text: String::from("Leave graphical output mode") });
            }

}

fn ardmode(workspace: &WorkSpace) -> Result<(),String> {
    let graphics_context = workspace.get_graphics_context();
    {
        let mut graphics_state = graphics_context.graphics_state.lock().unwrap();
        match *graphics_state {
            GraphicsStates::Ardmode => return Ok(()),
            GraphicsStates::Idle => (),
            GraphicsStates::Ttymode => {
                *graphics_state = GraphicsStates::Ardmode;
                return Ok(());
            },
        }
    }

    graphics_context.create_application(1200, 1200, "PPL")?;
    Ok(())
}

fn clearscreen(workspace: &WorkSpace) -> Result<(),String> {
    let graphics_context = workspace.get_graphics_context();
    if *graphics_context.graphics_state.lock().unwrap() != GraphicsStates::Ardmode {
        return Err(format!("Not in graphical mode"));
    }

    graphics_context.clearscreen()?;
    Ok(())
}

fn dottedvec(x: &Value, y: &Value, workspace: &WorkSpace) -> Result<(),String> {
    let graphics_context = workspace.get_graphics_context();
    if *graphics_context.graphics_state.lock().unwrap() != GraphicsStates::Ardmode {
        return Err(format!("Not in graphical mode"));
    }
    graphics_context.dottedvec(x.as_i32()?, y.as_i32()?)?;
    Ok(())
}

fn setlinewidth(width: &Value, workspace: &WorkSpace) -> Result<(),String> {
    let graphics_context = workspace.get_graphics_context();
    if *graphics_context.graphics_state.lock().unwrap() != GraphicsStates::Ardmode {
        return Err(format!("Not in graphiscal mode"));
    }
    graphics_context.setlinewidth(width.as_i32()?)?;
    Ok(())
}

fn setpoint(x: &Value, y: &Value, workspace: &WorkSpace) -> Result<(),String> {
    let graphics_context = workspace.get_graphics_context();
    if *graphics_context.graphics_state.lock().unwrap() != GraphicsStates::Ardmode {
        return Err(format!("Not in graphical mode"));
    }
    graphics_context.setpoint(x.as_i32()?, y.as_i32()?)?;
    Ok(())
}

fn solidvec(x: &Value, y: &Value, workspace: &WorkSpace) -> Result<(),String> {
    let graphics_context = workspace.get_graphics_context();
    if *graphics_context.graphics_state.lock().unwrap() != GraphicsStates::Ardmode {
        return Err(format!("Not in graphical mode"));
    }
    graphics_context.solidvec(x.as_i32()?, y.as_i32()?)?;
    Ok(())
}

fn ttymode(workspace: &WorkSpace) -> Result<(),String> {
    let graphics_context = workspace.get_graphics_context();
    let mut graphics_state = graphics_context.graphics_state.lock().unwrap();
    match *graphics_state {
        GraphicsStates::Ardmode => {
            *graphics_state = GraphicsStates::Ttymode;
            return Ok(());
        },
        GraphicsStates::Idle => return Ok(()),
        GraphicsStates::Ttymode => return Ok(()),
    }
}