//  This module holds the context block for the overall interpreter

//  The invocation_stack holds one entry for each function executing, plus [optionally] one for the current
//  immediate mode command.  In the case where the user issues an immediate mode command to resume a stopped
//  function, we'll pop the immediate mode command off and apply the goto to the most recent (stopped) function. Thus
//  we need a way for execution to not end the immediate mode invocation.

use std::arch::asm;
use std::cell::{Ref, RefCell, RefMut};
use std::io::Cursor;
use std::ops::Deref;
use std::rc::Rc;
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crate::execution::functions::FunctionInvocationBlock;
use crate::execution::runtime::{invocation::Invocation, executable::Executable};
use crate::execution::sentinal::ExecutionSentinal;
use crate::execution::system_functions;
use crate::execution::value::Value;
use crate::execution::value::sequence::SequenceInstance;
use crate::graphics::GraphicsContext;
use crate::stack_ptr;
use crate::symbols::datatype::RootDataType;
use crate::symbols::help::Help;
use crate::symbols::metadata::{VariableDescription, self};
use crate::symbols::name::Name;
use crate::symbols::{metadata::{MetaDataType, FunctionDescription, SelectorDescription}, SymbolTable};

use self::debug::DebugOption;
use self::optional_features::Feature;
use self::options::Options;

pub mod debug;
pub mod io;
pub mod optional_features;
pub mod options;



 pub struct WorkSpace {
    datatype_symbol_table: RefCell<SymbolTable<MetaDataType>>,
    system_function_symbol_table: RefCell<SymbolTable<RefCell<Vec<Rc<FunctionDescription>>>>>,
    user_function_symbol_table: RefCell<SymbolTable<FunctionDescription>>,
    selector_symbol_table: RefCell<SymbolTable<SelectorDescription>>,
    variable_symbol_table: RefCell<SymbolTable<VariableDescription>>,
    invocation_stack: RefCell<Vec<Rc<Invocation>>>,
    value_stack: RefCell<Vec<Value>>,
    last_statement_value: RefCell<Option<Value>>,
    stack_start: RefCell<usize>,
    format_parser: crate::execution::value::format::format_parser::Parser,
    floating_point_parser: crate::execution::value::format::floating_point::Parser,
    alternate_print_destination: RefCell<Option<Cursor<Vec<u8>>>>,
    graphics_context: Arc<GraphicsContext>,
    pub debug_options: RefCell<Options<DebugOption>>,
    pub features: RefCell<Options<Feature>>,
    pub execution_sentinal: RefCell<ExecutionSentinal>,
}

impl Help for Rc<FunctionDescription> {
    fn help_text(&self, workspace: &WorkSpace) -> Option<String> {
        self.deref().help_text(workspace)
    }

    fn help_text_len(&self, workspace: &WorkSpace) -> usize {
        self.deref().help_text_len(workspace)
    }

    fn pretty_print(&self) -> String {
        self.deref().pretty_print()
    }

    fn show_help(&self, name: &str, workspace: &WorkSpace) -> Result<Value,String> {
        self.deref().show_help(name, workspace)
    }
}

impl Help for RefCell<Vec<Rc<FunctionDescription>>> {
    fn help_text(&self, workspace: &WorkSpace) -> Option<String> {
        let mut help_text = String::new();
        let mut separator = "";
        for func in self.borrow().iter() {
            help_text += format!("{}{}", separator, func.help_text(workspace).or(Some(String::from(""))).unwrap()).as_str();
            separator = "\n   - ";
        }
        Some(help_text)
    }

    fn help_text_len(&self, workspace: &WorkSpace) -> usize {
        match self.help_text(workspace) {
            Some(help_text) => help_text.len(),
            None => 0,
        }
    }

    fn pretty_print(&self) -> String {
        let mut help_text = String::new();
        let mut separator = "";
        for func in self.borrow().iter() {
            help_text += format!("{}{}", separator, func.pretty_print()).as_str();
            separator = "\n   - ";
        }
        help_text
    }

