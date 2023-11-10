//  This module holds the definitions of the Invocation structure.

use core::{fmt, panic};
use std::{cell::RefCell, rc::Rc};

use crate::{execution::{functions::FunctionInvocationBlock, value::Value}, 
    symbols::metadata::{FunctionDescription, FunctionImplementation}, 
    parser::tree::Node, 
    lexical::LineNumber, 
    utility::convert_escape_sequences};

use super::{executable::Executable, statement::Statement};



#[derive(Debug,Clone,Copy,PartialEq)]
pub enum ExecutionState {
    NotExecuting,
    Stopped,
    Resumed,
    Executing,
}

pub struct Invocation {
    executable: Rc<Executable>,
    execution_state: RefCell<ExecutionState>,
    fib: Option<Rc<FunctionInvocationBlock>>,
    base_stack_size: usize,
    current_node_index: RefCell<Option<usize>>,
    next_node_index: RefCell<usize>,
    pending_goto_index: RefCell<Option<usize>>,
}

type StatementBlockTag = usize;
    
impl Invocation {
    pub fn as_executable(&self) -> Rc<Executable> {
        self.executable.clone()
    }

    pub fn dump(&self, padding: &str) {
        let mut source = convert_escape_sequences(self.as_executable().as_source());
        if source.len() > 40 {
            source.truncate(40);
            source += "...";
        }
        println!("{}Invocation for: {}", padding, source);
        let node_text:String;
        match *self.current_node_index.borrow() {
            Some(index) => {
                node_text = format!("{} ({})", index, self.executable.get_node(index));
                match self.get_current_statement_source() {
                    Some(s) => source = s,
                    None => source = String::from("None"),
                }
            },
            None => {
                node_text = String::from("None");
                source = String::from("None");
            },
        }
        print!("{}   Current node: {}, Source: {}", padding, node_text, source);
    }

    pub fn get_base_stack_size(&self) -> usize {
        self.base_stack_size
    }

    pub fn get_current_line_number(&self) -> Option<LineNumber> {
        if let Some(node_index) = *self.current_node_index.borrow() {
            if let Some(statement) = self.executable.get_statement_from_node_index(node_index) {
                return Some(statement.as_line_number());
            }
        } 
        None     
    }

    pub fn get_current_statement(&self) -> Option<Rc<Statement>> {
        if let Some(node_index) = *self.current_node_index.borrow() {
            if let Some(statement) = self.executable.get_statement_from_node_index(node_index) {
                return Some(statement);
            }
        } 
        None     
    }

    pub fn get_current_statement_source(&self) -> Option<String> {
        if let Some(node_index) = *self.current_node_index.borrow() {
            if let Some(statement) = self.executable.get_statement_from_node_index(node_index) {
                return Some(statement.as_source());
            }
        } 
        None     
    }
    
    pub fn get_execution_state(&self) -> ExecutionState {
        *self.execution_state.borrow()
    }

    pub fn get_fib(&self) -> Option<Rc<FunctionInvocationBlock>> {
        match &self.fib {
            Some(fib) => Some(fib.clone()),
            None => None,
        }
    }

    fn get_function_return_line_number(&self) -> LineNumber {
        if let Some(fib) = &self.fib {
            let f = &fib.function_description.clone();
            if let FunctionImplementation::User(body) = &f.implementation_class {
                return body.executable.get_function_return_line_number().unwrap();
            }
        }
        panic!("internal error");
    }

    pub fn get_pending_goto_index(&self) -> Option<usize> {
        *self.pending_goto_index.borrow()
    }

    pub fn new(executable: Rc<Executable>, base_stack_size: usize) -> Rc<Invocation> {
        let invocation = Rc::new(Invocation { 
            executable: executable.clone(), 
            execution_state: RefCell::new(ExecutionState::NotExecuting), 
            fib: None,
            base_stack_size: base_stack_size,
            current_node_index: RefCell::new(None),
            next_node_index: RefCell::new(0),
            pending_goto_index: RefCell::new(None), });
        invocation.reset();
        invocation
    }

