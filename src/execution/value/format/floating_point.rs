//      This module holds the parser for floating point numbers

use std::cell::RefCell;
use std::rc::Rc;

use super::state_machine::{State, Rule};
use super::ParsedF64;



#[derive(Debug)]
pub struct Parser {
    parse_result: RefCell<ParsedF64>,
    states: Vec<Rc<State<Parser>>>,
}




impl  Parser {
    fn init_states() -> Vec<Rc<State<Parser>>> {
        let mut states = Vec::new();

        let stop = Rc::new(State::new("FP", true));
        let rules = [
        ];
        stop.set_rules(&rules);
        states.push(stop.clone());

        let fp = Rc::new(State::new("FP", false));
        let rules = [
            Rule::new (
                &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], 
                |parser: &Parser, c: char| { 
                    parser.parse_result.borrow_mut().digits.push(c); 
                    parser.parse_result.borrow_mut().fractional_digits += 1; 
                    Ok(()) 
                },
                fp.clone()
            ),
            Rule::new (
                &['\n'], 
                |_parser: &Parser, _c: char| { 
                    Ok(()) 
                },
                stop.clone()
            ),
        ];
        fp.set_rules(&rules);
        states.push(fp.clone());

        let ip = Rc::new(State::new("IP", false));
        let rules = [
            Rule::new (
                &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], 
                |parser: &Parser, c: char| { 
                    parser.parse_result.borrow_mut().digits.push(c); 
                    Ok(()) 
                },
                ip.clone()
            ),
            Rule::new (
                &['.'],
                |_parser: &Parser, _c: char| { 
                    Ok(()) 
                },
                fp.clone()
            ),
            Rule::new (
                &['\n'], 
                |_parser: &Parser, _c: char| { 
                    Ok(()) 
                },
                stop.clone()
            ),
        ];
        ip.set_rules(&rules);
        states.push(ip.clone());
        
        let leading_zeroes = Rc::new(State::new("LEADING_ZEROES", false));
        let rules = [
            Rule::new (
                &['1', '2', '3', '4', '5', '6', '7', '8', '9'], 
                |parser: &Parser, c: char| { 
                    parser.parse_result.borrow_mut().digits = String::from(c); 
                    parser.parse_result.borrow_mut().fractional_digits = 0;
                    Ok(()) 
                },
                ip.clone()
            ),
            Rule::new (
                &['0'], 
                |_parser: &Parser, _c: char| { 
                    Ok(()) 
                },
                leading_zeroes.clone()
            ),
            Rule::new (
                &['.'],
                |parser: &Parser, _c: char| { 
                    parser.parse_result.borrow_mut().digits = String::new();
                    Ok(()) 
                },
                fp.clone()
            ),
            Rule::new (
                &['\n'],
                |parser: &Parser, _c: char| { 
                    parser.parse_result.borrow_mut().digits = String::from('0'); 
                    parser.parse_result.borrow_mut().is_negative = false;
                    Ok(()) 
                },
                stop.clone()
            ),
        ];
        leading_zeroes.set_rules(&rules);
        states.push(leading_zeroes.clone());
        
        let sign_done = Rc::new(State::new("SIGN_DONE", false));
        let rules = [
            Rule::new (
                &['1', '2', '3', '4', '5', '6', '7', '8', '9'], 
                |parser: &Parser, c: char| { 
                    parser.parse_result.borrow_mut().digits = String::from(c); 
                    parser.parse_result.borrow_mut().fractional_digits = 0;
                    Ok(()) 
                },
                ip.clone()
            ),
            Rule::new (
                &['0'], 
                |parser: &Parser, _c: char| { 
                    parser.parse_result.borrow_mut().fractional_digits = 0;
                    Ok(()) 
                },
                leading_zeroes.clone()
            ),
            Rule::new (
                &['.'],
                |parser: &Parser, _c: char| { 
                    parser.parse_result.borrow_mut().digits = String::new();
                    parser.parse_result.borrow_mut().fractional_digits = 0;
                    Ok(()) 
                },
                fp.clone()
            ),
        ];
        sign_done.set_rules(&rules);
        states.push(sign_done.clone());
        
        let start = Rc::new(State::new ("START", false));
        let rules = [
            Rule::new (
                &['-'],
                |parser: &Parser, _c: char| { 
                    parser.parse_result.borrow_mut().is_negative = true; 
                    Ok(())
                },
                sign_done.clone()
            ),
            Rule::new (
                &['1', '2', '3', '4', '5', '6', '7', '8', '9'], 
                |parser: &Parser, c: char| { 
                    parser.parse_result.borrow_mut().is_negative = false;
                    parser.parse_result.borrow_mut().digits = String::from(c); 
                    parser.parse_result.borrow_mut().fractional_digits = 0;
                    Ok(()) 
                },
                ip.clone()
            ),
            Rule::new (
                &['0'], 
                |parser: &Parser, _c: char| { 
                    parser.parse_result.borrow_mut().is_negative = false;
                    parser.parse_result.borrow_mut().fractional_digits = 0;
                    Ok(()) 
                },
                leading_zeroes.clone()
            ),
            Rule::new (
                &['.'],
                |parser: &Parser, _c: char| { 
                    parser.parse_result.borrow_mut().is_negative = false;
                    parser.parse_result.borrow_mut().digits = String::new();
                    parser.parse_result.borrow_mut().fractional_digits = 0;
                    Ok(()) 
                },
                fp.clone()
            ),
        ];
        start.set_rules(&rules);
        states.push(start.clone());
    
        states
    }

    pub fn new() -> Parser {
        Parser { 
            parse_result: RefCell::new(ParsedF64::new()),
            states: Parser::init_states(),
        }
    }

    pub fn parse(& self, number: f64) -> Result<ParsedF64,String> {
        let mut current_state = self.states.last().unwrap().clone();

        for c in number.to_string().chars() {
            current_state = current_state.execute(c, self)?;
        }
        current_state = current_state.execute('\n', self)?;
        //println!("{}", current_state.name);
        if current_state.is_stoppable {
            Ok(self.parse_result.borrow().clone())
        } else {
            Err(format!("Invalid format specification"))
        }
    }
}



