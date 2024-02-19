//  This module implements the RootDataType

use std::fmt;
use crate::{execution::{value::{Value, sequence::SequenceInstance}, evaluate_identifier_by_value}, workspace::{WorkSpace, GeneralSymbol}};

use super::{metadata::{BuiltAlternates, MetaSequence, MetaStructure, Metadata, MetaAlternate}, help::Help};



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RootDataType {
    Int,
    Real,
    Dbl,
    Bool,
    Char,
    Structure(MetaStructure),
    Sequence(MetaSequence),
    Alternate(MetaAlternate),
    BuiltinAlternate(BuiltAlternates),
}

impl RootDataType {

    pub fn coerce(&self, v: &Value, workspace: &WorkSpace) -> Result<Value,String> {

        //  Before we can coerce the value, it has to be an atomic value

        match v {
            Value::Selector(selector) => return Err(format!("{} is not a value", selector)),
            Value::Symbol(symbol) => return self.coerce(&evaluate_identifier_by_value(symbol, workspace)?, workspace),
            Value::ValueByReference(cell_ref) => return self.coerce(&*cell_ref.cell.borrow().as_ref_to_value(), workspace),
            Value::LogicalLink(link) => return self.coerce(&*link.as_ref_to_value(), workspace),
            _ => {},
        }

        //  Now try to coerce the atomic value to the requested type

        match self {
            RootDataType::Int => Ok(Value::Int(v.as_i32()?)),
            RootDataType::Real =>  Ok(Value::Real(v.as_f32()?)),
            RootDataType::Dbl =>  Ok(Value::Double(v.as_f64()?)),
            RootDataType::Bool =>  Ok(Value::Bool(v.as_bool()?)),
            RootDataType::Char => Ok(Value::Char(v.as_char()?)),
            RootDataType::Sequence(seq) =>  match v {
                Value::Sequence(instance) => {
                    if instance.as_string() != seq.name.as_string() {
                        Err(format!("Cannot convert value from {} to {}", v.as_datatype()?,  seq.name.as_string()))
                    } else {
                        Ok(v.clone())
                    }
                },
                _ => {
                    if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                        dbg!(seq);
                        dbg!(v); 
                    }
                    Err(format!("Cannot convert value {} to {}", v, seq.name.as_string()))
                },
            },
            RootDataType::Structure(structure) =>  match v {
                Value::Structure(instance) => {
                    if instance.as_string() != structure.name.as_string() {
                        Err(format!("Cannot convert value from {} to {}", instance.as_datatype(), structure.name.as_string()))
                    } else {
                        Ok(v.clone())
                    }
                },
                _ => {
                    if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                        dbg!(structure);
                        dbg!(v); 
                    }
                    Err(format!("Cannot convert value {} to {}", v, structure.name.as_string()))
                }
            },
            RootDataType::Alternate(alternate) => {
                 if is_assignable_to_a_alternate(v, alternate, workspace)? {
                    Ok(v.clone())
                 } else {
                    if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                        dbg!(self);
                        dbg!(v); 
                    }
                    Err(format!("Cannot convert value"))
                }
            },
            RootDataType::BuiltinAlternate(alternate) => {
                if is_a_builtin_alternate(v, alternate, workspace)? {
                    Ok(v.clone())
                 } else {
                    if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                        dbg!(self);
                        dbg!(v); 
                    }
                    Err(format!("Cannot convert value"))
                 }
            },
        }
    }

    pub fn from_value(v: &Value, workspace: &WorkSpace) -> Result<RootDataType,String> {
        match v {
            Value::Empty => Err(format!("Empty value")),
            Value::Bool(_) => Ok(RootDataType::Bool),
            Value::Int(_) => Ok(RootDataType::Int),
            Value::Real(_) => Ok(RootDataType::Real),
            Value::Double(_) => Ok(RootDataType::Dbl),
            Value::Char(_) => Ok(RootDataType::Char),
            Value::Selector(sel) => Err(format!("{} is not a value", sel)),
            Value::Sequence(seq) => workspace.resolve_datatype(&seq.as_datatype().as_string()),
            Value::Structure(structure) => workspace.resolve_datatype(&structure.as_datatype().as_string()),
            Value::Symbol(symbolic_reference) => {
                if let GeneralSymbol::Variable(v) = symbolic_reference.as_symbol() {
                    RootDataType::from_value(&*v.cell.borrow().as_ref_to_value(), workspace)
                } else {
                    Err(format!("{} is not a value", symbolic_reference))
                }
            },
            Value::ValueByReference(c) => workspace.resolve_datatype(&c.datatype.as_string()),
            Value::LogicalLink(l) => RootDataType::from_value(&*l.as_ref_to_value(), workspace),
        }
    }

}

