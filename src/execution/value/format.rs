//      This module holds the Value type's format function

use std::fmt;

use crate::workspace::WorkSpace;

use super::Value;

pub mod format_parser;
pub mod floating_point;
mod state_machine;

#[derive(Debug, Clone)]
pub enum FormatType {
    Default(FormatControl),
    FixedPoint(FormatControl),
    Real(FormatControl),
    Double(FormatControl),
    Free,
}

#[derive(Debug, Clone)]
pub struct FormatControl {
    pub integer_zero_suppression_digits: usize,
    pub integer_non_suppressed_digits: usize,
    pub decimal_required: bool,
    pub fractional_non_suppressed_digits: usize,
    pub fractional_zero_suppressed_digits: usize,
    pub exponent_symbol: Option<char>,
}

impl FormatControl {
    pub fn new() -> FormatControl {
        FormatControl { 
            integer_zero_suppression_digits: 0,
            integer_non_suppressed_digits: 0,
            decimal_required: false,
            fractional_non_suppressed_digits: 0,
            fractional_zero_suppressed_digits: 0,
            exponent_symbol: None
        }
    }
}

impl fmt::Display for FormatControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.exponent_symbol {
            None => {},
            Some(e) => write!(f, "{}", e)?,
        }

        if self.integer_zero_suppression_digits > 0 {
            write!(f, "{}z", self.integer_zero_suppression_digits)?;
        }
        if self.integer_non_suppressed_digits > 0 {
            write!(f, "{}d", self.integer_non_suppressed_digits)?;
        }
        if self.decimal_required {
            write!(f, ".")?;
            if self.fractional_non_suppressed_digits > 0 {
                write!(f, "{}d", self.fractional_non_suppressed_digits)?;
            }
            if self.fractional_zero_suppressed_digits > 0 {
                write!(f, "{}z", self.fractional_zero_suppressed_digits)?;
            }
        }

        Ok(())
    }
}


#[derive(Debug,Clone)]
pub struct ParsedF64 {
    pub is_negative: bool,
    pub digits: String,
    pub fractional_digits: i32,
}

impl ParsedF64 {
    pub fn new() -> ParsedF64 {
        ParsedF64 { is_negative: false, digits: String::new(), fractional_digits: 0, }
    }
}




impl super::Value {
    pub fn format(args: &[Value], workspace: &WorkSpace) -> Result<String,String> {
        let parser = workspace.get_format_parser();
        let mut last_format = FormatType::Default(FormatControl::new());
        let mut result = String::new();

        for arg in args {
            match (arg, &last_format) {
                (Value::Int(i), FormatType::Default(f)) => result += Value::format_int(*i, f)?.as_str(),
                (Value::Int(i), FormatType::Double(f)) => result += Value::format_float(*i as f64, false, &f.exponent_symbol.unwrap(), f, workspace)?.as_str(),
                (Value::Int(i), FormatType::FixedPoint(f)) => result += Value::format_int(*i, f)?.as_str(),
                (Value::Int(i), FormatType::Real(f)) => result += Value::format_float(*i as f64, false, &f.exponent_symbol.unwrap(), f, workspace)?.as_str(),
                (Value::Int(_), FormatType::Free) => result += format!("{}", arg).as_str(),
                (Value::Real(r), FormatType::Default(f)) => result += Value::format_float(*r as f64, false, &'E', f, workspace)?.as_str(),
                (Value::Real(r), FormatType::Double(f)) => result += Value::format_float(*r as f64, false, &f.exponent_symbol.unwrap(), f, workspace)?.as_str(),
                (Value::Real(r), FormatType::FixedPoint(f)) => result += Value::format_int(*r as i32, f)?.as_str(),
                (Value::Real(r), FormatType::Real(f)) => result += Value::format_float(*r as f64, false, &f.exponent_symbol.unwrap(), f, workspace)?.as_str(),
                (Value::Real(_), FormatType::Free) => result += format!("{}", arg).as_str(),
                (Value::Double(d), FormatType::Default(f)) => result += Value::format_float(*d, false, &'D', f, workspace)?.as_str(),
                (Value::Double(d), FormatType::Double(f)) => result += Value::format_float(*d as f64, false, &f.exponent_symbol.unwrap(), f, workspace)?.as_str(),
                (Value::Double(d), FormatType::FixedPoint(f)) => result += Value::format_int(*d as i32, f)?.as_str(),
                (Value::Double(d), FormatType::Real(f)) => result += Value::format_float(*d as f64, false, &f.exponent_symbol.unwrap(), f, workspace)?.as_str(),
                (Value::Double(_), FormatType::Free) => result += format!("{}", arg).as_str(),
                (Value::Sequence(seq), _) => {
                    if seq.as_datatype().as_string().as_str() == "string" {
                        last_format = parser.parse(&*seq.as_values())?;
                    } else {
                        return Err(format!("Value cannot be formatted"));   
                    }
                }
                _ => return Err(format!("Value cannot be formatted")),
            }
        }
        Ok(result)
    }

