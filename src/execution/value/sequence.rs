//  This holds the implementation of the SequenceInstance value

use std::fmt;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::{workspace::WorkSpace, 
    symbols::{metadata::{MetaDataTypeName, MetaSequence, MetaDataType}, 
    datatype::{RootDataType, strongest_datatype}}};

use super::recursion_detector::Cycle;
use super::{Cell, Value};


pub struct SequenceInstance {
    datatype: MetaDataTypeName,
    lower_bound: i32,
    recursion_pass: Cycle,
    values: RefCell<Vec<Rc<RefCell<Cell>>>>
}

impl SequenceInstance {
    pub fn as_datatype(&self) -> MetaDataTypeName {
        self.datatype.clone()
    }

    pub fn as_recursion_pass(&self) -> &Cycle {
        &self.recursion_pass
    }
    
    pub fn length(&self) -> i32 {
        self.values.borrow().len() as i32
    }

    pub fn lower_bound(&self) -> i32 {
        self.lower_bound
    }
}

impl Clone for SequenceInstance {
    fn clone(&self) -> Self {
        let mut cloned_values = Vec::new();
        for cell in &*self.values.borrow() {
            cloned_values.push(Rc::new(RefCell::new(Cell::clone(&*cell.borrow()))));
        }
        SequenceInstance { 
            datatype: self.datatype.clone(), 
            lower_bound: self.lower_bound.clone(), 
            recursion_pass: Cycle::new(),
            values: RefCell::new(cloned_values) }
    }
}

impl SequenceInstance {
    pub fn access_cell_by_value(&self, index: i32, workspace: &WorkSpace) -> Result<(),String> {
        let value = self.values.borrow()[self.check_and_normalize_index(index)?].borrow().contents.value.borrow().clone();
        workspace.push_value(&value);
        Ok(())
    }

    pub fn access_cell_by_reference(&self, index: i32, workspace: &WorkSpace) -> Result<(),String> {
        let normalized_index = self.check_and_normalize_index(index)?;
        let my_rootdatatype = workspace.resolve_datatype(&self.as_datatype().as_string())?;
        if let RootDataType::Sequence(seq) = my_rootdatatype {
            let value = Value::ValueByReference(super::CellReference { datatype: seq.member_type.clone(), cell: self.values.borrow()[normalized_index].clone()});
            workspace.push_value(&value);
            Ok(())
        } else {
            panic!("internal error");
        }
    }

    pub fn as_string(&self) -> String {
        self.datatype.as_string()
    }

