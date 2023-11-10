//      This module holds the parser for the format specification

use std::cell::RefCell;
use std::rc::Rc;

use crate::execution::value::Cell;

use super::state_machine::{State, Rule};
use super::{FormatType, FormatControl};



#[derive(Debug)]
pub struct Parser {
    format_type: RefCell<FormatType>,
    format_control: RefCell<FormatControl>,
    states: Vec<Rc<State<Parser>>>,
    number: RefCell<usize>,
}




impl  Parser {
    fn init_states() -> Vec<Rc<State<Parser>>> {
        let mut states = Vec::new();

        let stop =  Rc::new({
            State::<Parser>::new("STOP", true) 
        });
        states.push(stop.clone());

        let is_full = Rc::new({
            State::<Parser>::new ("IS_FULL", true)
        });
        states.push(is_full.clone());
    
        let checking_full = Rc::new(State::new ("CHECKING_FULL", false));
        let rules = [
            Rule::new(
                &['f', 'F'], 
                |parser,_c|  { *parser.format_type.borrow_mut() = FormatType::Free; Ok(()) }, 
                is_full.clone())
        ];
        checking_full.set_rules(&rules);
        states.push(checking_full.clone());
    
        let z_done2 = Rc::new(State::new ("Z_DONE2", true));
        states.push(z_done2.clone());
    
        let checking_z2 = Rc::new(State::new ("CHECKING_Z2", false));
        let rules = [
            Rule::new(
                &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], 
                |parser: &Parser,c: char|  { *parser.number.borrow_mut() = *parser.number.borrow() * 10 + c.to_digit(10).unwrap() as usize; Ok(()) }, 
                checking_z2.clone())
            ,
            Rule::new(
                &['z', 'Z'], 
                |parser,_c|  { 
                    parser.format_control.borrow_mut().fractional_zero_suppressed_digits = *parser.number.borrow(); 
                    Ok(()) }, 
                z_done2.clone())
        ];
        checking_z2.set_rules(&rules);
        states.push(checking_z2.clone());
    