    fn format_float(number: f64, is_rounded: bool, exponent_symbol: &char, f: &FormatControl, workspace: &WorkSpace) -> Result<String,String> {
        let mut result = String::new();
        let mut parse_result = workspace.get_floating_point_parser().parse(number)?.clone();
        let mut exponent = 0;
        let exponent_allowed = f.exponent_symbol != None;
        let mut integer_digits = parse_result.digits.len() as i32 - parse_result.fractional_digits;
        let available_integer_zero_suppression_digits: usize;
        
        if parse_result.is_negative {
            if f.integer_zero_suppression_digits > 0 {
                available_integer_zero_suppression_digits = f.integer_zero_suppression_digits - 1;
            } else {
                return Err(format!("Format \"{}\" does not support negative numbers", f));
            }
        } else {
            available_integer_zero_suppression_digits = f.integer_zero_suppression_digits;
        }

        // Shift the decimal point until the integer portion fits into the allocated space

        while exponent_allowed && integer_digits > (f.integer_non_suppressed_digits + available_integer_zero_suppression_digits) as i32 {
            integer_digits -= 1;
            parse_result.fractional_digits += 1;
            exponent += 1;
        }

        //  First stage of handling required non-suppressed integer digits: borrow them from the fraction.
        //  Note that the parser removed leading zeroes to the left of the decimal point, so any zeroes at the head
        //  of the string are to the right of the decimal point.

        while exponent_allowed && integer_digits < f.integer_non_suppressed_digits as i32 && parse_result.fractional_digits > 0 {
            parse_result.fractional_digits -= 1;
            exponent -= 1;
            if parse_result.digits.chars().next().unwrap() != '0' {
                integer_digits += 1;
            } else {
                parse_result.digits = String::from(&parse_result.digits[1..]);
            }
        }

        if integer_digits > (f.integer_non_suppressed_digits + available_integer_zero_suppression_digits) as i32 {
            return Err(format!("Significant high-order digits lost"));
        }


        //  Second stage: prepend zeroes

        while integer_digits < f.integer_non_suppressed_digits as i32 {

            //  Note that at this point, fractional digits is 0

            parse_result.digits = format!("0{}", parse_result.digits);
            integer_digits += 1;
        }

        //  Now prepend the sign and any leading spaces

        if parse_result.is_negative {
            parse_result.digits = format!("-{}", parse_result.digits);
            integer_digits += 1;
        }
        while integer_digits < (f.integer_non_suppressed_digits + available_integer_zero_suppression_digits) as i32 {
            parse_result.digits = format!(" {}", parse_result.digits);
            integer_digits += 1;
        }

        let mut digit_iterator = parse_result.digits.chars();
        while integer_digits > 0 {
            let c = digit_iterator.next().unwrap();
            result = format!("{}{}", result, c);
            integer_digits -= 1;
        }

        let mut fractional_digits:i32 = 0;

        if f.decimal_required {
            result += ".";

            //  Now handle the fraction

            while fractional_digits < (f.fractional_non_suppressed_digits + f.fractional_zero_suppressed_digits) as i32 && 
                fractional_digits < parse_result.fractional_digits as i32 {
                result += String::from(digit_iterator.next().unwrap()).as_str();
                fractional_digits += 1;
            }
        }

        if !is_rounded {

            //  See if we should have rounded the result up

            let must_round: bool;
            match digit_iterator.next() {
                Some(c) => match c {
                    '5' => must_round =  !parse_result.is_negative,
                    '6' | '7' | '8' | '9' => {
                        must_round = true;

                    }
                    _ => must_round = false,
                }
                None => must_round = false,
            }

            if must_round {
                let mut round_quantity = f64::powi(10.0, exponent - fractional_digits as i32);
                if parse_result.is_negative {
                    round_quantity = -round_quantity;
                }
                return Value::format_float(number + round_quantity, true, exponent_symbol, f, workspace);
            }
        }

        if f.decimal_required {

            //  Pad out to the requested length

            while fractional_digits < f.fractional_non_suppressed_digits as i32 {
                result += "0";
                fractional_digits += 1;
            }

            while fractional_digits < (f.fractional_non_suppressed_digits + f.fractional_zero_suppressed_digits) as i32 {
                result += " ";
                fractional_digits += 1;
            }

            //  Check the tail to see if trailing suppressable zeroes snuck in...

            let mut trailing_zeroes = 0;
            while result.chars().nth(result.len() - 1 - trailing_zeroes).unwrap() == '0' {
                trailing_zeroes += 1;
            }
            if trailing_zeroes > f.fractional_zero_suppressed_digits {
                trailing_zeroes = f.fractional_zero_suppressed_digits;
            }
            if trailing_zeroes > 0 {
                result = format!("{}{}", &result[0..result.len()-trailing_zeroes], " ".repeat(trailing_zeroes));
            }
        }

        //  And the exponent

        if exponent_allowed {
            result += String::from(*exponent_symbol).as_str();
            result += if exponent < 0 {
                exponent = -exponent;
                 "-"
            } else {
                 " "
            };
    
            let exponent_string = exponent.to_string();
            match exponent_string.len() {
                1 => {
                    result += "0";
                    result += exponent_string.as_str();
                },
                2 => result += exponent_string.as_str(),
                _ => return Err(format!("Significant high-order digits lost")),
            }
        }

        Ok(result)

    }