impl fmt::Display for RootDataType {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RootDataType::Int => write!(fmt, "Int"),
            RootDataType::Real => write!(fmt, "Real"),
            RootDataType::Dbl => write!(fmt, "Dbl"),
            RootDataType::Bool => write!(fmt, "Bool"),
            RootDataType::Char => write!(fmt, "Char"),
            _ => write!(fmt, "{}", (self as &dyn Metadata).as_definition()),
        }
    }
}

impl Metadata for RootDataType {
    fn as_definition(&self) -> String {
        match self {
            RootDataType::Int => String::from("int"),
            RootDataType::Real => String::from("real"),
            RootDataType::Dbl => String::from("dbl"),
            RootDataType::Bool => String::from("bool"),
            RootDataType::Char => String::from("char"),
            RootDataType::Structure(s) => s.as_definition(),
            RootDataType::Sequence(s) => s.as_definition(),
            RootDataType::Alternate(a) => a.as_definition(),
            RootDataType::BuiltinAlternate(a) => a.as_definition(),
        }
    }
}

impl Help for RootDataType {
    fn help_text(&self, _workspace: &WorkSpace) -> Option<String> {
        match self {
            RootDataType::Int => Some(format!("Holds integer values ranging from {} to {}", i32::MIN, i32::MAX)),
            RootDataType::Real => Some(format!("Holds floating point values ranging from {} to {}", f32::MIN, f32::MAX)),
            RootDataType::Dbl => Some(format!("Holds floating point values ranging from {} to {}", f64::MIN, f64::MAX)),
            RootDataType::Bool => Some(format!("Holds TRUE or FALSE")),
            RootDataType::Char => Some(format!("Holds a single Unicode character")),
            _ => None,
        }        
    }

    fn help_text_len(&self, workspace: &WorkSpace) -> usize {
        match self.help_text(workspace) {
            Some(help_text) => help_text.len(),
            None => 0,
        }
    }

    fn pretty_print(&self) -> String {
        match self {
            RootDataType::Int => String::from("int"),
            RootDataType::Real => String::from("real"),
            RootDataType::Dbl => String::from("dbl"),
            RootDataType::Bool => String::from("bool"),
            RootDataType::Char => String::from("char"),
            RootDataType::Structure(s) => format!("{}", s),
            RootDataType::Sequence(s) => format!("{}", s),
            RootDataType::Alternate(a) => format!("{}", a),
            RootDataType::BuiltinAlternate(b) => match b {
                BuiltAlternates::Structure => String::from("structure"),
                BuiltAlternates::Sequence => String::from("sequence"),
                BuiltAlternates::VSequence => String::from("v.sequence"),
                BuiltAlternates::General => String::from("general"),
            },
        }    
    }

    fn show_help(&self, name: &str, workspace: &WorkSpace) -> Result<Value,String> {
        match self.help_text(workspace) {
            Some(t) => Ok(SequenceInstance::construct_string_sequence(&format!("{} - {}", name, &t))),
            None => Err(format!("help not implemented for this type")),
        }
    }
}

pub fn is_an_instance_of(v: &Value, datatype: &RootDataType, workspace: &WorkSpace) -> Result<bool,String> {
    match datatype {
        RootDataType::Int => match v {
            Value::Int(_) => Ok(true),
            _ => Ok(false),
        },
        RootDataType::Real =>  match v {
            Value::Real(_) => Ok(true),
            _ => Ok(false),
        },
        RootDataType::Dbl =>  match v {
            Value::Double(_) => Ok(true),
            _ => Ok(false),
        },
        RootDataType::Bool =>  match v {
            Value::Bool(_) => Ok(true),
            _ => Ok(false),
        },
        RootDataType::Char =>  match v {
            Value::Char(_) => Ok(true),
            _ => Ok(false),
        },
        RootDataType::Sequence(datatype) =>  match v {
            Value::Sequence(v) => Ok(v.as_string() == datatype.name.as_string()),
            _ => Ok(false),
        },
        RootDataType::Structure(datatype) =>  match v {
            Value::Structure(v) => Ok(v.as_string() == datatype.name.as_string()),
            _ => Ok(false),
        },
        RootDataType::Alternate(alternate) => is_a_alternate(v, alternate, workspace),
        RootDataType::BuiltinAlternate(alternate) => is_a_builtin_alternate(v, alternate, workspace),
    }
}

fn is_a_alternate(v: &Value, alternate: &MetaAlternate, workspace: &WorkSpace) -> Result<bool,String> {
    for alternate_name in &alternate.members {
        let opt_symbol = workspace.try_get_datatype(alternate_name.as_string().as_str());
        if opt_symbol.is_some() {
            if is_an_instance_of(v, opt_symbol.unwrap().root_data_type(), workspace)? {
                return Ok(true);
            }
            continue;
        }
        return Err(format!("Datatype {} not found", alternate_name.as_string()));
    }

    Ok(false)
}