        let d_done2 = Rc::new(State::new ("D_DONE", true));
        let   rules = [
            Rule::new(
                &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], 
                |parser,c|  { *parser.number.borrow_mut() = c.to_digit(10).unwrap() as usize; Ok(()) }, 
                checking_z2.clone())
            ];
        d_done2.set_rules(&rules);
        states.push(d_done2.clone());
    
        let checking_d2 = Rc::new(State::new("CHECKING_D2", false));
        let   rules = [
            Rule::new(
                &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], 
                |parser: &Parser,c:char|  { *parser.number.borrow_mut() = *parser.number.borrow() * 10 + c.to_digit(10).unwrap() as usize; Ok(()) }, 
                checking_d2.clone())
            ,
            Rule::new(
                &['d', 'D'], 
                |parser,_c|  { 
                    parser.format_control.borrow_mut().fractional_non_suppressed_digits = *parser.number.borrow(); 
                    Ok(()) }, 
                d_done2.clone())
            ,
            Rule::new(
                &['z', 'Z'], 
                |parser,_c|  { 
                    parser.format_control.borrow_mut().fractional_zero_suppressed_digits = *parser.number.borrow(); Ok(()) }, 
                z_done2.clone())
            ];
        checking_d2.set_rules(&rules);
        states.push(checking_d2.clone());
    
        let decimal = Rc::new(State::new("DECIMAL", true));
        let   rules = [
            Rule::new(
                &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], 
                |parser: &Parser,c|  { *parser.number.borrow_mut() = c.to_digit(10).unwrap() as usize; Ok(()) }, 
                checking_d2.clone())
            ];
        decimal.set_rules(&rules);
        states.push(decimal.clone());
    
        let d_done = Rc::new(State::new("D_DONE", true));
        let   rules = [
            Rule::new(
                &['.'], 
                |parser,_c|  { parser.format_control.borrow_mut().decimal_required = true; Ok(()) }, 
                decimal.clone())
            ];
        d_done.set_rules(&rules);
        states.push(d_done.clone());
    
        let checking_d = Rc::new(State::new("CHECKING_D", false));
        let   rules = [
            Rule::new(
                &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], 
                |parser: &Parser,c:char|  { *parser.number.borrow_mut() = *parser.number.borrow() * 10 + c.to_digit(10).unwrap() as usize; Ok(()) }, 
                checking_d.clone())
            ,
            Rule::new(
                &['d', 'D'], 
                |parser,_c|  { parser.format_control.borrow_mut().integer_non_suppressed_digits = *parser.number.borrow(); Ok(()) }, 
                d_done.clone())
            ,
            Rule::new(
                &['.'], 
                |parser,_c|  { parser.format_control.borrow_mut().decimal_required = true; Ok(()) }, 
                decimal.clone())
            ];
        checking_d.set_rules(&rules);
        states.push(checking_d.clone());
    
        let z_done = Rc::new(State::new("Z_DONE", true));
        let   rules = [
            Rule::new(
                &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], 
                |parser: &Parser,c|  { *parser.number.borrow_mut() = c.to_digit(10).unwrap() as usize; Ok(()) }, 
                checking_d.clone())
            ,
            Rule::new(
                &['.'], 
                |parser,_c|  { parser.format_control.borrow_mut().decimal_required = true; Ok(()) }, 
                decimal.clone())
            ];
        z_done.set_rules(&rules);
        states.push(z_done.clone());
    
        let ednum = Rc::new(State::new("EDNUM", false));
        let   rules = [
            Rule::new(
                &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], 
                |parser: &Parser,c|  { *parser.number.borrow_mut() = *parser.number.borrow() * 10 +  c.to_digit(10).unwrap() as usize; Ok(()) }, 
                ednum.clone())
            ,
            Rule::new(
                &['z', 'Z'], 
                |parser,_c|  { parser.format_control.borrow_mut().integer_zero_suppression_digits = *parser.number.borrow(); Ok(()) }, 
                z_done.clone())
            ,
            Rule::new(
                &['d', 'D'], 
                |parser,_c|  { parser.format_control.borrow_mut().integer_non_suppressed_digits = *parser.number.borrow(); Ok(()) }, 
                d_done.clone())
            ];
        ednum.set_rules(&rules);
        states.push(ednum.clone());
    
        let checking_ed = Rc::new(State::new("CHECKING_ED", true));
        let   rules = [
            Rule::new(
                &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], 
                |parser: &Parser,c|  { *parser.number.borrow_mut() = c.to_digit(10).unwrap() as usize; Ok(()) }, 
                ednum.clone()),
            Rule::new(
                &['.'], 
                |parser,_c|  { parser.format_control.borrow_mut().decimal_required = true; Ok(()) }, 
                decimal.clone()),
                ];
        checking_ed.set_rules(&rules);
        states.push(checking_ed.clone());
    
        let start_state = Rc::new(State::new("START_STATE", false));
        let  rules = [
            Rule::new(
                &['f', 'F'],
                |_parser: &Parser,_c| Ok(()),
                checking_full.clone()),
             Rule::new(
                &['d', 'D'],
                |parser,c| { 
                    *parser.format_type.borrow_mut() = FormatType::Double(FormatControl::new()); 
                    parser.format_control.borrow_mut().exponent_symbol = Some(c); 
                    Ok(())},
                checking_ed.clone()),
            Rule::new(
                &['e', 'E'],
                |parser,c| { 
                    *parser.format_type.borrow_mut() = FormatType::Real(FormatControl::new()); 
                    parser.format_control.borrow_mut().exponent_symbol = Some(c); 
                    Ok(())},
                checking_ed.clone()),
            Rule::new(
                &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], 
                |parser,c|  { *parser.number.borrow_mut() = c.to_digit(10).unwrap() as usize; Ok(()) }, 
                ednum.clone()),
            Rule::new(
                &['.'], 
                |parser,_c|  { parser.format_control.borrow_mut().decimal_required = true; Ok(()) }, 
                decimal.clone())];
        start_state.set_rules(&rules);                 
        states.push(start_state.clone());
    
        states
    
    }

    pub fn new() -> Parser {
        let parser = Parser { 
            format_type: RefCell::new(FormatType::Default(FormatControl::new())), 
            format_control: RefCell::new(FormatControl::new()), 
            states: Parser::init_states(),
            number: RefCell::new(0) };
        parser
    }

    pub fn parse(& self, s: &Vec<Rc<RefCell<Cell>>>) -> Result<FormatType,String> {
        *self.format_type.borrow_mut() = FormatType::Default(FormatControl::new());
        *self.format_control.borrow_mut() = FormatControl::new();
        let mut current_state = self.states.last().unwrap().clone();

        for cell in s {
            current_state = current_state.execute(cell.borrow().as_ref_to_value().as_char()?, self)?;
        }
        //println!("{}", current_state.name);
        if current_state.is_stoppable {
            match &*self.format_type.borrow() {
                FormatType::Default(_) => Ok(FormatType::Default(self.format_control.borrow().clone())),
                FormatType::FixedPoint(_) => Ok(FormatType::FixedPoint(self.format_control.borrow().clone())),
                FormatType::Real(_) => Ok(FormatType::Real(self.format_control.borrow().clone())),
                FormatType::Double(_) => Ok(FormatType::Double(self.format_control.borrow().clone())),
                FormatType::Free => Ok(FormatType::Free),
            }
        } else {
            Err(format!("Invalid format specification"))
        }
    }
}