    fn format_int(mut number: i32, f: &FormatControl) -> Result<String,String> {
        let sign = if number < 0 {number = -number; "-"} else {" "};
        let digits = format!("{}", number);
        Value::format_internal(sign, digits.as_str(), "", f)
    }

    fn format_internal(sign: &str, int_part: &str, float_part: &str, f: &FormatControl) -> Result<String,String> {
        let mut int_digits = String::from(int_part);

        let available_integer_zero_suppression_digits: usize;
        if sign == "-" {
            if f.integer_zero_suppression_digits > 0 {
                available_integer_zero_suppression_digits = f.integer_zero_suppression_digits - 1;
            } else {
                return Err(format!("Format \"{}\" does not support negative numbers", f));
            }
        } else {
            available_integer_zero_suppression_digits = f.integer_zero_suppression_digits;
        }

        if int_digits.len() > f.integer_non_suppressed_digits + available_integer_zero_suppression_digits {
            return Err(format!("Significant high-order digits lost"));
        }
        while int_digits.len() < f.integer_non_suppressed_digits { int_digits = format!("0{}", int_digits); };

        int_digits = format!("{}{}{}", 
            " ".repeat(available_integer_zero_suppression_digits + f.integer_non_suppressed_digits - int_digits.len()),
            if sign == "-" { "-" } else { "" }, 
            int_digits);

        let mut float_digits = String::new();
        if f.decimal_required || float_part.len() > 0 {
            float_digits += ".";
            while float_digits.len() < f.fractional_non_suppressed_digits + 1 { float_digits += "0"; }
            while float_digits.len() < f.fractional_zero_suppressed_digits + f.integer_non_suppressed_digits + 1 { float_digits += " "; }
        } 

        Ok(format!("{}{}", int_digits, float_digits))
    }

}
