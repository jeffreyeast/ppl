//  This module holds the datatype conversion methods for Value

use crate::{workspace::GeneralSymbol, lexical::LineNumber};

use super::Value;



impl Value {
    pub fn as_bool(&self) -> Result<bool,String> {
        match self {
            Value::Bool(v) => Ok(*v),
            Value::Int(v) => Ok(if *v == 0 { false } else { true }),
            Value::Real(v) => Ok(if *v == 0.0 { false } else { true }),
            Value::Double(v) => Ok(if *v == 0.0 { false } else { true }),
            Value::Char(c) => {
                match c.to_ascii_lowercase() {
                    'f' => Ok(false),
                    't' => Ok(true),
                    _ => Err(format!("{} is not a valid BOOL", c)),
                }
            },
            Value::Symbol(symbol) => {
                match symbol.as_symbol() {
                    GeneralSymbol::Variable(v) => {
                        v.cell.borrow().as_contents().value.borrow().as_bool()
                    },
                    _ => Err(format!("Cannot convert {} to bool", symbol))
                }
            },
            Value::ValueByReference(v) => {
                v.cell.borrow().as_ref_to_value().as_bool()
            },
            _ => Err(format!("Invalid BOOL")),
        }
    }

    pub fn as_char(&self) -> Result<char,String> {
        match self {
            Value::Char(c) => Ok(*c),
            Value::Symbol(symbol) => {
                match symbol.as_symbol() {
                    GeneralSymbol::Variable(v) => {
                        v.cell.borrow().as_contents().value.borrow().as_char()
                    },
                    _ => Err(format!("Cannot convert {} to CHAR", symbol))
                }
            },
            Value::ValueByReference(v) => {
                v.cell.borrow().as_ref_to_value().as_char()
            },
            _ => Err(format!("Invalid CHAR")),
        }
    }

    pub fn as_f32(&self) -> Result<f32,String> {
        match self {
            Value::Bool(v) => Ok(if *v { 1.0 } else { 0.0 }),
            Value::Int(v) => Ok(*v as f32),
            Value::Real(v) => Ok(*v as f32),
            Value::Double(v) => Ok(*v as f32),
            Value::Char(c) => {
                if c.is_digit(10) {
                    Ok(c.to_digit(10).unwrap() as f32)
                } else {
                    Err(format!("'{} is not a digit", c)) 
                }
            },
            Value::Symbol(symbol) => {
                match symbol.as_symbol() {
                    GeneralSymbol::Variable(v) => {
                        v.cell.borrow().as_ref_to_value().as_f32()
                    },
                    _ => Err(format!("Cannot convert {} to REAL", symbol))
                }
            },
            Value::ValueByReference(v) => {
                v.cell.borrow().as_ref_to_value().as_f32()
            },
            _ => Err(format!("Unable to cast to REAL")),
        }
    }

    pub fn as_f64(&self) -> Result<f64,String> {
        match self {
            Value::Bool(v) => Ok(if *v { 1.0 } else { 0.0 }),
            Value::Int(v) => Ok(*v as f64),
            Value::Real(v) => Ok(*v as f64),
            Value::Double(v) => Ok(*v as f64),
            Value::Char(c) => {
                if c.is_digit(10) {
                    Ok(c.to_digit(10).unwrap() as f64)
                } else {
                    Err(format!("'{} is not a digit", c)) 
                }
            },
            Value::Symbol(symbol) => {
                match symbol.as_symbol() {
                    GeneralSymbol::Variable(v) => {
                        v.cell.borrow().as_contents().value.borrow().as_f64()
                    },
                    _ => Err(format!("Cannot convert {} to DBL", symbol))
                }
            },
            Value::ValueByReference(v) => {
                v.cell.borrow().as_ref_to_value().as_f64()
            },
            _ => Err(format!("Unable to cast to DBL")),
        }
    }

    pub fn as_i32(&self) -> Result<i32,String> {
        match self {
            Value::Bool(v) => Ok(if *v { 1 } else { 0 }),
            Value::Int(v) => Ok(*v as i32),
            Value::Real(v) => Ok(*v as i32),
            Value::Double(v) => Ok(*v as i32),
            Value::Char(c) => {
                if c.is_digit(10) {
                    Ok(c.to_digit(10).unwrap() as i32)
                } else {
                    Err(format!("'{} is not a digit", c)) 
                }
            },
            Value::Symbol(symbol) => {
                match symbol.as_symbol() {
                    GeneralSymbol::Variable(v) => {
                        v.cell.borrow().as_contents().value.borrow().as_i32()
                    },
                    _ => Err(format!("Cannot convert {} to INT", symbol))
                }
            },
            Value::ValueByReference(v) => {
                v.cell.borrow().as_ref_to_value().as_i32()
            },
            _ => Err(format!("Unable to cast to INT")),
        }
    }

