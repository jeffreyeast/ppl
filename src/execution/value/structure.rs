//  This module holds the implementation of  StructureInstance

use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use crate::{workspace::WorkSpace, symbols::{datatype::{RootDataType, strongest_datatype}, metadata::{MetaDataTypeName, MetaDataType, SelectorDescription}, name::Name}};

use super::{Cell, Value, recursion_detector::Cycle};


#[derive(Clone)]
pub struct SelectorInstance {
    pub member: Name,
    pub structures: Vec<Name>,
}

impl SelectorInstance {
    pub fn from_description(desc: &SelectorDescription) -> SelectorInstance {
        SelectorInstance { member: Name::from_name(&desc.member), structures: desc.structures.borrow().clone() }
    }
}

impl fmt::Display for SelectorInstance {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.member.as_string())
    }
}

#[derive(Clone)]
pub struct StructureInstance {
    datatype: MetaDataTypeName,
    recursion_pass: Cycle,
    members: Vec<StructureInstanceMember>
}

impl StructureInstance {
    pub fn as_datatype(&self) -> MetaDataTypeName {
        self.datatype.clone()
    }

    pub fn as_recursion_pass(&self) -> &Cycle {
        &self.recursion_pass
    }

    pub fn length(&self) -> i32 {
        self.members.len() as i32
    }
}

pub struct StructureInstanceMember {
    name: Name,
    datatype: MetaDataTypeName,
    cell: Rc<RefCell<Cell>>,
}

impl Clone for StructureInstanceMember {
    fn clone(&self) -> Self {
        StructureInstanceMember { 
            name:  self.name.clone(), 
            datatype: self.datatype.clone(), 
            cell: self.cell.clone()
        }
    }
}

impl StructureInstanceMember {
    pub fn as_string(&self) -> String {
        self.name.as_string()
    }

    pub fn as_reference(&self) -> Value {
        Value::ValueByReference(super::CellReference { datatype: self.datatype.clone(), cell: self.cell.clone() })
    }

    pub fn as_value(&self) -> Value {
        self.cell.borrow().as_ref_to_value().clone()
    }
}



impl StructureInstance {
    pub fn access_field_by_value(&self, selector: &SelectorInstance, workspace: &WorkSpace) -> Result<(),String> {
        for target in &selector.structures {
            if target.as_string() == self.as_string() {
                for member in &self.members {
                    if member.as_string() == selector.member.as_string() {
                        workspace.push_value(&*member.cell.borrow().contents.value.borrow());
                        return Ok(());
                    }
                }
            }
        }
        Err(format!("{} does not have a field {}", self.as_string(), selector.member.as_string()))
    }

    pub fn access_field_by_reference(&self, selector: &SelectorInstance, workspace: &WorkSpace) -> Result<(),String> {
        for target in &selector.structures {
            if target.as_string() == self.as_string() {
                for member in &self.members {
                    if member.as_string() == selector.member.as_string() {
                        let value = Value::ValueByReference(super::CellReference { datatype: member.datatype.clone(), cell: member.cell.clone()});
                        workspace.push_value(&value);
                        return Ok(());
                    }
                }
            }
        }
        Err(format!("{} does not have a field {}", self.as_string(), selector.member.as_string()))
    }

    pub fn as_string(&self) -> String {
        self.datatype.as_string()
    }

    pub fn as_values(&self) -> &Vec<StructureInstanceMember> {
        &self.members
    }

