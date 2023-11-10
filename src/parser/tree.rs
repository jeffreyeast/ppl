//  This module holds the definition of the parse tree

use std::fmt;
use crate::{execution::{value::Value, runtime::executable::Executable}, symbols::{name::Name, metadata::FunctionDescription}, utility::convert_escape_sequences};


pub trait PrettyPrint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, padding: &str, node_number: usize, executable: &Executable) -> fmt::Result;
}

pub enum Node {
    Definition(DefinitionNode),
    FunctionReturn,
    IdentifierByReference(ReferenceNode),
    IdentifierByValue(ReferenceNode),
    Index(IndexNode),
    Noop,
    Operation(OperationNode),
    StatementEnd(usize),            // Node index of expression tree root
    StatementLabel(LabelNode),
    Value(Value),
}


impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Definition(d) => write!(f, "{}", d),
            Node::FunctionReturn => write!(f, "return"),
            Node::Index(i) => write!(f, "{}", i),
            Node::Noop => write!(f, "Noop()"),
            Node::Operation(op) => write!(f, "{}", op),
            Node::IdentifierByValue(r) => write!(f, "{}", r),
            Node::IdentifierByReference(r) => write!(f, "${}", r),
            Node::StatementEnd(r) => write!(f, "<end>{}", r),
            Node::StatementLabel(s) => write!(f, "{}:\t{}", s.name, s.statement_index),
            Node::Value(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Definition(def) => write!(f, "Node::Def({:?})", def),
            Node::FunctionReturn => write!(f, "Node::FunctionReturn"),
            Node::Index(idx) => write!(f, "Node::Index({:?})", idx),
            Node::Noop => write!(f, "Node::Noop"),
            Node::Operation(op) => write!(f, "Node::Op({:?})", op),
            Node::IdentifierByValue(id) =>  write!(f, "Node::Id({:?})", id),
            Node::IdentifierByReference(idr) => write!(f, "Node::IdByRef({:?})", idr),
            Node::StatementEnd(r) => write!(f, "Node::StatementEnd({})", r),
            Node::StatementLabel(s) => write!(f, "Node::StatementLabel({})", s),
            Node::Value(v) => write!(f, "Node::Value({:?})", v),
        }
    }
}

impl PrettyPrint for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, padding: &str, node_number: usize, _executable: &Executable) -> fmt::Result {
        match self {
            Node::Definition(d) => {
                writeln!(f, "{}({}) Node::Definition: {}", padding, node_number, d.name)
            },
            Node::FunctionReturn => writeln!(f, "{}({}) Node::FunctionReturn", padding, node_number),
            Node::Index(_) =>  {
                writeln!(f, "{}({})Node::Index", padding, node_number)
            },
            Node::Noop => writeln!(f, "{}({}) Node::Noop", padding, node_number),
            Node::Operation(o) =>  {
                writeln!(f, "{}({}) Node::Operation ({})", padding, node_number, o.name)
            },
            Node::IdentifierByValue(i) => writeln!(f, "{}({}) Node::IdentifierByValue ({})", padding, node_number, i.as_string()),
            Node::IdentifierByReference(i) => writeln!(f, "{}({}) Node::IdentifierByReference (${})", padding, node_number, i.as_string()),
            Node::StatementEnd(r) => writeln!(f, "{}({}) Node::StatementEnd({})", padding, node_number, r),
            Node::StatementLabel(l) => {
                writeln!(f, "{}({}) Node::StatementLabel ({})", padding, node_number, l)
            },
            Node::Value(v) => writeln!(f, "{}({}) Node::Value ({})", padding, node_number, convert_escape_sequences(v.as_string().as_str())),
        }
    }
}


#[derive(Debug)]
pub enum DefinitionType {
    Function(FunctionDescription),
    Structure(StructureDefinition),
    Sequence(SequenceDefinition),
    Alternate(Vec<usize>),      // Node indices of the alternatives
}

pub struct DefinitionNode {
    name: Name,
    class: DefinitionType,
}

impl DefinitionNode {
    pub fn as_string(&self) -> String {
        self.name.as_string()
    }

    pub fn from_string(name: &String, class: DefinitionType) -> DefinitionNode {
        DefinitionNode { name: Name::from_string(name), class: class }
    }

    pub fn get(&self) -> &DefinitionType {
        &self.class
    }
}

impl fmt::Display for DefinitionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[...]", self.name.as_str())        
    }
}

