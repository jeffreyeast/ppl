//  This module defines a scanner of the lexicazl token vector

use std::iter::Peekable;

use crate::lexical::{TokenType, Token, TokenPosition};



pub struct TokenScanner<'a> {
    iterator: Peekable<std::slice::Iter<'a,Token>>,
    saved_iterator: Option<Peekable<std::slice::Iter<'a,Token>>>,
    position: TokenPosition,
    eol_token: Token,
}

impl<'a> TokenScanner<'a> {
    fn construct_eol_token(&mut self, position: TokenPosition) -> &Token {
        self.eol_token = Token { token_type: TokenType::EOL, string_value: String::from(""), starting_position: position };
        &self.eol_token
    }

    pub fn consume_any_operator(&mut self) -> Result<String,String> {
        let t = self.peek();
        match t {
            Some(token) => {
                if let TokenType::Operator(_) = token.token_type {
                    let token = self.next();
                    return Ok(token.unwrap().string_value.clone());
                }
            },
            None => {},
        }
        Err(format!("operator not found"))
    }

   /*  pub fn consume_any_punctuation(&mut self) -> Result<String,String> {
        let t = self.peek();
        match t {
            Some(token) => {
                if let TokenType::Punctuation(_) = token.token_type {
                    let token = self.next();
                    return Ok(token.unwrap().string_value.clone());
                }
            },
            None => {},
        }
        Err(format!("punctuation not found"))
    } */

    pub fn consume_identifier(&mut self) -> Result<String,String> {
        let t = self.peek();
        match t {
            Some(token) => {
                if let TokenType::Identifier(_) = token.token_type {
                    let token = self.next();
                    return Ok(token.unwrap().string_value.clone());
                }
            },
            None => {},
        }
        Err(format!("identifier not found"))
    }

    pub fn consume_integer(&mut self) -> Result<i32,String> {
        let t = self.peek();
        match t {
            Some(token) => {
                if let TokenType::Integer(i) = token.token_type {
                    self.next();
                    return Ok(i);
                }
            },
            None => {},
        }
        Err(format!("integer not found"))
    }

    pub fn consume_keyword(&mut self, expected: &str) -> Result<String,String> {
        let t = self.peek();
        match t {
            Some(token) => {
                if let TokenType::Identifier(i) = &token.token_type {
                    if i.to_lowercase() == expected.to_ascii_lowercase() {
                        let token = self.next();
                        return Ok(token.unwrap().string_value.clone());
                    }
                }
            },
            None => {},
        }
        Err(format!("{} not found", expected))
    }

    pub fn consume_newline(&mut self) -> Result<(),String> {
        let t = self.peek();
        match t {
            Some(token) => {
                if let TokenType::EOL = token.token_type {
                    self.next();
                    return Ok(());
                }
            },
            None => return Ok(()),
        }
        Err(format!("newline not found"))
    }

    pub fn consume_operator(&mut self, expected: &str) -> Result<String,String> {
        let t = self.peek();
        match t {
            Some(token) => {
                if let TokenType::Operator(ref op) = token.token_type {
                    if op == expected {
                        let token = self.next();
                        return Ok(token.unwrap().string_value.clone());
                    }
                }
            },
            None => {},
        }
        Err(format!("{} not found", expected))
    }

    pub fn consume_punctuation(&mut self, expected: &str) -> Result<String,String> {
        let t = self.peek();
        match t {
            Some(token) => {
                if let TokenType::Punctuation(ref p) = token.token_type {
                    if p == expected {
                        let token = self.next();
                        return Ok(token.unwrap().string_value.clone());
                    }
                }
            },
            None => {},
        }
        Err(format!("{} not found", expected))
    }

    pub fn discard_saved_iterator(&mut self) {
        if self.saved_iterator.is_some() {
            self.saved_iterator = None;
        } else {
            panic!("internal error");
        }
    }

    pub fn get_position(&self) -> TokenPosition {
        self.position.clone()
    }

    pub fn is_eos(&mut self) -> bool {
        match self.iterator.peek() {
            Some(t) => {
                match t.token_type {
                    TokenType::EOS => true,
                    _ => false,
                }
            } ,
            None => true,
        }
    }

    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { 
            iterator: tokens.iter().peekable(), 
            saved_iterator: None,
            position: TokenPosition { n:0, index: 0, line_number: 0, column: 0 }, 
            eol_token: Token { token_type: TokenType::EOL, string_value: String::from(""), starting_position: TokenPosition { n:0, index: 0, line_number: 0, column: 0 } } }
    }

    pub fn next(&mut self) -> Option<&Token> {
        let t = self.iterator.next();
        match t {
            Some(t) => {
                if let TokenType::Comment = t.token_type {
                    Some(self.construct_eol_token(t.starting_position.clone()))
                } else {
                    self.position = t.starting_position.clone();
                    Some(t)
                }
            },
            None => {
                Some(self.construct_eol_token(self.position.clone()))
            },
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        let t = self.iterator.peek();
        match t {
            Some(t) => {
                if let TokenType::Comment = t.token_type {
                    let p = t.starting_position.clone();
                    Some(self.construct_eol_token(p))
                } else {
                    Some(*t)
                }
            },
            None => {
                Some(self.construct_eol_token(self.position.clone()))
            },
        }
    }

    pub fn peek_identifier(&mut self) -> bool {
        let t = self.peek();
        match t {
            Some(token) => {
                if let TokenType::Identifier(_) = token.token_type {
                    true
                } else {
                    false
                }
            },
            None => false,
        }
    }

    pub fn peek_operator(&mut self, expected: &str) -> bool {
        let t = self.peek();
        match t {
            Some(token) => {
                if let TokenType::Operator(ref op) = token.token_type {
                    if op == expected {
                        return true;
                    }
                }
            },
            None => {},
        }
        false
    }

    pub fn peek_punctuation(&mut self, expected: &str) -> bool {
        let t = self.peek();
        match t {
            Some(token) => {
                if let TokenType::Punctuation(ref p) = token.token_type {
                    if p == expected {
                        return true;
                    }
                }
            },
            None => {},
        }
        false
    }

    pub fn pop_iterator(&mut self) {
        match self.saved_iterator {
            Some(ref iterator) => {
                self.iterator = iterator.clone();
                self.saved_iterator = None;
            },
            None => panic!("internal error"),
        }
    }

    pub fn push_iterator(&mut self) {
        if self.saved_iterator.is_none() {
            self.saved_iterator = Some(self.iterator.clone());
        } else {
            panic!("internal error");
        }
    }
}
