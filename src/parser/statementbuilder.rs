//  Holds context used in the construction of statements during parsing

use crate::{execution::runtime::executable::Executable, lexical::LineNumber, utility::Set};

use super::tree::Node;

pub struct StatementBuilder<'a> {
    executable: &'a mut Executable,
    statement_line_number: LineNumber,
    statement_starting_offset: usize,
    statement_node_indices: Set<usize>,
}

impl<'a> StatementBuilder<'a> {
    pub fn add_node(&mut self, node: Node) -> usize {
        let node_index = self.executable.add_node(node);
        self.statement_node_indices.add(node_index);
        node_index
    }

    pub fn as_executable(&mut self) -> &mut Executable {
        self.executable
    }

    pub fn as_starting_line_number(&self) -> LineNumber {
        self.statement_line_number
    }

    pub fn begin_statement(line_number: LineNumber, offset: usize, executable: &'a mut Executable) -> StatementBuilder {
        StatementBuilder { statement_line_number: line_number, statement_starting_offset: offset, statement_node_indices: Set::new(), executable: executable }
    }

    pub fn finish_statement(&mut self, ending_offset: usize) {
        self.executable.add_statement(self.statement_line_number,
            self.statement_starting_offset, ending_offset, 
            &self.statement_node_indices);
    }

    pub fn replace_node(&mut self, node_index: usize,  new_node: Node) {
        self.executable.replace_node(node_index,new_node);
    }
}