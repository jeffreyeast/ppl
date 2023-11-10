//  This module manages optional functionality

use std::{collections:: HashMap, hash::Hash, str::FromStr, fmt::Display};


#[derive(Debug)]
pub struct Options<T>
where T:Eq, T:PartialEq, T:Hash, T:Clone
{
    options: HashMap<T,bool>,
}

impl<T> Options<T> 
where T:Display, T:Eq, T:PartialEq, T:Hash, T:Clone, T:FromStr, <T as FromStr>::Err:Display
{
    pub fn clear(&mut self, option: T) {
        self.options.insert(option, false);
    }

    pub fn clear_str(&mut self, option: &str) -> Result<(),String>{
        self.options.insert(T::from_str(option).map_err(|e| format!("{}", e))?, false);
        Ok(())
    }

    pub fn init(&mut self, iterator: &mut dyn Iterator<Item = T>) {
        for option in iterator {
            self.clear(option);
        }
    }

    pub fn is_set(&self, option: &T) -> bool {
        match self.options.get(option) {
            Some(result) => *result,
            None => false,
        }
    }

    pub fn is_set_str(&self, option: &str) -> Result<bool,String> {
        match self.options.get(&T::from_str(option).map_err(|e| format!("{}", e))?) {
            Some(result) => Ok(*result),
            None => Ok(false),
        }
    }

    pub fn new() -> Options<T> {
        Options { options: HashMap::new() }
    }

    pub fn set(&mut self, option: T) {
        self.options.insert(option, true);
    }

    pub fn set_str(&mut self, option: &str) -> Result<(),String> {
        self.options.insert(T::from_str(option).map_err(|e| format!("{}", e))?, true);
        Ok(())
    }

    pub fn show(&self) -> String {
        let mut result = String::new();
        let mut separator = "";

        for (option,setting) in &self.options {
            result += format!("{}{} - {}", separator, option.to_string(), 
                match setting {
                    true => "ON",
                    false => "OFF",
                }).as_str();
            separator = "\n";
        }

        result
    }
}