fn is_a_builtin_alternate(v: &Value, alternate: &BuiltAlternates, workspace: &WorkSpace) -> Result<bool,String> {
    match alternate {
        BuiltAlternates::Structure => {
            if let Value::Structure(_) = v {
                Ok(true)
            } else {
                Ok(false)
            }
        },
        BuiltAlternates::Sequence => {
            if let Value::Sequence(_) = v {
                Ok(true)
            } else {
                Ok(false)
            }
        },
        BuiltAlternates::VSequence => {
            if let Value::Sequence(sequence) = v {
                let opt_symbol = workspace.try_get_datatype(sequence.as_string().as_str());
                match opt_symbol {
                    Some(_) => {
                        if let RootDataType::Sequence(seq) = opt_symbol.unwrap().root_data_type() {
                            return Ok(seq.upper_index_bound.is_none());
                        }
                    }
                    None => {},
                }
                Err(format!("{} is not a sequence", sequence.as_string()))
            } else {
                Ok(false)
            }
        },
        BuiltAlternates::General => Ok(true),
    }
}

pub fn is_assignable_to(v: &Value, receiving_datatype: &RootDataType, workspace: &WorkSpace) -> Result<bool,String> {
    match receiving_datatype {
        RootDataType::Int => match v {
            Value::Bool(_) => Ok(true),
            Value::Int(_) => Ok(true),
            Value::Real(_) => Ok(true),
            Value::Double(_) => Ok(true),
            Value::Char(c) => Ok(char::is_ascii_digit(c)),
           _ => {
                if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                    dbg!(&receiving_datatype);
                    dbg!(v);
                }
                Ok(false)
            },
        },
        RootDataType::Real =>  match v {
            Value::Bool(_) => Ok(true),
            Value::Int(_) => Ok(true),
            Value::Real(_) => Ok(true),
            Value::Double(_) => Ok(true),
            Value::Char(c) => Ok(char::is_ascii_digit(c)),
            _ => {
                if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                    dbg!(&receiving_datatype);
                    dbg!(v);
                }
                Ok(false)
            },
        },
        RootDataType::Dbl =>  match v {
            Value::Double(_) => Ok(true),
            Value::Char(c) => Ok(char::is_ascii_digit(c)),
            _ => {
                if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                    dbg!(&receiving_datatype);
                    dbg!(v);
                }
                Ok(false)
            },
        },
        RootDataType::Bool =>  match v {
            Value::Bool(_) => Ok(true),
            Value::Int(v) => Ok(*v == 0 || *v == 1),
            Value::Real(v) => Ok(*v == 0.0 || *v == 1.0),
            Value::Double(v) => Ok(*v == 0.0 || *v == 1.0),
            Value::Char(c) => Ok(c.to_ascii_lowercase() == 't' || c.to_ascii_lowercase() == 'f'),
            _ => {
                if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                    dbg!(&receiving_datatype);
                    dbg!(v);
                }
                Ok(false)
            },
        },
        RootDataType::Char =>  match v {
            Value::Bool(_) => Ok(true),
            Value::Char(_) => Ok(true),
            _ => {
                if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                    dbg!(&receiving_datatype);
                    dbg!(v);
                }
                Ok(false)
            },
        },
        RootDataType::Sequence(datatype) =>  match v {
            Value::Sequence(v) => Ok(v.as_string() == datatype.name.as_string()),
            _ => {
                if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                    dbg!(&receiving_datatype);
                    dbg!(v);
                }
                Ok(false)
            },
        },
        RootDataType::Structure(datatype) =>  match v {
            Value::Structure(v) => Ok(v.as_string() == datatype.name.as_string()),
            _ => {
                if workspace.debug_options.borrow().is_set(&crate::workspace::debug::DebugOption::DataConversion) {
                    dbg!(&receiving_datatype);
                    dbg!(v);
                }
                Ok(false)
            },
        },
        RootDataType::Alternate(alternate) => is_assignable_to_a_alternate(v, alternate, workspace),
        RootDataType::BuiltinAlternate(alternate) => is_a_builtin_alternate(v, alternate, workspace),
    }
}

fn is_assignable_to_a_alternate(v: &Value, receiving_alternate: &MetaAlternate, workspace: &WorkSpace) -> Result<bool,String> {
    for alternate_name in &receiving_alternate.members {
        let opt_symbol = workspace.try_get_datatype(alternate_name.as_string().as_str());
        if opt_symbol.is_some() {
            if is_assignable_to(v, opt_symbol.unwrap().root_data_type(), workspace)? {
                return Ok(true);
            }
            continue;
        }
        return Err(format!("Datatype {} not found", alternate_name.as_string()));
    }

    Ok(false)
}

