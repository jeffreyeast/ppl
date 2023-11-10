//  This module implements the Name structure

use std::fmt;




#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    value: String,
}

impl Name {

    pub fn as_str<'a> (&'a self) -> &'a str {
        self.value.as_str()
    }

    pub fn as_string(&self) -> String {
        self.value.clone()
    }

    pub fn from_str(s: &str) -> Name {
        Name { value: String::from(s)}
    }

    pub fn from_string(s: &String) -> Name {
        Name { value: s.clone()}
    }

    pub fn from_name(n: &Name) -> Name {
        n.clone()
    }

    pub fn is_name_a_legal_identifier(name: &str) -> bool {
        for c in name.chars() {
            if !(c.is_alphabetic() || c.is_alphanumeric() || c == '.') {
                return false;
            }
        }

        name != "."
    }
}

impl fmt::Display for Name {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.as_string())
    }
}
