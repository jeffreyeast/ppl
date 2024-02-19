//  This module defines the mechanisms for storing user values

use std::{fmt, collections::HashMap};
use std::cell::RefCell;
use std::rc::Rc;

use crate::{execution::{runtime::executable::Executable, value::{Value, Cell}},
    lexical::LineNumber,
    symbols::{ Name, help::Help}, 
    utility::convert_escape_sequences,
    workspace::WorkSpace};

use super::datatype::RootDataType;



pub trait Metadata {
    fn as_definition(&self) -> String;
}


#[derive(Debug,Clone)]
pub enum FunctionClass {
    Nullary(fn(&WorkSpace) -> Result<Value, String>),
    Monadic(fn(&Value,&WorkSpace) -> Result<Value, String>),
    Diadic(fn(&Value,&Value,&WorkSpace) -> Result<Value, String>),
    Triadic(fn(&Value,&Value,&Value,&WorkSpace) -> Result<Value, String>),
    Varying(fn(&Vec<Value>,&WorkSpace) -> Result<Value, String>),
    NullValuedNullary(fn(&WorkSpace) -> Result<(), String>),
    NullValuedMonadic(fn(&Value,&WorkSpace) -> Result<(), String>),
    NullValuedDiadic(fn(&Value,&Value,&WorkSpace) -> Result<(), String>),
    NullValuedTriadic(fn(&Value,&Value,&Value,&WorkSpace) -> Result<(), String>),
    NullValuedVarying(fn(&Vec<Value>,&WorkSpace) -> Result<(), String>),
}


#[derive(Debug,Clone)]
pub enum ArgumentMechanism  {
    ByValue,
    ByReference,
    ByReferenceCreateIfNeeded,
}

#[derive(Debug,Clone)]
pub struct FormalArgument {
    pub name: Name,
    pub mechanism: ArgumentMechanism,
    pub datatype: MetaDataTypeName,
}

#[derive(Debug,Clone)]
pub enum FunctionImplementation {
    System(FunctionClass),
    User(Rc<FunctionBody>),
}

#[derive(Clone)]
pub struct FunctionBody {
    pub executable: Rc<Executable>,
    pub labels:     HashMap<String,LineNumber>,         //  Statement line numbers
}

impl fmt::Display for FunctionBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.executable)
    }
}

impl fmt::Debug for FunctionBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.executable)
    }
}


#[derive(Debug,Clone)]
pub enum FunctionArgumentList {
    Fixed(Vec<FormalArgument>),
    Varying(ArgumentMechanism),
}

#[derive(Debug,Clone)]
pub struct FunctionDescription {
    pub name: Name,
    pub arguments: FunctionArgumentList,
    pub local_variables: Option<Vec<Name>>,
    pub return_value: Option<MetaDataTypeName>,
    pub implementation_class: FunctionImplementation,
    pub help_text: String,
}

impl FunctionDescription {
    
    pub fn as_source(&self) -> String {
        match &self.implementation_class {
            FunctionImplementation::System(_) => {
                let mut source = format!("${}(", self.name);
                match &self.arguments {
                    FunctionArgumentList::Fixed(ref args) => {
                        let mut separator = "";
                        for arg in args {
                            source += format!("{}{}", separator, arg).as_str();
                            separator = ", ";
                        }
                    },
                    FunctionArgumentList::Varying(_) => source += "...",
                }
                source += ")";
                if let Some(local_variables) = &self.local_variables {
                    let mut separator = "; ";
                    for local_variable in local_variables {
                        source += format!("{}{}", separator, local_variable).as_str();
                        separator = ", ";
                    }
                }
                source += "\n";
        
                source
        
            },
            FunctionImplementation::User(body) => String::from(body.executable.as_source()),
        }
    }

        pub fn as_string(&self) -> String {
            self.name.as_string()
        }

        pub fn is_compatible_function(&self, actual_argument_list_length: usize) -> bool {
            if let FunctionArgumentList::Fixed(fixed) = &self.arguments {
                fixed.len() == actual_argument_list_length
            } else {
                true
            }
        }
    }

impl fmt::Display for FunctionDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.implementation_class {
            FunctionImplementation::System(_) => write!(f, "{}", self.as_source()),
            FunctionImplementation::User(body) => {
                write!(f, "${}({})", self.name, self.arguments)?;
                match &self.local_variables {
                    Some(locals) => {
                        write!(f, "; ")?;
                        let mut separator = "";
                        for v in locals {
                            write!(f, "{}{}", separator, v)?;
                            separator = ", ";
                        }
                        writeln!(f, "")?;
                    },
                    None => writeln!(f, "")?,
                }
                let mut line_number = 0;
                for line in body.executable.as_source().lines() {
                    if line_number > 0 {
                        writeln!(f, "[{}] {}", line_number, convert_escape_sequences(line))?;
                    }
                    line_number += 1;
                }
                Ok(())
            },
        }
    }
}

