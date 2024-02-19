//  This module holds the debug print implementations for Value

use std::{fmt, rc::Rc, cell::RefCell};

use crate::utility::convert_escape_sequences;

use super::{Cell, sequence::SequenceInstance, structure::{SelectorInstance, StructureInstance, StructureInstanceMember}, SymbolicReference, Value, ValueEnvelope};

thread_local! {
    static INDENTATION:RefCell<String> = RefCell::new(String::new());
}


impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let my_indentation = INDENTATION.with(|i| i.borrow().clone());
        match self {
            Value::Empty => write!(f, "Value::Empty"),
            Value::Bool(v) => write!(f, "Value::Bool({})", v),
            Value::Int(v) => write!(f, "Value::Int({})", v),
            Value::Real(v) => write!(f, "Value::Real({})", v),
            Value::Double(v) => write!(f, "Value::Double({})", v),
            Value::Char(v) => write!(f, "Value::Char({})", String::from(*v).replace('\n', "\\n").replace('\r', "\\r")),
            Value::Sequence(seq) => {
                if seq.as_recursion_pass().has_not_been_processed() {
                    writeln!(f, "{}Value::Sequence {:?}", my_indentation, seq)
                } else {
                    write!(f, "{}Value::Sequence ...", my_indentation)
                }
            },
            Value::Selector(sel) => write!(f, "Value::Selector({:?})", sel),
            Value::Structure(structure) => {
                if structure.as_recursion_pass().has_not_been_processed() {
                    write!(f, "{}Value::Structure({:?})", my_indentation, structure)
                } else {
                    write!(f, "{}Value::Structure ...", my_indentation)
                }
            },
            Value::Symbol(symbol) => write!(f, "Value::Symbol({:?})", symbol),
            Value::ValueByReference(c) => write!(f, "Value::ValueByReference(${:?})", &*c.cell.borrow().contents.value.borrow()),
            Value::LogicalLink(l) => write!(f, "Value::LogicalLink -> {}", l.as_ref_to_value()),
        }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = &self.contents;
        write!(f, "SC/WC: {}/{} Ptr: {:x} Val: ({:?}", Rc::<ValueEnvelope>::strong_count(v), Rc::<ValueEnvelope>::weak_count(v), std::ptr::addr_of!(*v) as usize, *self.as_ref_to_value())?;
        match &*self.as_ref_to_value() {
            Value::Sequence(_) | Value::Structure(_) => {
                let my_indentation = INDENTATION.with(|i| i.borrow().clone());
                writeln!(f, "{})", my_indentation)?;
            },
            _ => write!(f, ")")?,
        }
        Ok(())
    }
}

impl fmt::Debug for SequenceInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.as_datatype().as_string().as_str() == "string" {
            let mut result = String::new();
            for c in &*self.as_values() {
                result.push(c.borrow().contents.value.borrow().as_char().expect("strings must have Value::Char cells"));
            }
            write!(f, "\"{}\"", convert_escape_sequences(result.as_str()))?;
        } else {
            let base_indentation = INDENTATION.with(|i| i.borrow().clone());
            let my_indentation = base_indentation.clone() + "    ";
            INDENTATION.with(|i| *i.borrow_mut() = my_indentation.clone() + "    ");

            writeln!(f, "DT: {}, LB: {}, Values: [", self.as_datatype(), self.lower_bound())?;
            for v in &*self.as_values() {
                write!(f, "{}SC/WC: {}/{} Ptr: {:x} Val: ({:?}", my_indentation, Rc::<RefCell<Cell>>::strong_count(v), Rc::<RefCell<Cell>>::weak_count(v), std::ptr::addr_of!(*v.borrow()) as usize, *v.borrow())?;
                match &*v.borrow().as_ref_to_value() {
                    Value::Sequence(_) | Value::Structure(_) => writeln!(f, "{})", my_indentation)?,
                    _ => writeln!(f, ")")?,
                }
            }
            writeln!(f, "{}]", my_indentation)?;

            INDENTATION.with(|i| *i.borrow_mut() = base_indentation);
        }

        Ok(())
    } 
}

impl fmt::Debug for SelectorInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}.{{", self.member)?;
       let mut separator = "";
        for structure in &self.structures {
            write!(f, "{}{}", separator, structure)?;
            separator = ", ";
        }
       write!(f, "}}")
    } 
}

impl fmt::Debug for StructureInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    } 
}

impl fmt::Debug for StructureInstanceMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Debug for SymbolicReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{:?}", self.as_string(), self.as_symbol())
    } 
}
