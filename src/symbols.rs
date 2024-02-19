//  This module maintains the list of symbols

use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

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

    pub fn dump(&self) -> String {
        let mut names = self.get_all_names();
        names.sort();
        let mut line = String::from("    ");
        let mut result = String::new();
        for name in &names {
            let padding_count = 8 - (name.len() % 8);
            line = format!("{}{}{}", line, name, " ".repeat(padding_count));
            if line.len() >= 80 {
                result += line.as_str();
                result += "\n";
                line = String::from("    ");
            }
        }
        if line.len() > 4 {
            result += line.as_str();
            result += "\n";
        }
        result
    }

    pub fn get_all(&self) -> Vec<(String,Rc<T>)> {
        let mut result = Vec::new();
        result.reserve(self.list.len());
        for homonyms in &self.list {
            result.push((homonyms.0.clone(), homonyms.1.clone()));
        }
        return result;
    } 

    pub fn get_all_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        names.reserve(self.list.len());
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

        let mut total_space = 0;
        for (symbol_name, item) in &symbols {
            total_space += symbol_name.len() + item.help_text_len(workspace) + 4;
        }
        result.reserve(total_space);

        for (symbol_name, item) in &symbols {
            result += format!("{} - {}\n", symbol_name, item.help_text(workspace).or(Some(String::from(""))).unwrap()).as_str();
        }
        let t1 = Instant::now();
        let v = SequenceInstance::construct_string_sequence(&result);
        let t6 = t1.elapsed();
        println!("t6={}", t6.as_millis());

        Ok(v)
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