    fn show_help(&self, name: &str, workspace: &WorkSpace) -> Result<Value,String> {
        let mut help_text = String::new();
        for func in self.borrow().iter() {
            help_text += format!("{} - {}\n", name, func.help_text(workspace).or(Some(String::from(""))).unwrap()).as_str();
        }
        Ok(SequenceInstance::construct_string_sequence(&help_text))
    }
}

#[derive(Debug,Clone)]
pub enum GeneralSymbol {
    Datatype(Rc<MetaDataType>),
    Function(Rc<FunctionDescription>),
    Selector(Rc<SelectorDescription>),
    Variable(Rc<VariableDescription>),
    Unresolved(Name),
}

impl fmt::Display for GeneralSymbol  {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         match self {
            GeneralSymbol::Datatype(d) => write!(f, "{}", d),
            GeneralSymbol::Function(func) => write!(f, "{}", func),
            GeneralSymbol::Selector(s) => write!(f, "{}", s),
            GeneralSymbol::Variable(v) => write!(f, "{}", v),
            GeneralSymbol::Unresolved(u) => write!(f, "{}", u),
        }
     }       
}

impl crate::symbols::metadata::Metadata for  GeneralSymbol {
    fn as_definition(&self) -> String {
        match self {
            GeneralSymbol::Datatype(d) => d.as_definition(),
            GeneralSymbol::Function(f) => f.as_definition(),
            GeneralSymbol::Selector(s) => s.as_definition(),
            GeneralSymbol::Variable(v) => v.as_definition(),
            GeneralSymbol::Unresolved(name) => format!("{} is not defined", name),
        }
    }
}



impl WorkSpace {
    pub fn dump_symbol_table(&self)  -> Value {
        // dbg!(self.datatype_symbol_table.borrow());
        // dbg!(self.system_function_symbol_table.borrow());
        // dbg!(self.user_function_symbol_table.borrow());
        // dbg!(self.selector_symbol_table.borrow());
        // dbg!(self.variable_symbol_table.borrow());

        let mut result = String::from("Datatypes:\n");
        result += self.datatype_symbol_table.borrow().dump().as_str();

        result += "Variables:\n";
        result += self.variable_symbol_table.borrow().dump().as_str();

        result += "Functions:\n";
        result += self.user_function_symbol_table.borrow().dump().as_str();

        SequenceInstance::construct_string_sequence(result.as_str())
    }
    
    pub fn new() -> WorkSpace {
        let workspace = WorkSpace {
            datatype_symbol_table: RefCell::new(SymbolTable::new()),
            system_function_symbol_table: RefCell::new(SymbolTable::new()),
            user_function_symbol_table: RefCell::new(SymbolTable::new()),
            selector_symbol_table: RefCell::new(SymbolTable::new()),
            variable_symbol_table: RefCell::new(SymbolTable::new()),
            invocation_stack: RefCell::new(Vec::new()),
            value_stack: RefCell::new(Vec::new()),
            last_statement_value: RefCell::new(None),
            stack_start: RefCell::new(0),
            format_parser: crate::execution::value::format::format_parser::Parser::new(),
            floating_point_parser: crate::execution::value::format::floating_point::Parser::new(),
            alternate_print_destination: RefCell::new(None),
            graphics_context: GraphicsContext::new(),
            debug_options: RefCell::new(DebugOption::new()),
            features: RefCell::new(Feature::new()),
            execution_sentinal: RefCell::new(ExecutionSentinal::new()) };
        workspace.init();
        workspace
    }


    pub fn add_datatype (&self, name: &str, datatype: MetaDataType) {
        self.datatype_symbol_table.borrow_mut().add(Name::from_str(name), datatype)
    }

    pub fn add_selector (&self, name: &str, selector: SelectorDescription) {
        self.selector_symbol_table.borrow_mut().add(Name::from_str(name), selector)
    }

    pub fn add_system_function (&self, name: &str, f: FunctionDescription) {
        let opt_homonym_list = self.system_function_symbol_table.borrow_mut().try_get(name);
        match opt_homonym_list {
            Some(homonym_list) => {
                homonym_list.borrow_mut().push(Rc::new(f));
            },
            None => {
                self.system_function_symbol_table.borrow_mut().add(Name::from_str(name), RefCell::new(Vec::new()));
                self.add_system_function(name, f);
            },
        }
    }