impl fmt::Display for FunctionArgumentList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionArgumentList::Fixed(ref args) => {
                let mut separator = "";
                for arg in args {
                    write!(f, "{}{}", separator, arg)?;
                    separator = ", ";
                }
            },
            FunctionArgumentList::Varying(_) => write!(f, "...")?,
        }
        Ok(())
    }
}

impl fmt::Display for FormalArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mechanism {
            ArgumentMechanism::ByValue => {},
            ArgumentMechanism::ByReference | ArgumentMechanism::ByReferenceCreateIfNeeded => write!(f, "$")?,
        }
        write!(f, "{}:{}", self.name, self.datatype)
    }
}

impl Metadata for FunctionDescription {
    fn as_definition(&self) -> String {
        format!("{}", self)
    }
}


#[derive(Debug)]
pub struct SelectorDescription {
    pub member: Name,
    pub structures: Rc<RefCell<Vec<Name>>>,
}

impl fmt::Display for SelectorDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.member)        
    }
}

impl Metadata for SelectorDescription {
    fn as_definition(&self) -> String {
        let mut result = String::new();
        let mut separator = "";
    
        for structure in &*self.structures.borrow() {
            result += format!("{}{}:{}", separator, structure, self.member).as_str();
            separator = ", ";
        }
    
        result
    }
}

impl SelectorDescription {
    pub fn as_string(&self) -> String {
        self.member.as_string()
    }
}

#[derive(Debug)]
pub struct VariableDescription {
    pub cell: Rc<RefCell<Cell>>,
}

impl fmt::Display for VariableDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cell.borrow().as_ref_to_value())
    }
}

impl Metadata for VariableDescription {
    fn as_definition(&self) -> String {
        self.cell.borrow().as_ref_to_value().as_datatype().unwrap()
    }
}

impl VariableDescription {
    pub fn new(value: Value) -> VariableDescription {
        VariableDescription { cell: Cell::new(value)}
    }
}


//  The MetaType definitions describe the structure of an object, but not its value(s)

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaDataTypeName {
    name: Name,
}

impl MetaDataTypeName {
    pub fn as_string(&self) -> String {
        self.name.as_string()
    }

    pub fn from_str(s: &str) -> MetaDataTypeName {
        MetaDataTypeName { name: Name::from_str(s) }
    }

    pub fn from_string(s: &String) -> MetaDataTypeName {
        MetaDataTypeName { name: Name::from_string(s) }
    }
}

impl fmt::Display for MetaDataTypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaDataType {
    name: MetaDataTypeName,
    root_data_type: RootDataType,
}

impl fmt::Display for MetaDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Metadata for MetaDataType {
    fn as_definition(&self) -> String {
        self.root_data_type.as_definition()
    }
}

impl MetaDataType {
    pub fn as_name(&self) -> MetaDataTypeName {
        self.name.clone()
    }

    pub fn as_string(&self) -> String {
        self.name.as_string()
    }

    pub fn root_data_type(&self) -> &RootDataType {
        &self.root_data_type
    }

    pub fn from_str(s: &str, root_data_type: RootDataType) -> MetaDataType {
        MetaDataType { name: MetaDataTypeName::from_str(s), root_data_type: root_data_type}
    }

    pub fn from_string(s: &String, root_data_type: RootDataType) -> MetaDataType {
        MetaDataType { name: MetaDataTypeName::from_string(s), root_data_type: root_data_type }
    }
}

impl Help for MetaDataType {
    fn help_text(&self, workspace: &WorkSpace) -> Option<String> {
        self.root_data_type.help_text(workspace)
    }

    fn help_text_len(&self, workspace: &WorkSpace) -> usize {
        match self.help_text(workspace) {
            Some(help_text) => help_text.len(),
            None => 0,
        }
    }

    fn pretty_print(&self) -> String {
        self.root_data_type.pretty_print()    
    }

    fn show_help(&self, name: &str, workspace: &WorkSpace) -> Result<Value,String> {
        self.root_data_type.show_help(name, workspace)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaAlternate {
    pub name: Name,
    pub members: Vec<MetaDataTypeName>,
}

impl fmt::Display for MetaAlternate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}=", self.name)?;
        let mut separator = "";
        for member in &self.members {
            write!(f, "{}{}", separator, member)?;
            separator = " ! ";
        }
        Ok(())
    }
}

impl Metadata for MetaAlternate {
    fn as_definition(&self) -> String {
        {
            format!("{}", self)
        }
    }
}