    pub fn new_with_fib(f: &Rc<FunctionDescription>, base_stack_size: usize) ->  Rc<Invocation> {
        match f.implementation_class {
            crate::symbols::metadata::FunctionImplementation::System(_) => panic!("internal error"),
            crate::symbols::metadata::FunctionImplementation::User(ref body) => {
                let invocation = Rc::new(Invocation { 
                    executable: body.executable.clone(), 
                    execution_state: RefCell::new(ExecutionState::NotExecuting), 
                    fib: Some(Rc::new(FunctionInvocationBlock::new(f.clone()))),
                    base_stack_size: base_stack_size,
                    current_node_index: RefCell::new(None),
                    next_node_index: RefCell::new(0),
                    pending_goto_index: RefCell::new(None), });
                invocation.reset();
                invocation
                    },
        }
    }

    pub fn next_node(&self) -> Option<(usize, Rc<Node>)> {
        let new_current_node_index = *self.next_node_index.borrow();
        if let Some(last_node_index) = self.executable.get_last_node_index() {
            if new_current_node_index <= last_node_index {
                *self.current_node_index.borrow_mut() = Some(new_current_node_index);
                *self.next_node_index.borrow_mut() = new_current_node_index + 1;
                Some((new_current_node_index, self.executable.get_node(new_current_node_index)))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn repeat(&self) {
        if let Some(node_index) = *self.current_node_index.borrow() {
            *self.next_node_index.borrow_mut() = node_index;
        } else {
            panic!("internal error");
        }
    }

    pub fn reset(&self) {
        *self.execution_state.borrow_mut() = ExecutionState::NotExecuting;
        *self.current_node_index.borrow_mut() = None;
        *self.next_node_index.borrow_mut() = 0;
    }

    pub fn set_execution_state(&self, new_state: ExecutionState) {
        *self.execution_state.borrow_mut() = new_state;
    }

    pub fn set_next_node(&self, next_node_index: usize) {
        *self.next_node_index.borrow_mut() = next_node_index;
    }

    pub fn set_next_node_by_line_number(&self, line_number: LineNumber) -> Result<Value,String> {

        if let Some(_) = &self.fib {

            //  Look for a statement with the designated line number. If found, its first node is the next node to execute. If not,
            //  then we'll choose the FunctionReturn node (if there is one)

            if let Some(statement) = self.executable.get_statement_by_line_number(line_number) {
                self.set_next_node(statement.as_first_node_index());
                return Ok(Value::Int(line_number as i32));
            } else if let Some(line_number) = self.executable.get_function_return_line_number() {
                if let Some(statement) = self.executable.get_statement_by_line_number(line_number) {
                    self.set_next_node(statement.as_first_node_index());
                    return Ok(Value::Int(line_number as i32));
                }
            }
            panic!("internal error");

        } else {
            Err(format!("--> can only be used inside functions or to resume a stopped function"))
        }
    }

    pub fn set_pending_goto_index(&self, next_node_index: Option<usize>) {
        *self.pending_goto_index.borrow_mut() = next_node_index;
    }
}

impl fmt::Debug for Invocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "State: {}, Next node: {}\nExecutable:", *self.execution_state.borrow(), *self.next_node_index.borrow())?;
        writeln!(f, "{:?}", self.executable)?;
        if let Some(ref fib) = self.fib {
            writeln!(f, "FIB: {:?}", fib)?;
        }
        Ok(())
    }
}

impl fmt::Display for ExecutionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionState::NotExecuting => write!(f, "NotExecuting"),
            ExecutionState::Stopped => write!(f, "Stopped"),
            ExecutionState::Resumed => write!(f, "Resumed"),
            ExecutionState::Executing => write!(f, "Executing"),
        }
    }
}