    pub fn compare(&self, other: &StructureInstance, workspace: &WorkSpace) -> Result<i32,String> {
        let mut my_iter = self.members.iter();
        let mut other_iter = other.members.iter();

        loop {
            let opt_my_item = my_iter.next();
            let opt_other_item = other_iter.next();

            match (opt_my_item, opt_other_item) {
                (Some(my_item), Some(other_item)) => {
                    let my_datatype = RootDataType::from_value(&*my_item.cell.borrow().as_contents().value.borrow(), workspace)?;
                    let other_datatype = RootDataType::from_value(&*other_item.cell.borrow().as_contents().value.borrow(), workspace)?;
                    let datatypes = [my_datatype, other_datatype];
                    let result_type = strongest_datatype(&datatypes, workspace)?;

                    match result_type {
                        RootDataType::Int => {
                            if my_item.cell.borrow().as_contents().value.borrow().as_i32()? < other_item.cell.borrow().as_contents().value.borrow().as_i32()? {
                                return Ok(-1);
                            } else if my_item.cell.borrow().as_contents().value.borrow().as_i32()? > other_item.cell.borrow().as_contents().value.borrow().as_i32()? {
                                return Ok(1);
                            }
                        },
                        RootDataType::Real => {
                            if my_item.cell.borrow().as_contents().value.borrow().as_f32()? < other_item.cell.borrow().as_contents().value.borrow().as_f32()? {
                                return Ok(-1);
                            } else if my_item.cell.borrow().as_contents().value.borrow().as_f32()? > other_item.cell.borrow().as_contents().value.borrow().as_f32()? {
                                return Ok(1);
                            }
                        },
                        RootDataType::Dbl => {
                            if my_item.cell.borrow().as_contents().value.borrow().as_f64()? < other_item.cell.borrow().as_contents().value.borrow().as_f64()? {
                                return Ok(-1);
                            } else if my_item.cell.borrow().as_contents().value.borrow().as_f64()? > other_item.cell.borrow().as_contents().value.borrow().as_f64()? {
                                return Ok(1);
                            }
                        },
                        RootDataType::Bool => todo!(),
                        RootDataType::Char => {
                            if my_item.cell.borrow().as_contents().value.borrow().as_char()? < other_item.cell.borrow().as_contents().value.borrow().as_char()? {
                                return Ok(-1);
                            } else if my_item.cell.borrow().as_contents().value.borrow().as_char()? > other_item.cell.borrow().as_contents().value.borrow().as_char()? {
                                return Ok(1);
                            }
                        },
                        RootDataType::Structure(_) => {
                            if let Value::Structure(ref my_structure) = &*my_item.cell.borrow().as_contents().value.borrow() {
                                if let Value::Structure(ref other_structure) = &*other_item.cell.borrow().as_contents().value.borrow() {
                                    let result = my_structure.compare(other_structure, workspace)?;
                                    if result != 0 {
                                        return Ok(result);
                                    }
                                } else {
                                    return Err(format!("Invalid comparison"));
                                }
                            } else {
                                return Err(format!("Invalid comparison"));
                            }
                        },
                        RootDataType::Sequence(_) => {
                            if let Value::Sequence(ref my_sequence) = &*my_item.cell.borrow().as_contents().value.borrow() {
                                if let Value::Sequence(ref other_sequence) = &*other_item.cell.borrow().as_contents().value.borrow() {
                                    let result = my_sequence.compare(other_sequence, workspace)?;
                                    if result != 0 {
                                        return Ok(result);
                                    }
                                } else {
                                    return Err(format!("Invalid comparison"));
                                }
                            } else {
                                return Err(format!("Invalid comparison"));
                            }
                        },
                        _ => return Err(format!("Invalid comparison")),
                    }
                },
                (None, Some(_)) => return Ok(-1),
                (Some(_), None) => return Ok(1),
                (None, None) => return Ok(0),
            }
        }
    }

    pub fn construct(structure_datatype: &Rc<MetaDataType>, args: &Vec<Value>, workspace: &WorkSpace) -> Result<Value,String> {
  
        //  Get the structure's member types and validate the values are compatible
    
        let mut instance_fields = Vec::new();
        if let RootDataType::Structure(meta_structure) = structure_datatype.root_data_type() {
            if meta_structure.members.len() != args.len() {
                return Err(format!("{} has {} members, but only {} values were provided", structure_datatype.as_string(), meta_structure.members.len(), args.len()));
            }
    
            for i in 0..meta_structure.members.len() {
                let member = &meta_structure.members[i];
                let opt_member_datatype = workspace.try_get_datatype(&member.data_type.as_string().as_str());
                if opt_member_datatype.is_none() {
                    return Err(format!("Datatype {} is no longer defined", member.name.as_string()));
                }
                let member_datatype = opt_member_datatype.unwrap();
                let member_datatype_name = member_datatype.as_name().as_string();
                let member_datatype_rootdatatype = member_datatype.root_data_type();
                instance_fields.push(StructureInstanceMember { 
                    name: Name::from_string(&member.name.as_string()), 
                    datatype: MetaDataTypeName::from_string(&member_datatype_name),
                    cell: super::Cell::new(member_datatype_rootdatatype.coerce(&args[i], workspace)?)});
            }
    
            Ok(Value::Structure(StructureInstance::from_string(&structure_datatype.as_string(), instance_fields)))
        } else {
            Err(format!("internal error"))
        }
    }

    pub fn from_string(name: &String, members: Vec<StructureInstanceMember>) -> StructureInstance {
        StructureInstance { 
            datatype: MetaDataTypeName::from_string(name), 
            recursion_pass: Cycle::new(),
            members: members }
    }
}

impl  fmt::Display for StructureInstance {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut separator = "";
        write!(fmt, "[")?;

        for member in &self.members {
            write!(fmt, "{}{}", separator, member)?;
            separator = ", ";
        }

        write!(fmt, "]")
    }
}


impl  fmt::Display for StructureInstanceMember {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}:{}", self.name, self.cell.borrow().contents.value.borrow())?;
        Ok(())
    }
}