    pub fn add_system_function_by_reference (&self, name: &str, f: Rc<FunctionDescription>) {
        let opt_homonym_list = self.system_function_symbol_table.borrow_mut().try_get(name);
        match opt_homonym_list {
            Some(homonym_list) => {
                homonym_list.borrow_mut().push(f.clone());
            },
            None => {
                self.system_function_symbol_table.borrow_mut().add(Name::from_str(name), RefCell::new(Vec::new()));
                self.add_system_function_by_reference(name, f);
            },
        }
    }

    pub fn add_user_function (&self, name: &str, f: &FunctionDescription) {
        self.user_function_symbol_table.borrow_mut().add(Name::from_str(name), f.clone())
    }

    pub fn add_user_function_by_reference (&self, name: &str, f: Rc<FunctionDescription>) {
        self.user_function_symbol_table.borrow_mut().add_by_reference(Name::from_str(name), f)
    }

    pub fn add_variable (&self, name: &str, v: VariableDescription) {
        self.variable_symbol_table.borrow_mut().add(Name::from_str(name), v)
    }

    pub fn alternate_print_destination_as_string(&self) -> String {
        match &*self.alternate_print_destination.borrow() {
            Some(c) => String::from(std::str::from_utf8(&c.get_ref()[..]).expect("convert to String failed")),
            None => panic!("internal error"),
        }
    }

    pub fn contains_any(&self, name: &str) -> bool {
        self.datatype_symbol_table.borrow().contains_any(name) ||
        self.system_function_symbol_table.borrow().contains_any(name) ||
        self.user_function_symbol_table.borrow().contains_any(name) ||
        self.selector_symbol_table.borrow().contains_any(name) ||
        self.selector_symbol_table.borrow().contains_any(name)
    }

    pub fn current_fib(&self) -> Option<Rc<FunctionInvocationBlock>> {
        for invocation in self.invocation_stack.borrow().iter().rev() {
            if invocation.get_fib().is_some() {
                return invocation.get_fib();
            }
        }
        None
    }

    pub fn current_invocation(&self) -> Option<Rc<Invocation>> {
        match self.invocation_stack.borrow().last() {
            Some(i) => Some(i.clone()),
            None => None,
        }
    }

    pub fn disable_alternate_print_destination(&self) {
        *self.alternate_print_destination.borrow_mut() = None;
    }

    pub fn dump_invocation_stack(&self) {
        println!("Invocation Stack:");
        for i in (0..self.invocation_stack.borrow().len()).rev() {
            print!("  {} ", i);
            self.invocation_stack.borrow()[i].dump("    ");
        }
    }

    pub fn dump_value_stack(&self) {
        if self.value_stack.borrow().len() == 0 {
            println!("Value stack: <empty>");
        } else {
            println!("Value stack:");
            for i in (0..self.value_stack.borrow().len()).rev() {
                 println!("[{}]: {:?}", i, &self.value_stack.borrow()[i]);
            }
        }
    }

    pub fn enable_alternate_print_destination(&self) {
        *self.alternate_print_destination.borrow_mut() = Some(Cursor::new(Vec::new()));
    }

    pub fn end_invocation(&self) {
        self.invocation_stack.borrow_mut().pop();
    }