impl MetaAlternate {
    pub fn new(name: &str, members: Vec<MetaDataTypeName>) -> MetaAlternate {
        MetaAlternate { name: Name::from_str(name), members: members }
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltAlternates {
    Structure,
    Sequence,
    VSequence,
    General,
}

impl Metadata for BuiltAlternates {
    fn as_definition(&self) -> String {
        match self {
            BuiltAlternates::Structure => String::from("structure"),
            BuiltAlternates::Sequence => String::from("sequence"),
            BuiltAlternates::VSequence => String::from("v.sequence"),
            BuiltAlternates::General => String::from("general"),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaStructureMember {
    pub name: Name,
    pub data_type: MetaDataTypeName,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaStructure {
    pub name: Name,
    pub members: Vec<MetaStructureMember>,
}

impl fmt::Display for MetaStructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${} = [", self.name)?;
        let mut separator = "";
        for member in &self.members {
            write!(f, "{}{}:{}", separator, member.name, member.data_type)?;
            separator = ", ";
        }
        writeln!(f, "]")
    }
}

impl Metadata for MetaStructure {
    fn as_definition(&self) -> String {
        format!("{}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaSequence {
    pub name: Name,
    pub lower_index_bound: i32,
    pub upper_index_bound: Option<i32>,
    pub member_type: MetaDataTypeName,
}

impl fmt::Display for MetaSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let upper_index_bound = match self.upper_index_bound {
            Some(_) => format!("{}", self.upper_index_bound.unwrap()),
            None => String::from(""),
        };
        writeln!(f, "${} = [{}:{}] {}", self.name, self.lower_index_bound, upper_index_bound, self.member_type)
    }
}

impl Metadata for MetaSequence {
    fn as_definition(&self) -> String {
        format!("{}", self)
    }
}

pub fn init(workspace: &WorkSpace) {
    workspace.add_datatype("int", MetaDataType::from_str("int", RootDataType::Int));
    workspace.add_datatype("real", MetaDataType::from_str("real", RootDataType::Real));
    workspace.add_datatype("dbl", MetaDataType::from_str("dbl", RootDataType::Dbl));
    workspace.add_datatype("bool", MetaDataType::from_str("bool", RootDataType::Bool));
    workspace.add_datatype("char", MetaDataType::from_str("char", RootDataType::Char));
    workspace.add_datatype("string", MetaDataType::from_str("string",
        RootDataType::Sequence(MetaSequence { name: Name::from_str("string"), lower_index_bound: 1, upper_index_bound: None, member_type: MetaDataTypeName::from_str("char") })));
    workspace.add_datatype("tuple", MetaDataType::from_str("tuple",
        RootDataType::Sequence(MetaSequence { name: Name::from_str("tuple"), lower_index_bound: 1, upper_index_bound: None, member_type: MetaDataTypeName::from_str("general") })));
    workspace.add_datatype("arith", MetaDataType::from_str("arith",
        RootDataType::Alternate(MetaAlternate::new("arith",
                                vec![MetaDataTypeName::from_str("int"),
                                     MetaDataTypeName::from_str("real"),
                                     MetaDataTypeName::from_str("dbl")]))));
    workspace.add_datatype("atomic", MetaDataType::from_str("atomic",
        RootDataType::Alternate(MetaAlternate::new("atomic",
                                vec![MetaDataTypeName::from_str("int"),
                                     MetaDataTypeName::from_str("real"),
                                     MetaDataTypeName::from_str("dbl"),
                                     MetaDataTypeName::from_str("bool"),
                                     MetaDataTypeName::from_str("char")]))));
    workspace.add_datatype("structure", MetaDataType::from_str("structure", RootDataType::BuiltinAlternate(BuiltAlternates::Structure)));
    workspace.add_datatype("sequence", MetaDataType::from_str("sequence", RootDataType::BuiltinAlternate(BuiltAlternates::Sequence)));
    workspace.add_datatype("v.sequence", MetaDataType::from_str("v.sequence", RootDataType::BuiltinAlternate(BuiltAlternates::VSequence)));
    workspace.add_datatype("general", MetaDataType::from_str("general", RootDataType::BuiltinAlternate(BuiltAlternates::General)));
}

pub fn get_system_datatypes() -> Vec<String> {
    let mut result = Vec::new();
    result.push(String::from("int"));
    result.push(String::from("real"));
    result.push(String::from("dbl"));
    result.push(String::from("bool"));
    result.push(String::from("char"));
    result.push(String::from("string"));
    result.push(String::from("tuple"));
    result.push(String::from("arith"));
    result.push(String::from("atomic"));
    result.push(String::from("structure"));
    result.push(String::from("sequence"));
    result.push(String::from("v.sequence"));
    result.push(String::from("general"));
    result
}
