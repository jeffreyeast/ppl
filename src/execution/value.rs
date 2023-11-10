//  This module implements the runtime value mechanism

use std::fmt;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::parser::tree::ReferenceNode;
use crate::symbols::help::Help;
use crate::symbols::metadata::MetaDataTypeName;
use crate::workspace::GeneralSymbol;
use crate::{workspace::WorkSpace, symbols::{metadata::MetaDataType, name::Name, datatype::RootDataType}};

use self::recursion_detector::Cycle;
use self::sequence::SequenceInstance;
use self::structure::{StructureInstance, SelectorInstance};

pub mod conversion;
pub mod debug;
pub mod format;
pub mod recursion_detector;
pub mod sequence;
pub mod structure;

/* 
    +------------------------------------+
    |                                    |
    |               Cell                 |      Cells own ValueEnvelopes own Values.  
    |                                    |
    |   +============================+   |      Cells are addressable/referencable and are the subject of Value::ValueByReference.
    |   |                            |   |
    |   |      ValueEnvelope         |   |      ValueEnvelopes are replacable, and are what is exchanged in by-reference assignments.
    |   |                            |   |
    |   |   +--------------------+   |   |      Values are replaceable and are what is exchanged in by-value assignments.
    |   |   |                    |   |   |
    |   |   |                    |   |   |
    |   |   |      Value         |   |   |
    |   |   |                    |   |   |
    |   |   |                    |   |   |
    |   |   +--------------------+   |   |
    |   |                            |   |
    |   +============================+   |
    |                                    |
    +------------------------------------+
*/
#[derive(Clone)]
pub enum Value {
    Empty,
    Bool(bool),
    Int(i32),
    Real(f32),
    Double(f64),
    Char(char),
    Structure(StructureInstance),
    Sequence(SequenceInstance),
    Selector(SelectorInstance),
    Symbol(SymbolicReference),
    ValueByReference(CellReference),
    LogicalLink(Rc<ValueEnvelope>)
}   

impl Value {
    // pub fn access_by_reference(&self, index: &Value) -> Result<Value,String> {
    //     match  self {
    //         Value::Sequence(seq) => seq.access_cell_by_reference(index.as_i32()?),
    //         Value::Structure(structure) => {
    //             if let &Value::Selector(ref selector) = index {
    //                 structure.access_field_by_reference(selector)
    //             } else {
    //                 Err(format!("{} is not a field selector", index))
    //             }
    //         },
    //         _ => Err(format!("{} is not indexable", self)),
    //     }
    // }

    pub fn as_datatype(&self) -> Result<String,String> {
        match self {
            Value::Empty => Ok(String::from("Null")),
            Value::Bool(_) => Ok(String::from("Bool")),
            Value::Int(_) => Ok(String::from("Int")),
            Value::Real(_) => Ok(String::from("Real")),
            Value::Double(_) => Ok(String::from("Dbl")),
            Value::Char(_) => Ok(String::from("Char")),
            Value::Sequence(s) => Ok(s.as_datatype().as_string()),
            Value::Structure(s) => Ok(s.as_string()),
            Value::Symbol(s) => 
                match s.as_symbol() {
                    GeneralSymbol::Variable(v) => v.cell.borrow().as_ref_to_value().as_datatype(),
                    _ => Err(format!("Not a user datatype")),
                },
            Value::ValueByReference(c) => c.cell.borrow().as_ref_to_value().as_datatype(),
            Value::LogicalLink(l) => l.as_ref_to_value().as_datatype(),
            _ => Err(format!("Not a user datatype")),
        }
    }

    // pub fn create_from_reference(r: &ReferenceNode, workspace: &WorkSpace) -> Result<Value,String> {
    //     match Value::from_reference(r, workspace) {
    //         Ok(v) => Ok(v),
    //         Err(_) => {
    //             workspace.add_variable(r.as_name().as_str(), VariableDescription::new(Value::Empty));
    //             Value::from_reference(r, workspace)
    //         }
    //     }
    // }

    pub fn from_reference(r: &ReferenceNode, workspace: &WorkSpace) -> Result<Value,String> {
        Ok(Value::Symbol( SymbolicReference { referred_name: r.as_name(), resolution: workspace.try_get_any(r.as_string().as_str()) }))
    }

