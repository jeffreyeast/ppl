//      This module holds the definition of the state machine used in format manipulation

use std::{cell::RefCell, fmt};
use std::rc::Rc;



#[derive(Debug)]
pub struct State<T> {
    pub name: String,
    pub rules: RefCell<Vec<Rule<T>>>,
    pub is_stoppable: bool,
}

impl<T> State<T> {
    pub fn execute(&self, c: char, parser: &T) -> Result<Rc<State<T>>,String> {
      //  println!("{}: {}", self.name, c);
        for rule in &*self.rules.borrow() {    
        //    println!("Checking {}", rule);
            if rule.matching_characters.contains(&c) {
          //      println!("Matched");
                match (rule.action)(parser, c) {
                    Ok(_) => return Ok(rule.next_state.clone()),
                    Err(e) => return Err(e),
                }
            }
        }
        Err(format!("Invalid format specification"))
    }

    pub fn new(name: &str, is_stoppable: bool, ) -> State<T> {
        State { name: String::from(name), rules: RefCell::new(Vec::new()), is_stoppable: is_stoppable }
    }

    pub fn set_rules(&self, rules: &[Rule<T>]) {
         for rule in rules {
            self.rules.borrow_mut().push(rule.clone());
        }
    }
}



#[derive(Debug)]
pub struct Rule<T> {
    pub matching_characters: Vec<char>,
    pub action:    fn(&T, char) -> Result<(),String>,
    pub next_state: Rc<State<T>>,
}

impl<T> Rule<T> {
    pub fn new(matching_characters: &[char], action: fn(parser: &T, c: char) -> Result<(),String>, next_state: Rc<State<T>>) -> Rule<T> {
        Rule { matching_characters: matching_characters.to_vec(), action: action, next_state: next_state.clone() }
    }
}

impl<T> Clone for Rule<T> {
    fn clone(&self) -> Self {
        Rule::new(&self.matching_characters, self.action.clone(), self.next_state.clone())
    }
}

impl<T> fmt::Display for Rule<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matching characters {{")?;
        for c in &self.matching_characters {
            write!(f, "{}", c)?;
        }
        write!(f, "}}")
    }
}
