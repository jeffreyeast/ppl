//  This module holds the definitions of the Executable structure.

use core::fmt;
use std::{collections::HashMap, rc::Rc};

use crate::{parser::tree::{Node, PrettyPrint}, lexical::LineNumber, utility::{convert_escape_sequences, Set}};

use super::statement::Statement;


pub struct Executable {
    source: String,                                     //  The source string that was parsed
    statements: Vec<Rc<Statement>>,                     //  The parsed statements
    node_list: Vec<Rc<Node>>,                           //  The parse tree
    statements_by_node_index_table: HashMap<usize, Rc<Statement>>,
    function_return_line_number: Option<LineNumber>,    //  The origin-0 index of the function return node
}


impl Executable {
    pub fn add_node(&mut self, node: Node) -> usize {
        self.node_list.push(Rc::new(node));
        let node_index = (self.node_list.len() - 1) as usize;
        node_index
    }

    pub fn add_statement(&mut self, line_number: LineNumber, starting_offset: usize, ending_offset: usize, node_indices: &Set<usize>) -> usize {
        let statement = Statement::new(line_number, &self.source[starting_offset..ending_offset], node_indices);
        self.statements.push(Rc::new(statement));
        self.statements.len() - 1
    }

    pub fn as_source(&self) -> &str {
        self.source.as_str()
    }

    pub fn commit(&mut self) {
        // Build the node index -> statement table

        for s in &self.statements {
            for i in s.as_node_indices().iter() {
                self.statements_by_node_index_table.insert(i, s.clone());
            }
        }
    }

    pub fn get_function_return_line_number(&self) -> Option<LineNumber> {
        return self.function_return_line_number
    }

    pub fn get_last_node_index(&self) -> Option<usize> {
        match self.node_list.len() {
            0 => None,
            _ => Some(self.node_list.len() - 1),
        }
        
    }

    pub fn get_last_statement(&self) -> Option<&Rc<Statement>> {
        self.statements.last()
    }

    pub fn get_statement_from_node_index(&self, node_index: usize) -> Option<Rc<Statement>> {
        match self.statements_by_node_index_table.get(&node_index) {
            Some(s) => Some(s.clone()),
            None => None,
        }
    }

    pub fn get_node(&self, index: usize) -> Rc<Node> {
        self.node_list[index].clone()
    }

    pub fn get_next_node_index(&self) -> usize {
        self.node_list.len()
    }

    pub fn get_statement(&self, statement_index: usize) -> Option<Rc<Statement>> {
        if statement_index > 0 && statement_index <= self.statements.len() {
            Some(self.statements[statement_index-1].clone())
        } else {
            None
        }
    }

    pub fn get_statement_count(&self) -> usize {
        self.statements.len()
    }

    pub fn get_statement_by_line_number(&self, line_number: LineNumber) -> Option<Rc<Statement>> {
        self.get_statement_index_by_line_number(line_number).and_then(|statement_index| self.get_statement(statement_index))
    }

    pub fn get_statement_index_by_line_number(&self, line_number: LineNumber) -> Option<usize> {
        let mut low = 1;
        let mut high = self.statements.len();

        while low <= high {
            let target = (low + high) / 2;
            if self.statements[target-1].as_line_number() == line_number {
                return Some(target);
            } else if self.statements[target-1].as_line_number() > line_number {
                high = target - 1;
            } else {
                low = target + 1;
            }
        }

        None
    }

    pub fn is_first_executable_node(&self, statement: &Rc<Statement>, node_index: usize) -> bool {
        if node_index == statement.as_first_node_index() {
            return true;
        }
        // if let Node::StatementLabel(_) = self.get_node(statement.as_first_node_index()).as_ref() {
        //     match statement.as_second_node_index() {
        //         Some(second_node_index) => return node_index == second_node_index,
        //         None => {},
        //     }
        // }
        return false;
    }

    pub fn is_last_executable_node(&self, statement: &Rc<Statement>, node_index: usize) -> bool {
        node_index == statement.as_last_node_index()
    }

    pub fn is_line_number_valid(&self, line_number: LineNumber) -> bool {
        self.get_statement_index_by_line_number(line_number).is_some()
    }

    pub fn new(source: &str) -> Executable {
        Executable { 
            source: String::from(source), 
            statements: Vec::new(), 
            node_list: Vec::new(), 
            statements_by_node_index_table: HashMap::new(),
            function_return_line_number: None }
    }

    pub fn process_on_each_statement(&self, processor: fn(statement: Rc<Statement>)) {
        for statement in &self.statements {
            processor(statement.clone());
        }
    }

    pub fn replace_node(&mut self, position: usize, node: Node) {
        self.node_list[position] = Rc::new(node);
    }

    pub fn set_function_return_line_number(&mut self, function_return_line_number: LineNumber) {
        self.function_return_line_number = Some(function_return_line_number);
    }
}

impl fmt::Display for Executable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_source())
    }
}


impl fmt::Debug for Executable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let frln = match self.function_return_line_number {
            Some(i) => format!("{}", i),
            None => String::from("None"),
        };
        writeln!(f, "FRLN: {}, Source: \"{}\"", frln, convert_escape_sequences(self.source.trim_end()))?;
        for i in 0..self.node_list.len() {
            PrettyPrint::fmt(self.get_node(i).as_ref(), f, "    ", i, self)?;
        }
        Ok(())
    }
}
