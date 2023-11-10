//  The lexical module defines the structures and implementation of lexical scanning of the source.

use std::char;
use std::cell::RefCell;
use crate::{lexical::{CharacterClass, get_character_class}, workspace::WorkSpace};

use super::{TokenPosition, LineNumber};



pub struct Context<'a,'b: 'a> {
    pub source_string: &'a str,
    pub workspace: &'b WorkSpace,
    interior: RefCell<InteriorContext<'a>>,
} 

#[derive(Debug)]
struct InteriorContext<'a> {
    source_iterator: std::iter::Peekable<std::str::CharIndices<'a>>,
    source_length: usize,
    current_n: usize,
    current_index: usize,
    current_character: char,
    current_character_class: CharacterClass,
    next_line_number: LineNumber,
    current_line_number: Option<LineNumber>,
    next_n: usize,
    next_column: u32,
    current_column: Option<u32>,
    lexeme_starting_n: usize,
    lexeme_starting_index: usize,
    lexeme_starting_line_number: LineNumber,
    lexeme_starting_column: u32,
    integer_value: i64,
    integer_digit_count: i32,
    fractions_integer_value: i64,
    fractional_digit_count: i32,
    exponent_value: i32,
    exponent_sign: i32,
    is_double: bool,
}

impl<'a,'b> Context<'a,'b> {
    pub fn new(source_string: &'a str, workspace: &'b WorkSpace) -> Context<'a,'b> {
        Context {
            source_string: source_string,
            workspace: workspace,
            interior: RefCell::new(InteriorContext::new(source_string)) }
    }

    pub fn add_exponential_digit(self: &Self) -> Result<(),String>{
        self.interior.borrow_mut().add_exponential_digit()
    }

    pub fn add_fractional_digit(self: &Self) {
        self.interior.borrow_mut().add_fractional_digit();
    }

    pub fn add_integer_digit(self: &Self) -> Result<(),String> {
        self.interior.borrow_mut().add_integer_digit()
    }

    pub fn clear_number(self: &Self) {
        self.interior.borrow_mut().clear_number();
    }

    pub fn get_current_character(self: &Self) -> char {
        self.interior.borrow().get_current_character()
    }

    pub fn get_current_character_class(self: &Self) -> CharacterClass {
        self.interior.borrow().get_current_character_class()
    }

    pub fn get_current_position(&self) -> TokenPosition {
        self.interior.borrow().get_current_position()
    }

    pub fn get_float_value(self: &Self) -> f64 {
        self.interior.borrow().get_float_value()
    }

    pub fn get_integer_value(self: &Self) -> i64 {
        self.interior.borrow().get_integer_value()
    }

    pub fn get_lexeme_source(self: &Self) -> String {
        self.interior.borrow().get_lexeme_source(self.source_string)
    }

    pub fn get_starting_position(&self) -> TokenPosition {
        self.interior.borrow().get_starting_position()
    }

    pub fn is_double(&self) -> bool {
        self.interior.borrow().is_double
    }

    pub fn literal_next(self: &Self) -> bool {
        self.interior.borrow_mut().literal_next()
    }

    pub fn next(self: &Self) -> bool {
        self.interior.borrow_mut().next()
    }

    pub fn peek(self: &Self) -> CharacterClass {
        self.interior.borrow_mut().peek()
    }

    pub fn set_exponent_sign(self: &Self, s: i32) {
        self.interior.borrow_mut().set_exponent_sign(s);
    }

    pub fn set_is_double(&self) {
        self.interior.borrow_mut().set_is_double();
    }

    pub fn set_iterator_to_nth(&self, n: usize) {
        self.interior.borrow_mut().set_iterator_to_nth(self.source_string, n);
    }

    pub fn set_lexeme_start(self: &Self) {
        self.interior.borrow_mut().set_lexeme_start();
    }
}

impl<'a> InteriorContext<'a> {
    pub fn new(source_string: &'a str) -> InteriorContext {
        let mut context = InteriorContext {
            source_length: source_string.len(),
            source_iterator: source_string.char_indices().peekable(),
            next_n: 0,
            lexeme_starting_n: 0,
            lexeme_starting_index: 0,
            lexeme_starting_line_number: 0,
            lexeme_starting_column: 1,
            current_n: 0,
            current_index: 0,
            current_character: 'a',
            current_character_class: CharacterClass::Bos,
            next_line_number: 1,
            current_line_number: None,
            next_column: 0,
            current_column: None,
            integer_value: 0, integer_digit_count: 0, fractions_integer_value: 0, fractional_digit_count: 0, exponent_value: 0, exponent_sign: 1, is_double: false };
        context.next();
        context
    }

    pub fn add_exponential_digit(self: &mut Self) -> Result<(),String> {
        match self.exponent_value.checked_mul(10) {
            Some(v) => {
                match v.checked_add(self.current_character.to_digit(10).unwrap() as i32) {
                    Some(v) => self.exponent_value = v,
                    None => return Err(format!("arithmetic overflow")),
                }
            },
            None => return Err(format!("arithmetic overflow")),
        }
        Ok(())
    }

    pub fn add_fractional_digit(self: &mut Self) {
        match self.fractions_integer_value.checked_mul(10) {
            Some(v) => {
                match v.checked_add(i64::from(self.current_character.to_digit(10).unwrap())) {
                    Some(v) => {
                        self.fractions_integer_value = v;
                        self.fractional_digit_count += 1;
                        if self.integer_digit_count + self.fractional_digit_count > 8 {
                            self.is_double = true;
                        }
                    },
                    None => {},
                }
            },
            None => {},
        }
    }