    // pub fn access_cell_by_reference(&self, index: &Value) -> Result<Value,String> {
    //     match  self {
    //         Value::Sequence(seq) => seq.access_cell_by_reference(index.as_i32()?),
    //         Value::Structure(structure) => {
    //             if let &Value::Selector(ref selector) = index {
    //                 structure.access_field_by_reference(selector)
    //             } else {
    //                 Err(format!("{} is not a field selector", index))
    //             }
    //         },
    //         _ => Err(format!("{} is not indexable", self)),
    //     }
    // }
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(v) => write!(fmt, "{}", v),
            Value::Char(c) => write!(fmt, "{}", c),
            Value::Double(_) | Value::Real(_) => {
                let result = self.as_string();
                if result.contains('.') {
                    write!(fmt, "{}", self.as_string())
                } else {
                    write!(fmt, "{}.", self.as_string())
                }
            },
            Value::Empty => Ok(()),
            Value::Int(v) => write!(fmt, "{}", v),
            Value::Symbol(v) => write!(fmt, "{}", v),
            Value::Sequence(v) => {
                if  v.as_recursion_pass().has_not_been_processed() {
                    write!(fmt, "{}", v)
                } else {
                    write!(fmt, "...")
                }
            },
            Value::Selector(v) => {
                let open_brace = r#"{"#;
                let close_brace = r#"}"#;
                write!(fmt, "{} from {}", v.member, open_brace)?;
                let mut separator = "";
                for name in &v.structures {
                    write!(fmt, "{}{}", separator, name)?;
                    separator = ", ";
                }
                write!(fmt, "{}", close_brace)
            },
            Value::Structure(v) => {
                if v.as_recursion_pass().has_not_been_processed() {
                    write!(fmt, "{}", v)
                } else {
                    write!(fmt, "...")
                }
            },
            Value::ValueByReference(c) => {
                write!(fmt, "{}", c.cell.borrow().as_ref_to_value())
            },
            Value::LogicalLink(l) => {
             //   if l.as_recursion_pass().has_not_been_processed() {
                    write!(fmt, "{}", l.as_ref_to_value())
             //   } else {
             //       write!(fmt, "...")
             //   }
            },
        }
    }
}

impl Help for Value {
    fn help_text(&self, workspace: &WorkSpace) -> Option<String> {
        RootDataType::from_value(self, workspace).unwrap().help_text(workspace)
    }

    fn pretty_print(&self) -> String {
        self.as_string()
    }

    fn show_help(&self, name: &str, workspace: &WorkSpace) -> Result<Value,String> {
        RootDataType::from_value(self, workspace).unwrap().show_help(name, workspace)
    }
}

#[derive(Debug, Clone)]
pub struct ValueEnvelope {
    recursion_pass: Cycle,
    value: RefCell<Value>,
}

impl Help for ValueEnvelope {
    fn help_text(&self, workspace: &WorkSpace) -> Option<String> {
        self.value.borrow().help_text(workspace)        
    }

    fn pretty_print(&self) -> String {
        self.as_ref_to_value().as_string()    
    }

    fn show_help(&self, name: &str, workspace: &WorkSpace) -> Result<Value,String> {
        self.value.borrow().show_help(name, workspace)
    }
}

impl fmt::Display for ValueEnvelope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.borrow().fmt(f)
    }
}

impl ValueEnvelope {
    pub fn as_recursion_pass(&self) -> &Cycle {
        &self.recursion_pass
    }

    pub fn as_ref_to_value(&self) -> Ref<'_,Value> {
        self.value.borrow()
    }

    pub fn new (value: Value) -> Rc<ValueEnvelope> {
        Rc::new(ValueEnvelope { recursion_pass: Cycle::new(), value: RefCell::new(value) })
    }

    pub fn set_value(&self, value: &Value) {
        *self.value.borrow_mut() = value.clone();
    }
}


pub struct Cell {
    contents:   Rc<ValueEnvelope>,
}