pub fn strongest_datatype<'a>(args: &[RootDataType], workspace: &WorkSpace) -> Result<RootDataType,String> {
    let mut strongest = &args[0];
    let string_datatype = workspace.resolve_datatype(&String::from("string"))?;

    for i in 1..args.len() {
        match strongest {
            RootDataType::Int => 
                match args[i] {
                    RootDataType::Int => strongest = &RootDataType::Int,
                    RootDataType::Real => strongest = &RootDataType::Real,
                    RootDataType::Dbl => strongest = &RootDataType::Dbl,
                    RootDataType::Bool => strongest = &RootDataType::Int,
                    RootDataType::Char => strongest = &RootDataType::Int,
                    RootDataType::Sequence(ref seq) => {
                        if seq.name.as_str() == "string" {
                            strongest = &RootDataType::Int;
                        } else {
                            return Err(format!("INT is arithmetically incompatble with Sequence"));
                        }
                    },
                    RootDataType::Structure(_) => return Err(format!("INT is arithmetically incompatble with Structure")),
                    RootDataType::Alternate(_) | RootDataType::BuiltinAlternate(_) => return Err(format!("INT is arithmetically incompatble with Alternate")),
                },
            RootDataType::Real => 
            match args[i] {
                RootDataType::Int => strongest = &RootDataType::Real,
                RootDataType::Real => strongest = &RootDataType::Real,
                RootDataType::Dbl => strongest = &RootDataType::Dbl,
                RootDataType::Bool => strongest = &RootDataType::Real,
                RootDataType::Char => strongest = &RootDataType::Real,
                RootDataType::Sequence(ref seq) => {
                    if seq.name.as_str() == "string" {
                        strongest = &RootDataType::Real;
                    } else {
                        return Err(format!("Real is arithmetically incompatble with Sequence"));
                    }
                },
                RootDataType::Structure(_) => return Err(format!("REAL is arithmetically incompatble with Structure")),
                RootDataType::Alternate(_) | RootDataType::BuiltinAlternate(_) => return Err(format!("REAL is arithmetically incompatble with Alternate")),
            },
            RootDataType::Dbl => 
            match args[i] {
                RootDataType::Int => strongest = &RootDataType::Dbl,
                RootDataType::Real => strongest = &RootDataType::Dbl,
                RootDataType::Dbl => strongest = &RootDataType::Dbl,
                RootDataType::Bool => strongest = &RootDataType::Dbl,
                RootDataType::Char => strongest = &RootDataType::Dbl,
                RootDataType::Sequence(ref seq) => {
                    if seq.name.as_str() == "string" {
                        strongest = &RootDataType::Dbl;
                    } else {
                        return Err(format!("Double is arithmetically incompatble with Sequence"));
                    }
                },
                RootDataType::Structure(_) => return Err(format!("DOUBLE is arithmetically incompatble with Structure")),
                RootDataType::Alternate(_) | RootDataType::BuiltinAlternate(_) => return Err(format!("DOUBLE is arithmetically incompatble with Alternate")),
            },
            RootDataType::Bool => 
            match args[i] {
                RootDataType::Int => strongest = &RootDataType::Int,
                RootDataType::Real => strongest = &RootDataType::Real,
                RootDataType::Dbl => strongest = &RootDataType::Dbl,
                RootDataType::Bool => strongest = &RootDataType::Bool,
                RootDataType::Char => strongest = &RootDataType::Bool,
                RootDataType::Sequence(ref seq) => {
                    if seq.name.as_str() == "string" {
                        strongest = &RootDataType::Bool;
                    } else {
                        return Err(format!("Bool is arithmetically incompatble with Sequence"));
                    }
                },
                RootDataType::Structure(_) => return Err(format!("BOOL is arithmetically incompatble with Structure")),
                RootDataType::Alternate(_) | RootDataType::BuiltinAlternate(_) => return Err(format!("BOOL is arithmetically incompatble with Alternate")),
            },
            RootDataType::Char => 
            match args[i] {
                RootDataType::Int => strongest = &RootDataType::Int,
                RootDataType::Real => strongest = &RootDataType::Real,
                RootDataType::Dbl => strongest = &RootDataType::Dbl,
                RootDataType::Bool => strongest = &RootDataType::Bool,
                RootDataType::Char =>  strongest = &RootDataType::Char,
                RootDataType::Sequence(ref seq) => {
                    if seq.name.as_str() == "string" {
                        strongest = &string_datatype;
                    } else {
                        return Err(format!("Char is incompatble with Sequence"));
                    }
                },
                RootDataType::Structure(_) => return Err(format!("CHAR is arithmetically incompatble with Structure")),
                RootDataType::Alternate(_) | RootDataType::BuiltinAlternate(_) => return Err(format!("CHAR is arithmetically incompatble with Alternate")),
            },
            _ => continue,
        }
    }

    Ok(strongest.clone())
}