impl fmt::Debug for DefinitionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.name)?;
        match &self.class {
            DefinitionType::Function(func) => writeln!(f, "Function: {:?}", func),
            DefinitionType::Structure(structure) => writeln!(f, "Structure: {:?}", structure),
            DefinitionType::Sequence(seq) => writeln!(f, "Sequence: {:?}", seq),
            DefinitionType::Alternate(alt) => {
                write!(f, "Alternate: ")?;
                let mut separator = "";
                for alternates in alt {
                    write!(f, "{}{}", separator, alternates)?;
                    separator = " ! ";
                }
                writeln!(f, "") 
            },
        }
    }
}

impl PrettyPrint for DefinitionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, padding: &str, _node_number: usize, _executable: &Executable) -> fmt::Result {
        writeln!(f, "{}{:?}", padding, self)
    }
}


pub struct IndexNode {
    pub value_position: usize,
    pub index_position: usize,
}

impl fmt::Display for IndexNode {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}[ {} ]", self.value_position, self.index_position)
    }
}

impl fmt::Debug for IndexNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IP:{}, Val:{}", self.index_position, self.value_position)
    }
}

impl PrettyPrint for IndexNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, padding: &str, _node_number: usize, executable: &Executable) -> fmt::Result {
        let child_padding = format!("{}    ", padding);
        writeln!(f, "{}Value:", padding)?;
        executable.get_node(self.value_position).fmt(f, &child_padding, self.value_position, executable)?;
        writeln!(f, "{}Index:", padding)?;
        executable.get_node(self.index_position).fmt(f, &child_padding, self.index_position, executable)
    }
}

pub struct LabelNode {
    pub name: String,
    pub statement_index: usize,
}

impl fmt::Display for LabelNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} :\t{}", self.name, self.statement_index)
    }
}

impl fmt::Debug for LabelNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} :\t{}", self.name, self.statement_index)
    }
}




pub struct OperationNode {
    name: Name,
    actual_arguments: Vec<usize>,     //  Indices into the NodeList
}

impl OperationNode {
    
    pub fn from_str(name: &str, args: Vec<usize>) -> OperationNode {
        OperationNode { name: Name::from_str(name), actual_arguments: args } 
    }
    
    pub fn from_string(name: &String, args: Vec<usize>) -> OperationNode {
        OperationNode { name: Name::from_string(name), actual_arguments: args } 
    }

    pub fn get_actual_argument_list(&self) -> &Vec<usize> {
        &self.actual_arguments
    }

    pub fn get_name(&self) -> String {
        self.name.as_string()
    }
}

impl fmt::Display for OperationNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(...)", self.name.as_str())
    }
}

impl fmt::Debug for OperationNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.name)?;
        let mut separator = "";
        for arg in &self.actual_arguments {
            write!(f, "{}{}", separator, arg)?;
            separator = ", ";
        }
        write!(f, ")")
    }
}

impl PrettyPrint for OperationNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, padding: &str, _node_number: usize, executable: &Executable) -> fmt::Result {
        let child_padding = format!("{}    ", padding);
        for i in 0..self.actual_arguments.len() {
            writeln!(f, "{}[{}]", padding, i)?;
            executable.get_node(self.actual_arguments[i]).fmt(f, &child_padding, self.actual_arguments[i], executable)?;
        }
        Ok(())
    }
}


pub struct ReferenceNode {
    name: Name,
}

impl  ReferenceNode {
    pub fn as_name(&self) -> Name {
        self.name.clone()
    }

    pub fn as_string(&self) -> String{
        self.name.as_string()
    }
   
    pub fn from_str(name: &str) -> ReferenceNode {
        ReferenceNode { name: Name::from_str(name) } 
    }
   
    pub fn from_string(name: &String) -> ReferenceNode {
        ReferenceNode { name: Name::from_string(name) } 
    }
}

impl fmt::Display for ReferenceNode {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "{}", self.name.as_string())?;
        Ok(())
    }
}

impl fmt::Debug for ReferenceNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}", self.name)
    }
}



pub struct SequenceDefinition {
    pub root_datatype_name: Name,
    pub lower_bound: i32,
    pub upper_bound: Option<i32>,
}

impl SequenceDefinition {
    pub fn from_string(name: &String, lower_bound: i32, upper_bound: Option<i32>) -> SequenceDefinition {
        SequenceDefinition { root_datatype_name: Name::from_string(name), lower_bound: lower_bound, upper_bound: upper_bound }
    }
}

impl fmt::Debug for SequenceDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let upper_bound = match self.upper_bound {
            Some(u) => format!("{}", u),
            None => String::from("None"),
        };
        write!(f, "Root: {}, Low: {}, Upper: {}", self.root_datatype_name, self.lower_bound, upper_bound)
    }
}


#[derive(Debug)]
pub struct StructureDefinition {
    pub members: Vec<StructureMemberDescription>,
}

#[derive(Debug)]
pub struct StructureMemberDescription {
    pub name: Name,
    pub datatype: Name,
}