    pub fn as_values(&self) -> Ref<'_,Vec<Rc<RefCell<super::Cell>>>> {
        self.values.borrow()
    }

    pub fn check_and_normalize_index(&self, index: i32) -> Result<usize,String> {
        let upper_bound = self.lower_bound + (self.values.borrow().len() as i32) - 1;
        if index >= self.lower_bound && index <= upper_bound {
            return Ok((index - self.lower_bound) as usize);
        } else {
            Err(format!("{} is out of bounds", index))
        }
    }

    pub fn compare(&self, other: &SequenceInstance, workspace: &WorkSpace) -> Result<i32,String> {
        let my_values = &*self.values.borrow();
        let other_values = &*other.values.borrow();
        let mut my_iter = my_values.iter();
        let mut other_iter = other_values.iter();

        loop {
            let opt_my_item = my_iter.next();
            let opt_other_item = other_iter.next();

            match (opt_my_item, opt_other_item) {
                (Some(my_item), Some(other_item)) => {
                    let my_datatype = RootDataType::from_value(&*my_item.borrow().as_contents().value.borrow(), workspace)?;
                    let other_datatype = RootDataType::from_value(&*other_item.borrow().as_contents().value.borrow(), workspace)?;
                    let datatypes = [my_datatype, other_datatype];
                    let result_type = strongest_datatype(&datatypes, workspace)?;

                    match result_type {
                        RootDataType::Int => {
                            if my_item.borrow().as_contents().value.borrow().as_i32()? < other_item.borrow().as_contents().value.borrow().as_i32()? {
                                return Ok(-1);
                            } else if my_item.borrow().as_contents().value.borrow().as_i32()? > other_item.borrow().as_contents().value.borrow().as_i32()? {
                                return Ok(1);
                            }
                        },
                        RootDataType::Real => {
                            if my_item.borrow().as_contents().value.borrow().as_f32()? < other_item.borrow().as_contents().value.borrow().as_f32()? {
                                return Ok(-1);
                            } else if my_item.borrow().as_contents().value.borrow().as_f32()? > other_item.borrow().as_contents().value.borrow().as_f32()? {
                                return Ok(1);
                            }
                        },
                        RootDataType::Dbl => {
                            if my_item.borrow().as_contents().value.borrow().as_f64()? < other_item.borrow().as_contents().value.borrow().as_f64()? {
                                return Ok(-1);
                            } else if my_item.borrow().as_contents().value.borrow().as_f64()? > other_item.borrow().as_contents().value.borrow().as_f64()? {
                                return Ok(1);
                            }
                        },
                        RootDataType::Bool => todo!(),
                        RootDataType::Char => {
                            if my_item.borrow().as_contents().value.borrow().as_char()? < other_item.borrow().as_contents().value.borrow().as_char()? {
                                return Ok(-1);
                            } else if my_item.borrow().as_contents().value.borrow().as_char()? > other_item.borrow().as_contents().value.borrow().as_char()? {
                                return Ok(1);
                            }
                        },
                        RootDataType::Structure(_) => {
                            if let Value::Structure(ref my_structure) = &*my_item.borrow().as_contents().value.borrow() {
                                if let Value::Structure(ref other_structure) = &*other_item.borrow().as_contents().value.borrow() {
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
                            if let Value::Sequence(ref my_sequence) = &*my_item.borrow().as_contents().value.borrow() {
                                if let Value::Sequence(ref other_sequence) = &*other_item.borrow().as_contents().value.borrow() {
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

    pub fn concat(left: &SequenceInstance, right: &SequenceInstance) -> Result<Value,String> {
        let mut catenated_values = Vec::new();
        for v in &*left.values.borrow() {
            catenated_values.push(Cell::new(v.borrow().as_ref_to_value().clone()));
        }
        for v in &*right.values.borrow() {
            catenated_values.push(Cell::new(v.borrow().as_ref_to_value().clone()));
        }
        Ok(Value::Sequence(SequenceInstance { 
            datatype: left.datatype.clone(), 
            lower_bound: left.lower_bound, 
            recursion_pass: Cycle::new(),
            values: RefCell::new(catenated_values) }))
    }

    pub fn construct(sequence_datatype: &Rc<MetaDataType>, args: &Vec<Value>, workspace: &WorkSpace) -> Result<Value,String> {
  
        //  Get the sequence's member type and validate the values are compatible with it
    
        let mut coerced_values = Vec::new();
    
        let sequence_root_datatype = sequence_datatype.root_data_type();
        if let RootDataType::Sequence(ref seq) = sequence_root_datatype {
            if seq.upper_index_bound.is_some() && args.len() != (seq.upper_index_bound.unwrap() - seq.lower_index_bound + 1) as usize {
                return Err(format!("{} requires {} elements", sequence_datatype.as_string(), seq.upper_index_bound.unwrap() - seq.lower_index_bound + 1));
            }
            let opt_member = workspace.try_get_datatype(&seq.member_type.as_string().as_str());
            if opt_member.is_none() {
                return Err(format!("Datatype {} is no longer defined", seq.member_type.as_string()));
            }
            let member = opt_member.unwrap();
            let member_datatype = member.root_data_type();
            for arg in args {
                coerced_values.push(member_datatype.coerce(arg, workspace)?);
            }
            Ok(Value::Sequence(SequenceInstance::from_string(&sequence_datatype.as_string(), seq.lower_index_bound, coerced_values)))
        } else {
            panic!("internal error");
        }
    
    
    }
    
    pub fn construct_string_sequence(v: &str) -> Value {
        let mut characters = Vec::new();
        characters.reserve(v.len());
        for c in v.chars() {
            characters.push(Cell::new(Value::Char(c)));
        }
        Value::Sequence(SequenceInstance::new(MetaDataTypeName::from_str("string"), 1, characters))
    }

    pub fn from_string(name: &String, lower_bound: i32, values: Vec<Value>) -> SequenceInstance {
        let mut cells = Vec::new();
        for value in values {
            cells.push(Cell::new(value))
        }
        SequenceInstance { 
            datatype: MetaDataTypeName::from_string(name), 
            lower_bound: lower_bound, 
            recursion_pass: Cycle::new(),
            values: RefCell::new(cells) }
    }

/*     pub fn get_reference_to_cell(&self, index: i32) -> Result<Value,String> {
        let normalized_index = self.check_and_normalize_index(index)?;
        Ok(Value::ValueByReference(self.values.borrow()[normalized_index].clone()))
    }
 */
    pub fn make(seq: &MetaSequence, count: i32, value: &Value, workspace: &WorkSpace) -> Result<Value,String> {
        let member_datatype = workspace.resolve_datatype(&seq.member_type.as_string())?;
        let member_value = member_datatype.coerce(value, workspace)?;
        let mut sequence_values = Vec::new();
        for _ in 0..count {
            sequence_values.push(super::Cell::new(member_value.clone()));
        }
        Ok(Value::Sequence(SequenceInstance { 
                                datatype: MetaDataTypeName::from_string(&seq.name.as_string()), 
                                lower_bound: seq.lower_index_bound, 
                                recursion_pass: Cycle::new(),
                                values: RefCell::new(sequence_values) }))
    }

    pub fn new(datatype: MetaDataTypeName, lower_bound: i32, values: Vec<Rc<RefCell<Cell>>>) -> SequenceInstance {
        SequenceInstance { 
            datatype: datatype, 
            lower_bound: lower_bound, 
            recursion_pass: Cycle::new(),
            values: RefCell::new(values) }
    }
}


impl  fmt::Display for SequenceInstance {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.datatype.as_string().as_str() == "string" {
            for c in &*self.values.borrow() {
                write!(fmt, "{}", c.borrow().contents.value.borrow())?;
            }
        } else {
            let mut separator = "";
            write!(fmt, "[")?;
    
            for c in &*self.values.borrow() {
                    write!(fmt, "{}{}", separator, c.borrow().contents.value.borrow())?;
                separator = ", ";
            }
    
            write!(fmt, "]")?;
        }
        Ok(())
    }
}