    pub fn get_alternate_print_destinatin(&self) -> RefMut<'_,Option<Cursor<Vec<u8>>>> {
        self.alternate_print_destination.borrow_mut()
    }

    pub fn get_execution_sentinal(&self) -> Ref<'_,ExecutionSentinal> {
        self.execution_sentinal.borrow()
    }

    pub fn get_execution_sentinal_mut(&self) -> RefMut<'_,ExecutionSentinal> {
        self.execution_sentinal.borrow_mut()
    }

    pub fn get_execution_sentinal_as_atomicbool(&self) -> Arc<AtomicBool> {
        self.execution_sentinal.borrow().as_atomicbool().clone()
    }

    pub fn get_floating_point_parser(&self) -> &crate::execution::value::format::floating_point::Parser {
        &self.floating_point_parser
    }

    pub fn get_format_parser(&self) -> &crate::execution::value::format::format_parser::Parser {
        &self.format_parser
    }

    pub fn get_graphics_context(&self) -> Arc<GraphicsContext> {
        self.graphics_context.clone()
    }

    pub fn get_last_statement_value(&self) -> Option<Value> {
        match &*self.last_statement_value.borrow() {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    pub fn get_stack_size(&self) -> usize {
        *self.stack_start.borrow() - stack_ptr!()
    }

    pub fn get_value_stack_size(&self) -> usize {
        self.value_stack.borrow().len()
    }

    pub fn help_all(&self) -> Result<Value,String> {
        self.system_function_symbol_table.borrow().help_all(self)
    }
    
    pub fn help_one(&self, name: &str) -> Result<Value,String> {
        let mut result = String::new();
        let symbols = self.try_get_functions(name);
        for symbol in symbols {
            result += symbol.show_help(name, self)?.as_string().as_str();
            result += "\n";
        }
        Ok(SequenceInstance::construct_string_sequence(&result))
    }
    
    fn init(&self) {
        self.features.borrow_mut().set(optional_features::Feature::Ardmode);
        self.features.borrow_mut().set(optional_features::Feature::LineNames);
        self.features.borrow_mut().set(optional_features::Feature::StringEscapes);
        system_functions::init(&self);
        metadata::init(&self);
    }
    
    pub fn is_in_graphics_mode(&self) -> bool {
        self.graphics_context.is_in_graphics_mode()
    }

    pub fn pop_value(&self) -> Value {
        self.value_stack.borrow_mut().pop().unwrap()
    }

    pub fn push_value(&self, value: &Value) {
        self.value_stack.borrow_mut().push(value.clone())
    }

    pub fn remove(&self, name: &str) {
        self.reset_function_state();
        match self.try_get_any(name) {
            GeneralSymbol::Datatype(_) => self.datatype_symbol_table.borrow_mut().remove(name),
            GeneralSymbol::Function(_) => self.user_function_symbol_table.borrow_mut().remove(name),
            GeneralSymbol::Selector(_) => self.selector_symbol_table.borrow_mut().remove(name),
            GeneralSymbol::Variable(_) => self.variable_symbol_table.borrow_mut().remove(name),
            GeneralSymbol::Unresolved(_) => {},
        }
    }

    pub fn remove_all(&self) {
        self.reset_function_state();
        self.datatype_symbol_table.borrow_mut().clear();
        self.system_function_symbol_table.borrow_mut().clear();
        self.user_function_symbol_table.borrow_mut().clear();
        self.selector_symbol_table.borrow_mut().clear();
        self.variable_symbol_table.borrow_mut().clear();
        self.init();
    }

    pub fn reset(&self) {
        self.invocation_stack.borrow_mut().clear();
        self.value_stack.borrow_mut().clear();
        *self.last_statement_value.borrow_mut() = None;
    }

    pub fn reset_function_state(&self) {
        self.invocation_stack.borrow_mut().clear();
    }

    pub fn resolve_datatype(&self, datatype_name: &String) -> Result<RootDataType,String> {
        let opt_symbol = self.try_get_datatype(datatype_name.as_str());
        if opt_symbol.is_some() {
            return Ok(opt_symbol.unwrap().root_data_type().clone());
        }

        Err(format!("{} not found", datatype_name))
    }

    pub fn set_last_statement_value(&self, value: &Value) {
        *self.last_statement_value.borrow_mut() = Some(value.clone());
    }

    pub fn start_immediate_mode(&self, executable: &Rc<Executable>) {
        *self.stack_start.borrow_mut() = stack_ptr!();
        let invocation = Invocation::new(executable.clone(), self.get_value_stack_size());
        self.invocation_stack.borrow_mut().push(invocation.clone());
    }

    pub fn start_user_function(&self, f: &Rc<FunctionDescription>) {
        let invocation = Invocation::new_with_fib(f, self.get_value_stack_size());
        self.invocation_stack.borrow_mut().push(invocation.clone());
    }
    
    pub fn try_get_any(&self, name: &str) -> GeneralSymbol {
        if let Some(fib) = &self.current_fib() {
            if let Some(v) = fib.variable_symbol_table.borrow().try_get(name) {
                return GeneralSymbol::Variable(v.clone());
            }
        }

        if let Some(v) = self.variable_symbol_table.borrow().try_get(name) {
            return GeneralSymbol::Variable(v.clone());
        }

        if let Some(d) = self.datatype_symbol_table.borrow().try_get(name) {
            return GeneralSymbol::Datatype(d.clone());
        }

        if let Some(s) = self.selector_symbol_table.borrow().try_get(name) {
            return GeneralSymbol::Selector(s.clone());
        }

        if let Some(f) = self.user_function_symbol_table.borrow().try_get(name) {
            return GeneralSymbol::Function(f.clone());
        }

        if let Some(f) = self.system_function_symbol_table.borrow().try_get(name) {
            for func in f.borrow().iter() {
                return GeneralSymbol::Function(func.clone());
            }
        }

        GeneralSymbol::Unresolved(Name::from_str(name))
    }

    pub fn try_get_datatype(&self, name: &str) -> Option<Rc<MetaDataType>> {
        self.datatype_symbol_table.borrow().try_get(name)
    }

    pub fn try_get_function(& self, name: &str) -> Option<Rc<FunctionDescription>> {
        match self.user_function_symbol_table.borrow().try_get(name) {
            Some(func) => Some(func),
            None => match self.system_function_symbol_table.borrow().try_get(name) {
                Some(v) => {
                    for func in v.borrow().iter() {
                        return Some(func.clone());
                    }
                    None
                },
                None => None,
            }
        }
    }

    pub fn try_get_functions(&self, name: &str) -> Vec<GeneralSymbol> {
        let mut result = Vec::new();
        if let Some(f) = self.user_function_symbol_table.borrow().try_get(name) {
            result.push(GeneralSymbol::Function(f));
        }

        if let Some(v) = self.system_function_symbol_table.borrow().try_get(name) {
            for f in v.borrow().iter() {
                result.push(GeneralSymbol::Function(f.clone()));
            }
        }

        if let Some(v) = self.datatype_symbol_table.borrow().try_get(name) {
            result.push(GeneralSymbol::Datatype(v));
        }

        if let Some(s) = self.selector_symbol_table.borrow().try_get(name) {
            result.push(GeneralSymbol::Selector(s));
        }

        result
    }

    pub fn try_get_selector(& self, name: &str) -> Option<Rc<SelectorDescription>> {
        self.selector_symbol_table.borrow().try_get(name)
    }

    pub fn try_get_user_function(& self, name: &str) -> Option<Rc<FunctionDescription>> {
        self.user_function_symbol_table.borrow().try_get(name)
    }

    pub fn try_get_variable(& self, name: &str) -> Option<Rc<VariableDescription>> {
        if let Some(fib) = &self.current_fib() {
            if let Some(v) = fib.variable_symbol_table.borrow().try_get(name) {
                return Some(v);
            }
        }
        self.variable_symbol_table.borrow().try_get(name)
    }

    pub fn try_peek_value(&self) -> Option<Value> {
        match self.value_stack.borrow().last() {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    pub fn try_pop_value(&self, min_stack_size: usize) -> Option<Value> {
        let stack = &mut *self.value_stack.borrow_mut();
        match stack.len() as i32 - min_stack_size as i32 {
            0 => None,
            1 => stack.pop(),
            2 => {
                //  There is a case where the user continued a stopped function where there will be two entries on the
                //  stack, rather than just 1 (the extra entry is the return value of the goto statement that resumed the stopped
                //  function)

                let result = stack.pop();
                stack.truncate(min_stack_size);
                result
            },
            _ => {
                dbg!(stack);
                panic!("internal error -- value stack not emptied");
            }
        }
    }

}
