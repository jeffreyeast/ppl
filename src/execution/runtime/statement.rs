//  This module holds the definitions of the Statement and StatementBlockIterator structures.

use core::fmt;
use std::cell::RefCell;

use crate::{lexical::LineNumber, utility::{convert_escape_sequences, Set, SetIterator}};




pub struct Statement {
    source: String,                  //  The slice of the Executable's source represented by this Statement
    line_number: LineNumber,
    node_indices: Set<usize>,               //  The origin-0 index numbers of the nodes corresponding to this statement
    trace: RefCell<bool>,            //  TRUE if the execution of the statement should be traced
    stop: RefCell<bool>,             //  TRUE if a breakpoint is set on the statement
}


impl Statement {
    pub fn as_first_node_index(&self) -> usize {
        self.node_indices.as_members().first().unwrap().get_first()
    }

    pub fn as_node_indices(&self) -> &Set<usize> {
        &self.node_indices
    }

    pub fn as_line_number(&self) -> LineNumber {
        self.line_number
    }

    pub fn as_second_node_index(&self) -> Option<usize> {
        let mut iter = self.node_indices.iter();
        iter.next();
        iter.next()
    }

    pub fn as_source(&self) -> String {
        self.source.clone()
    }

    pub fn is_stop_set(&self) -> bool {
        *self.stop.borrow()
    }

    pub fn is_trace_set(&self) -> bool {
        *self.trace.borrow()
    }

    pub fn iter(&self) -> SetIterator<'_,usize> {
        self.node_indices.iter()
    }

    pub fn new(line_number: u32, source: &str, node_indices: &Set<usize>) -> Statement {
        Statement { 
            source: String::from(source), 
            line_number: line_number,  
            node_indices: node_indices.clone(),
            trace: RefCell::new(false), 
            stop: RefCell::new(false), }
    }

    pub fn set_stop(&self, value: bool) {
        *self.stop.borrow_mut() = value;
    }

    pub fn set_trace(&self, value: bool) {
        *self.trace.borrow_mut() = value;
    }
}


impl fmt::Debug for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}) [{}]: \"{}\"", self.line_number, self.node_indices, convert_escape_sequences(self.source.trim_end()))
    }
}