    pub fn as_i64(&self) -> Result<i64,String> {
        match self {
            Value::Bool(v) => Ok(if *v { 1 } else { 0 }),
            Value::Int(v) => Ok(*v as i64),
            Value::Real(v) => Ok(*v as i64),
            Value::Double(v) => Ok(*v as i64),
            Value::Char(c) => {
                if c.is_digit(10) {
                    Ok(c.to_digit(10).unwrap() as i64)
                } else {
                    Err(format!("'{} is not a digit", c)) 
                }
            },
            Value::Symbol(symbol) => {
                match symbol.as_symbol() {
                    GeneralSymbol::Variable(v) => {
                        v.cell.borrow().as_contents().value.borrow().as_i64()
                    },
                    _ => Err(format!("Cannot convert {} to INT", symbol))
                }
            },
            Value::ValueByReference(v) => {
                v.cell.borrow().as_ref_to_value().as_i64()
            },
            _ => Err(format!("Unable to cast to INT")),
        }
    }

    pub fn as_line_number(&self) -> Result<LineNumber,String> {
        self.as_u32()
    }
    
    pub fn as_u32(&self) -> Result<u32,String> {
        match self {
            Value::Bool(v) => Ok(if *v { 1 } else { 0 }),
            Value::Int(v) => Ok(*v as u32),
            Value::Real(v) => Ok(*v as u32),
            Value::Double(v) => Ok(*v as u32),
            Value::Char(c) => {
                if c.is_digit(10) {
                    Ok(c.to_digit(10).unwrap() as u32)
                } else {
                    Err(format!("'{} is not a digit", c)) 
                }
            },
            Value::Symbol(symbol) => {
                match symbol.as_symbol() {
                    GeneralSymbol::Variable(v) => {
                        v.cell.borrow().as_contents().value.borrow().as_u32()
                    },
                    _ => Err(format!("Cannot convert {} to INT", symbol))
                }
            },
            Value::ValueByReference(v) => {
                v.cell.borrow().as_ref_to_value().as_u32()
            },
            _ => Err(format!("Unable to cast to INT")),
        }
    }

    pub fn as_string(&self) -> String {

        //  The PPL spec specifies that floating point numbers are displayed in their shortest
        //  form.  The code below does this. Note, however, that PPL was built on a PDP-10,
        //  which has a 36-bit word.  So the precision in the spec doesn't match the IEEE formats
        //  used on the x86.
        
        match self {
            Value::Real(r) => {
                let s1 = format!("{}", r);
                let s2 = format!("{:.4E}", r);      //  Precision > 4 doesn't work
                let s3;
                if s2.contains('E') {
                    s3 = s2;
                } else {
                    s3 = String::from(s2.trim_end_matches('0'));
                }
                if s1.len() <= s3.len() {
                    s1
                } else {
                    s3
                }
            },
            Value::Double(d) => {
                let s1 = format!("{}", d);
                let s2 = format!("{:.12E}", d);     //  Precision > 12 doesn't work
                let s3;
                if s2.contains('E') {
                    s3 = s2.replace('E',"D");
                } else {
                    s3 = String::from(s2.trim_end_matches('0'));
                }
                if s1.len() <= s3.len() {
                    s1
                } else {
                    s3
                }
            }
            Value::Symbol(symbol) => {
                match symbol.as_symbol() {
                    GeneralSymbol::Datatype(d) => d.as_string(),
                    GeneralSymbol::Function(f) => f.as_string(),
                    GeneralSymbol::Selector(s) => s.as_string(),
                    GeneralSymbol::Variable(_) | GeneralSymbol::Unresolved(_) => symbol.as_string(),
                }
            },
            Value::ValueByReference(v) => {
                v.cell.borrow().as_ref_to_value().as_string()
            },
            _ => format!("{}", self),
        }
    }

    pub fn as_usize(&self) -> Result<usize,String> {
        match self {
            Value::Bool(v) => Ok(if *v { 1 } else { 0 }),
            Value::Int(v) => Ok(*v as usize),
            Value::Real(v) => Ok(*v as usize),
            Value::Double(v) => Ok(*v as usize),
            Value::Char(c) => {
                if c.is_digit(10) {
                    Ok(c.to_digit(10).unwrap() as usize)
                } else {
                    Err(format!("'{} is not a digit", c)) 
                }
            },
            Value::Symbol(symbol) => {
                match symbol.as_symbol() {
                    GeneralSymbol::Variable(v) => {
                        v.cell.borrow().as_contents().value.borrow().as_usize()
                    },
                    _ => Err(format!("Cannot convert {} to usize", symbol))
                }
            },
            Value::ValueByReference(v) => {
                v.cell.borrow().as_ref_to_value().as_usize()
            },
            _ => Err(format!("Unable to cast to INT")),
        }
    }
}


