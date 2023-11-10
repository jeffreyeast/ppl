//  This module holds the routines for defining objects and types

use std::cell::RefCell;
use std::rc::Rc;

use crate::parser::tree::{DefinitionNode, DefinitionType, SequenceDefinition, StructureDefinition};
use crate::symbols::datatype::RootDataType;
use crate::symbols::metadata::{MetaDataTypeName, MetaDataType, MetaSequence, MetaStructureMember, SelectorDescription, MetaStructure, FunctionDescription, MetaAlternate};
use crate::symbols::name::Name;
use crate::workspace::WorkSpace;





fn execute_alternate_definition(name: &String, alternates: &Vec<usize>, workspace: &WorkSpace) -> Result<(),String> {

    //  Assemble the alternates

    let mut resolved_alternates = Vec::new();
    for _i in 0..alternates.len() {
        let alternate_name = workspace.pop_value().as_string();
        resolved_alternates.push(MetaDataTypeName::from_string(&alternate_name));
    }

    //  And define the new datatype

    workspace.add_datatype(name.as_str(), 
        MetaDataType::from_string(&name, RootDataType::Alternate(MetaAlternate::new(name.as_str(), resolved_alternates ))));
    Ok(())
}

pub fn execute_definition(d: &DefinitionNode, workspace: &WorkSpace) -> Result<(),String> {
    if workspace.contains_any(d.as_string().as_str()) {
        return Err(format!("{} is already defined", d.as_string()));
    }

    match  d.get() {
        DefinitionType::Alternate(alternates) => {
            execute_alternate_definition(&d.as_string(), alternates, workspace)
        },
        DefinitionType::Function(function_definition) => {
            execute_function_definition(&d.as_string(), function_definition, workspace)
        },
        DefinitionType::Sequence(sequence_definition) => {
            execute_sequence_definition(&d.as_string(), sequence_definition, workspace)
        },
        DefinitionType::Structure(structure_definition) => {
            execute_structure_definition(&d.as_string(), structure_definition, workspace)
        },
    }
}

fn execute_function_definition(name: &String, def: &FunctionDescription, workspace: &WorkSpace) -> Result<(),String> {
    match workspace.try_get_any(name.as_str()) {
        crate::workspace::GeneralSymbol::Unresolved(_) => {
            workspace.add_user_function(name, def);
            Ok(())
        },
        _ => Err(format!("{} already exists", name)),
    }
}

fn execute_sequence_definition(name: &String, def: &SequenceDefinition, workspace: &WorkSpace) -> Result<(),String> {

    //  And define the new datatype

    workspace.add_datatype(name.as_str(), 
        MetaDataType::from_string(name, 
            RootDataType::Sequence(MetaSequence { name: Name::from_string(name), lower_index_bound: def.lower_bound, upper_index_bound: def.upper_bound, member_type: MetaDataTypeName::from_string(&def.root_datatype_name.as_string())})));
    Ok(())
}

fn execute_structure_definition(name: &String, def: &StructureDefinition, workspace: &WorkSpace) -> Result<(),String> {

    // Validate the members

    let mut meta_members = Vec::new();

    for member in &def.members {
        let datatype_name = member.datatype.as_string();
        let meta_member = MetaStructureMember { name: member.name.clone(), data_type: MetaDataTypeName::from_string(&datatype_name) };
        meta_members.push(meta_member);
    }

    //  And define the new datatype

    for meta_member in &meta_members {
        let selector_description = workspace.try_get_selector(&meta_member.name.as_string());
        match selector_description {
            Some(_) => {
                selector_description.unwrap().structures.borrow_mut().push(Name::from_string(name));
            },
            None => {
                workspace.add_selector(meta_member.name.as_str(), 
                    SelectorDescription { member: Name::from_name(&meta_member.name), structures: Rc::new(RefCell::new(vec![ Name::from_string(name)]))});
            },
        }
    }

    workspace.add_datatype(name.as_str(), 
        MetaDataType::from_string(name, 
            RootDataType::Structure(MetaStructure { name: Name::from_string(name), members: meta_members })));
    Ok(())
}
