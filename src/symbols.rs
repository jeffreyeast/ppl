//  This module maintains the list of symbols

use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;

use crate::execution::value::Value;
use crate::execution::value::sequence::SequenceInstance;
use crate::symbols::help::Help;
use crate::workspace::WorkSpace;

use self::name::Name;

pub mod datatype;
pub mod help;
pub mod metadata;
pub mod name;




#[derive(Clone)]
pub struct SymbolTable<T: Help> {
    list: HashMap<String, Rc<T>>,
}

impl<T: Help> SymbolTable<T> {
    pub fn add (&mut self, name: Name, item: T) {
        self.list.insert(name.as_string().to_ascii_lowercase(), Rc::new(item));
    }

    pub fn add_by_reference (&mut self, name: Name, item: Rc<T>) {
        self.list.insert(name.as_string().to_ascii_lowercase(), item);
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn contains_any(&self, name: &str) -> bool {
        self.list.contains_key(name.to_ascii_lowercase().as_str())
    }

    pub fn get_all(&self) -> Vec<(String,Rc<T>)> {
        let mut result = Vec::new();
        for homonyms in &self.list {
            result.push((homonyms.0.clone(), homonyms.1.clone()));
        }
        return result;
    } 

    pub fn get_all_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for homonyms in &self.list {
            names.push(homonyms.0.clone());
        }
        names
    } 

    pub fn help_all(&self, workspace: &WorkSpace) -> Result<Value,String> {
        let mut result = String::new();

        //  Gather the list of all the symbols, together with their help text

        let mut symbols = self.get_all();
        symbols.sort_by(|a,b| a.0.cmp(&b.0));
        
        for (symbol_name, item) in symbols {
            result += format!("{} - {}\n", symbol_name, item.help_text(workspace).or(Some(String::from(""))).unwrap()).as_str();
        }

        Ok(SequenceInstance::construct_string_sequence(&result))
    }

    pub fn new() -> SymbolTable<T> {
        SymbolTable {list: HashMap::new() }
    }
    
    pub fn remove(&mut self, name: &str) {
        let normalized_name = name.to_ascii_lowercase();
        self.list.remove(normalized_name.as_str());
    }

    pub fn try_get(&self, name: &str) -> Option<Rc<T>> {
        let result = self.list.get(name.to_ascii_lowercase().as_str());
        match result {
            Some(item) => Some(item.clone()),
            None => None,
        }
    }
}

impl<T: Help> fmt::Debug for SymbolTable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k,i) in &self.list {
            writeln!(f, "{}: {}", k, i.as_ref().pretty_print())?;
        }
        Ok(())
    }        
}