    pub fn add_integer_digit(self: &mut Self) -> Result<(),String> {
        match self.integer_value.checked_mul(10) {
            Some(v) => {
                match v.checked_add(i64::from(self.current_character.to_digit(10).unwrap())) {
                    Some(v) => self.integer_value = v,
                    None => return Err(format!("arithmetic overflow")),
                }
            },
            None => return Err(format!("arithmetic overflow")),
        }
        if self.integer_value != 0 {
            self.integer_digit_count += 1;
            if self.integer_digit_count > 8 {
                self.is_double = true;
            }
        }
        Ok(())
    }

    pub fn clear_number(self: &mut Self) {
        self.integer_value = 0;
        self.integer_digit_count = 0;
        self.fractions_integer_value = 0;
        self.fractional_digit_count = 0;
        self.exponent_value = 0;
        self.exponent_sign = 1;
        self.is_double = false;
    }

    pub fn get_current_character(self: &Self) -> char {
        self.current_character
    }
    pub fn get_current_character_class(self: &Self) -> CharacterClass {
        self.current_character_class
    }
/* 
    pub fn get_current_n(&self) -> usize {
        self.current_n
    }
 */
    pub fn get_current_position(&self) -> TokenPosition {
        TokenPosition {n: self.current_n, index: self.current_index, line_number: self.current_line_number.unwrap(), column: self.current_column.unwrap() }
    }

    pub fn get_starting_position(&self) -> TokenPosition {
        TokenPosition {n: self.lexeme_starting_n, index: self.lexeme_starting_index, line_number: self.lexeme_starting_line_number, column: self.lexeme_starting_column }
    }

    pub fn get_float_value(self: &Self) -> f64 {
        let fraction: f64 = self.fractions_integer_value as f64 * 10.0_f64.powi(-i32::from(self.fractional_digit_count));
        let raw_result = self.integer_value as f64 + fraction;
        raw_result * 10.0_f64.powi(if self.exponent_sign > 0 { self.exponent_value } else { -self.exponent_value })
    }

    pub fn get_integer_value(self: &Self) -> i64 {
        self.integer_value
    }

    pub fn get_lexeme_source(self: &Self, source_string: &'a str) -> String {
        String::from(&source_string[self.lexeme_starting_index..self.current_index])
    }

    pub fn literal_next(self: &mut Self) -> bool {
        // self.current_line_number = Some(self.next_line_number);
        // self.current_column = Some(self.next_column);
        // let x = self.source_iterator.next();
        // if x.is_none() {
        //     self.current_index = self.source_length;
        //     self.current_character_class = CharacterClass::Eos;
        //     false
        // } else {
        //     self.current_n = self.next_n;
        //     self.next_n += 1;
        //     self.current_index = x.unwrap().0;
        //     self.current_character = x.unwrap().1;
        //     self.current_character_class = get_character_class(self.current_character);
        //     self.next_column += 1;
        //     true
        // }
        self.next()
    }

    pub fn next(self: &mut Self) -> bool {
        self.current_line_number = Some(self.next_line_number);
        self.current_column = Some(self.next_column);
        let x = self.source_iterator.next();
        if x.is_none() {
            self.current_index = self.source_length;
            self.current_character_class = CharacterClass::Eos;
            false
        } else {
            self.current_n = self.next_n;
            self.next_n += 1;
            self.current_index = x.unwrap().0;
            self.current_character = x.unwrap().1;
            self.current_character_class = get_character_class(self.current_character);
            if let CharacterClass::Newline =  self.current_character_class {
                self.next_line_number += 1;
                self.next_column = 1;
            } else {
                self.next_column += 1;
            }
            true
        }
    }

    pub fn peek(self: &mut Self) -> CharacterClass {
        let x = self.source_iterator.peek();
        if x.is_none() {
            CharacterClass::Eos
        } else {
            get_character_class(x.unwrap().1)
        }
    }

    pub fn set_exponent_sign(self: &mut Self, s: i32) {
        self.exponent_sign = s;
    }

    pub fn set_is_double(&mut self) {
        self.is_double = true;
    }

    pub fn set_iterator_to_nth(&mut self, source_string: &'a str, n: usize) {
        if self.current_n != n {
            self.source_iterator = source_string.char_indices().peekable();
            self.next_n = 0;
            self.lexeme_starting_n = 0;
            self.lexeme_starting_index = 0;
            self.lexeme_starting_line_number = 0;
            self.lexeme_starting_column = 1;
            self.current_n = 0;
            self.current_index = 0;
            self.current_character = 'a';
            self.current_character_class = CharacterClass::Bos;
            self.next_line_number = 1;
            self.current_line_number = None;
            self.next_column = 0;
            self.current_column = None;
            self.integer_value = 0; self.integer_digit_count = 0; self.fractions_integer_value = 0; self.fractional_digit_count = 0; self.exponent_value = 0; self.exponent_sign = 1; self.is_double = false;
        
            for _ in 0..=n {
                self.next();
            }
        }
    }

    pub fn set_lexeme_start(self: &mut Self) {
        self.lexeme_starting_n = self.current_n;
        self.lexeme_starting_index = self.current_index;
        self.lexeme_starting_line_number = self.current_line_number.unwrap();
        self.lexeme_starting_column = self.current_column.unwrap();
    }
}