impl Help for Cell {
    fn help_text(&self, workspace: &WorkSpace) -> Option<String> {
        self.contents.help_text(workspace)
    }

    fn pretty_print(&self) -> String {
        self.as_ref_to_value().as_string()    
    }

    fn show_help(&self, name: &str, workspace: &WorkSpace) -> Result<Value,String> {
        self.contents.show_help(name, workspace)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.contents.fmt(f)
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        match &*self.as_ref_to_value() {
            Value::ValueByReference(_) => Cell { contents: self.contents.clone() },
            v => Cell { contents: ValueEnvelope::new(v.clone())},
        }
    }
}

impl Cell {
    pub fn as_contents(&self) -> Rc<ValueEnvelope> {
        self.contents.clone()
    }

    pub fn new(value: Value) -> Rc<RefCell<Cell>> {
        Rc::new(RefCell::new(Cell { contents: ValueEnvelope::new(value)}))
    }

    pub fn new_by_reference(v: Rc<ValueEnvelope>) -> Rc<RefCell<Cell>> {
        Rc::new(RefCell::new(Cell { contents: Rc::new(ValueEnvelope { recursion_pass: Cycle::new(), value: RefCell::new(Value::LogicalLink(v.clone())) })}))
    }

    pub fn as_ref_to_value(&self) -> Ref<'_,Value> {
        self.contents.as_ref_to_value()
    }

    pub fn set_reference(&mut self, r: Rc<ValueEnvelope>) {
        self.set_value(&Value::LogicalLink(r.clone())).expect("set_value failed");
    }

    pub fn set_value(&self, value: &Value) -> Result<(),String> {
        self.contents.set_value(Cell::validate_value(value)?);
        Ok(())
    }

    pub fn validate_value(value: &Value) -> Result<&Value,String> {
        match value {
            Value::Selector(sel) => Err(format!("{} is not a value", sel.member)),
            Value::Symbol(symbol) => {
                match symbol.as_symbol() {
                    GeneralSymbol::Unresolved(_) => Err(format!("{} not found", symbol.as_string())),
                    _ => panic!("internal error"),
                }
            },
            Value::ValueByReference(_) => todo!(),
            _ => {
                Ok(value)
            },
        }
    }
}


#[derive(Debug,Clone)]
pub struct CellReference {
    pub datatype: MetaDataTypeName,
    pub cell: Rc<RefCell<Cell>>,
}


#[derive(Clone)]
pub struct SymbolicReference {
    referred_name: Name,
    resolution: GeneralSymbol,
}

impl SymbolicReference {
    pub fn as_string(&self) -> String {
        self.referred_name.as_string()
    }

    pub fn as_symbol(&self) -> &GeneralSymbol {
        &self.resolution
    }
}

impl fmt::Display for SymbolicReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.resolution)
    }
}



pub fn construct(datatype: &Rc<MetaDataType>, args: &Vec<Value>, workspace: &WorkSpace) -> Result<Value,String> {
    match &datatype.root_data_type() {
        RootDataType::Int | RootDataType::Real | RootDataType::Dbl | RootDataType::Bool | RootDataType::Char => construct_atomic(datatype, args),
        RootDataType::Structure(_) => StructureInstance::construct(datatype, args, workspace),
        RootDataType::Sequence(_) => SequenceInstance::construct(datatype, args, workspace),
        _ => Err(format!("{} is not a constructor", datatype.as_string())),
    }
}

fn construct_atomic(datatype: &Rc<MetaDataType>, args: &Vec<Value>) -> Result<Value,String> {
    if args.len() != 1 {
        Err(format!("{}() takes 1 argument", datatype.as_string()))
    } else {
        match datatype.root_data_type() {
            RootDataType::Int => Ok(Value::Int(args[0].as_i32()?)),
            RootDataType::Real =>  Ok(Value::Real(args[0].as_f32()?)),
            RootDataType::Dbl =>  Ok(Value::Double(args[0].as_f64()?)),
            RootDataType::Bool => Ok(Value::Bool(args[0].as_bool()?)),
            RootDataType::Char => Ok(Value::Char(args[0].as_char()?)),
            _ => panic!("internal error"),
        }
    }
}
