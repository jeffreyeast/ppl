//  The lexical module defines the structures and implementation of lexical scanning of the source.

mod context;

use std::char;
use std::fmt;
use context::Context;

use crate::workspace::WorkSpace;


pub type LineNumber = u32;

#[derive(Clone)]
pub struct TokenPosition {
    pub n: usize,
    pub index: usize,
    pub line_number: LineNumber,
    pub column: u32,
}

impl fmt::Debug for TokenPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{({},{}) {}:{}}}", self.n, self.index, self.line_number, self.column)
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub string_value: String,
    pub starting_position: TokenPosition,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:\t\"{}\"\t{:?}", &self.token_type, &sanitize_for_display(self.string_value.as_str()), &self.starting_position)
    }
}

#[derive(Clone)]
pub enum TokenType {
    Punctuation(String),
    Integer(i32),
    Double(f64),
    Real(f32),
    Identifier(String),
    Operator(String),
    Character(char),
    String(String),
    Comment,
    EOL,
    EOS,
}

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Punctuation(p) => write!(f, "Punctuation(\"{}\")", p),
            TokenType::Integer(i) => write!(f, "Int({})", i),
            TokenType::Double(d) => write!(f, "Dbl({})", d),
            TokenType::Real(r) => write!(f, "Real({})", r),
            TokenType::Identifier(id) => write!(f, "Identifier({})", id),
            TokenType::Operator(op) => write!(f, "Operator(\"{}\")", op),
            TokenType::Character(c) => write!(f, "Char('{}')", c),
            TokenType::String(s) => write!(f, "String(\"{}\"", sanitize_for_display(s.as_str())),
            TokenType::Comment => write!(f, "Comment"),
            TokenType::EOL => write!(f, "EOL"),
            TokenType::EOS => write!(f, "EOS"),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Punctuation(p) => writeln!(fmt, "Punctuation: {}", p),
            TokenType::Integer(v) => writeln!(fmt, "Integer: {}", v),
            TokenType::Real(v) => writeln!(fmt, "Real: {}", v),
            TokenType::Double(v) => writeln!(fmt, "Double: {}", v),
            TokenType::Identifier(i) => writeln!(fmt, "Identifier: {}", i),
            TokenType::Operator(op) => writeln!(fmt, "Operator: {}", op),
            TokenType::Character(c) => writeln!(fmt, "Character: {}", c),
            TokenType::String(s) => writeln!(fmt, "String: {}", s),
            TokenType::Comment => writeln!(fmt, "Comment"),
            TokenType::EOL => writeln!(fmt, "EOL"),
            TokenType::EOS => writeln!(fmt, "EOS"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CharacterClass {
    Bos,
    Alpha,
    Numeric,
    Dot,
    BackSlash,
    Dollar,
    Plus,
    Minus,
    SingleQuote,
    DoubleQuote,
    Punctuation,
    ComposableOperator,
    WhiteSpace,
    Newline,
    Eos,
    Other,
}

enum PostProcessTokenClass {
    Number,
    Minus,
    Plus,
    OperatorOrPunctuation,
    Other,
}

fn convert_escape_sequence(input: &str) -> char {
    match input {
        "\\n" => '\n',
        "\\r" => '\r',
        "\\t" => '\t',
        s => {
            let mut c = s.chars();
            if c.next().unwrap() == '\'' {
                return c.next().unwrap();
            }
            panic!("internal error");
        },
    }
}

fn convert_escape_sequences(input_string: &str) -> String {
    input_string.replace("\\r", "\r").replace("\\n", "\n").replace("\\t", "\t")
}

pub fn get_character_class(c: char) -> CharacterClass {
    match c {
        'a' | 'b' |  'c' |  'd' |  'e' |  'f' |  'g' |  'h' |  'i' |  'j' |  'k' |  'l' |  'm' |  'n' |  'o' |  'p' |  'q' |  'r' |  's' |  't' |  'u' |  'v' |  'w' |  'x' |  'y' |  'z' => CharacterClass::Alpha,
        'A' | 'B' |  'C' |  'D' |  'E' |  'F' |  'G' |  'H' |  'I' |  'J' |  'K' |  'L' |  'M' |  'N' |  'O' |  'P' |  'Q' |  'R' |  'S' |  'T' |  'U' |  'V' |  'W' |  'X' |  'Y' |  'Z' => CharacterClass::Alpha,
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => CharacterClass::Numeric,
        '\\' => CharacterClass::BackSlash,
        '.' => CharacterClass::Dot,
        '$' => CharacterClass::Dollar,
        '+' => CharacterClass::Plus,
        '-' => CharacterClass::Minus,
        '\'' => CharacterClass::SingleQuote,
        '"' => CharacterClass::DoubleQuote,
        '(' | ')' | '[' | ']' | ',' | ';' => CharacterClass::Punctuation,
        '*' | '>' | '=' | '<' | '^' | '_' | '&' | '#'  | '/' | '@' | ':' | '!' | '~' | '?' | '%' => CharacterClass::ComposableOperator,
        ' ' | '\t' | '\r' => CharacterClass::WhiteSpace,
        '\n' => CharacterClass::Newline,
        _ => CharacterClass::Other,
    }
}

pub struct  Lexer<'a,'b: 'a> {
    context:    &'b Context<'a,'b>,
}

impl<'a,'b: 'a> Lexer<'a,'b> {
    fn character(&self, tokens: &mut Vec<Token>) -> Result<bool, String> { 
        self.context.literal_next();
        match self.context.get_current_character_class() {
            CharacterClass::Eos => {
                return self.error("Missing character following single quote");
            },
            _ => {
                let c = self.context.get_current_character();
                self.context.next();
                tokens.push(Token { 
                    token_type: TokenType::Character(c), 
                    string_value: self.context.get_lexeme_source(),
                    starting_position: self.context.get_starting_position() });
                return Ok(true);
            },
        }
    }

    fn comment(&self, tokens: &mut Vec<Token>) -> Result<bool, String> {
        loop {
            match self.context.get_current_character_class() {
                CharacterClass::Newline => {
                    self.context.next();
                    break;
                },
                CharacterClass::Eos => {
                    break;
                },
                _ => {
                    self.context.next();
                },
            }
        }

        tokens.push(Token { 
            token_type: TokenType::Comment, 
            string_value: self.context.get_lexeme_source(),
            starting_position: self.context.get_starting_position() });
        return Ok(true);
    }

    fn dot(&self, tokens: &mut Vec<Token>) -> Result<bool, String> {
        match self.context.peek() {
            CharacterClass::Numeric => {
                self.context.clear_number();
                self.context.next();
                return self.fraction(tokens);
            },
            _ => {
                return self.operator(tokens);
            },
        }
    }

    fn error(&self, message: &str) -> Result<bool, String> {
        let current_position = self.context.get_current_position();
        let mut target_line =  self.context.source_string;
        let mut remaining_lines =  current_position.line_number;
        for source_line in self.context.source_string.lines() {
            target_line = source_line;
            remaining_lines -= 1;
            if remaining_lines == 0 {
                break;
            }
        }
        let column = if current_position.column > 0 {current_position.column - 1} else { 0 };
        Err(format!("\n{}\n{}^\n{}", target_line, " ".repeat(column as usize), message))
    }

    fn exponent(&self, tokens: &mut Vec<Token>) -> Result<bool, String> {
        match self.context.get_current_character_class() {
            CharacterClass::Numeric => {
                return self.exponent_value(tokens);
            },
            CharacterClass::Minus => {
                self.context.next();
                self.context.set_exponent_sign(-1);
                return self.exponent_value(tokens);
            },
            CharacterClass::Plus => {
                self.context.next();
                return self.exponent_value(tokens);
            },
            _ => {
                return self.error("Invalid exponent, expected a sign or a number");
            },
        }
    }

    fn exponent_value(&self, tokens: &mut Vec<Token>) -> Result<bool, String> {
        match self.context.get_current_character_class() {
            CharacterClass::Numeric => {
                self.context.add_exponential_digit()?;
                self.context.next();
            },
            _ => {
                return self.error("Invalid exponent value, expected a number");
            },
        }

        loop {
            match self.context.get_current_character_class() {
                CharacterClass::Numeric => {
                    self.context.add_exponential_digit()?;
                    self.context.next();
                },
                _ => {
                    self.push_float_token(tokens);
                    return Ok(true);
                },
            }
        }
    }

    fn fraction(&self, tokens: &mut Vec<Token>) -> Result<bool, String> {
        loop {
            match self.context.get_current_character_class() {
                CharacterClass::Alpha => {
                    match self.context.get_current_character() {
                        'd' | 'D' => {
                            self.context.set_is_double();
                            self.context.next();
                            return self.exponent(tokens);
                        },
                        'e' | 'E' => {
                            self.context.next();
                            return self.exponent(tokens);
                        },
                        _ => {
                            self.push_float_token(tokens);
                            return Ok(true);
                        },
                    }
                }
                CharacterClass::Numeric => {
                    self.context.add_fractional_digit();
                    self.context.next();
                },
                _ => {
                    self.push_float_token(tokens);
                    return Ok(true);
                },
            }
        }

    }

    fn identifier(&self, tokens: &mut Vec<Token>) -> Result<bool, String> {
        loop {
            match self.context.get_current_character_class() {
                CharacterClass::Alpha | CharacterClass::Numeric => {
                    self.context.next();
                },
                CharacterClass::Dot => {
                    match self.context.peek() {
                        CharacterClass::Alpha | CharacterClass::Numeric => {
                            self.context.next();
                        },
                        _ => {
                            tokens.push(Token { 
                                token_type: TokenType::Identifier(self.context.get_lexeme_source()), 
                                string_value: self.context.get_lexeme_source(),
                                starting_position: self.context.get_starting_position() });
                            return Ok(true);
                        },
                    }
                },
                _ => {
                    tokens.push(Token { 
                        token_type: TokenType::Identifier(self.context.get_lexeme_source()), 
                        string_value: self.context.get_lexeme_source(),
                        starting_position: self.context.get_starting_position() });
                    return Ok(true);
                },
            }
        }
    }

    fn integer(&self, tokens: &mut Vec<Token>) -> Result<bool, String> {
        self.context.clear_number();
        loop {
            match self.context.get_current_character_class() {
                CharacterClass::Alpha => {
                    match self.context.get_current_character() {
                        'd' | 'D' => {
                            self.context.set_is_double();
                            self.context.next();
                            return self.exponent(tokens);
                        },
                        'e' | 'E' => {
                            self.context.next();
                            return self.exponent(tokens);
                        },
                        _ => {
                            tokens.push(Token { 
                                token_type: TokenType::Integer(self.context.get_integer_value() as i32), 
                                string_value: self.context.get_lexeme_source(),
                                starting_position: self.context.get_starting_position() });
                            return Ok(true);
                        },
                    }
                }
                CharacterClass::Numeric => {
                    self.context.add_integer_digit()?;
                    self.context.next();
                },
                CharacterClass::Dot => {
                    self.context.next();
                    return self.fraction(tokens);
                },
                _ => {
                    if self.context.get_integer_value() > i32::MAX as i64 || self.context.get_integer_value() < i32::MIN as i64 {
                        return Err(format!("integer overflow"));
                    }
                    tokens.push(Token { 
                        token_type: TokenType::Integer(self.context.get_integer_value() as i32), 
                        string_value: self.context.get_lexeme_source(),
                        starting_position: self.context.get_starting_position() });
                    return Ok(true);
                },
            }
        }
    }

    fn newline(&self, tokens: &mut Vec<Token>) -> Result<bool, String> { 
        self.context.next();
        let p = self.context.get_lexeme_source();
        tokens.push(Token { 
            token_type: TokenType::EOL, 
            string_value: p.clone(),
            starting_position: self.context.get_starting_position() });
        return Ok(true);
    }

    fn operator(&self, tokens: &mut Vec<Token>) -> Result<bool, String> {

        //  The current character is an operator. If it's composable (in the set of operator characters that can be used to form
        //  custom user-defined operators), then we check until we find a non-composable character.  Then we look at the set of known
        //  operators to decide which of these are user-defined and which are stand-alone. Once we make our decision, we back up the scanner to continue
        //  with the next character.

        let mut positions = vec![self.context.get_starting_position()];
        loop {
            match self.context.get_current_character_class() {
                CharacterClass::Dollar | CharacterClass::Minus | CharacterClass::Plus | CharacterClass::BackSlash | 
                CharacterClass::ComposableOperator => {
                    self.context.next();
                    positions.push(self.context.get_current_position());
                    continue;
                },
                CharacterClass::Dot => {
                    self.context.next();
                    positions.push(self.context.get_current_position());
                    if self.context.get_lexeme_source().as_str() == "..." {
                        return self.comment(tokens);
                    }
                    continue;
                },
                _ => break, 
            }        
        }

        let candidate_operator_string = self.context.get_lexeme_source();
        let mut candidate_operator = candidate_operator_string.as_str();
        while candidate_operator.len() > 1 && self.context.workspace.try_get_function(&candidate_operator).is_none() {
            candidate_operator = &candidate_operator[0..candidate_operator.len()-1];
            positions.pop();
        }

        tokens.push(Token { 
            token_type: TokenType::Operator(String::from(candidate_operator)), 
            string_value: String::from(candidate_operator),
            starting_position: self.context.get_starting_position() });
        self.context.set_iterator_to_nth(positions.pop().unwrap().n);
        Ok(true)
    }

    fn post_process_numeric_signs(&self, tokens: &Vec<Token>) -> Vec<Token>{

        //  Scan the tokens for the + and - operators. When applied monadically to a number, remove the operator and merge the sign into the number

        let mut processed_tokens = Vec::new();
        let mut penultimate_token_class = PostProcessTokenClass::OperatorOrPunctuation;
        let mut ultimate_token_class = PostProcessTokenClass::OperatorOrPunctuation;

        for token in tokens {

            //  First, classify the current token

            let current_token_class: PostProcessTokenClass;
            match token.token_type {
                TokenType::Real(_) | TokenType::Double(_) | TokenType::Integer(_)=> current_token_class = PostProcessTokenClass::Number,
                TokenType::Operator(ref op) => {
                    match op.as_str() {
                        "-" => current_token_class = PostProcessTokenClass::Minus,
                        "+" => current_token_class = PostProcessTokenClass::Plus,
                        _ => current_token_class = PostProcessTokenClass::OperatorOrPunctuation,
                    }
                },
                TokenType::Punctuation(ref p) => {
                    match p.as_str() {
                        "(" | "[" | "," | ":" | ";" => current_token_class = PostProcessTokenClass::OperatorOrPunctuation,
                        _ => current_token_class = PostProcessTokenClass::Other,
                    }
                },
                _ => current_token_class = PostProcessTokenClass::Other,
            }

            //  Now see if we can performa a merge

            match (&penultimate_token_class, &ultimate_token_class, &current_token_class) {
                (PostProcessTokenClass::OperatorOrPunctuation, PostProcessTokenClass::Minus, PostProcessTokenClass::Number) => {
                    match &token.token_type {
                        TokenType::Real(float) => {
                            processed_tokens.pop();
                            processed_tokens.push(Token { 
                                token_type: TokenType::Real(-float), 
                                string_value: (format!("-{}", &token.string_value)),
                                starting_position: token.starting_position.clone() });
                        },
                        TokenType::Double(float) => {
                            processed_tokens.pop();
                            processed_tokens.push(Token { 
                                token_type: TokenType::Double(-float), 
                                string_value: (format!("-{}", &token.string_value)),
                                starting_position: token.starting_position.clone() });
                        },
                        TokenType::Integer(int) => {
                            processed_tokens.pop();
                            processed_tokens.push(Token { 
                                token_type: TokenType::Integer(-int), 
                                string_value: (format!("-{}", &token.string_value)),
                                starting_position: token.starting_position.clone() });
                        },
                        _ => processed_tokens.push(token.clone()),
                    }
                    penultimate_token_class = PostProcessTokenClass::Other;
                    ultimate_token_class = PostProcessTokenClass::Number;
                },
                (PostProcessTokenClass::OperatorOrPunctuation, PostProcessTokenClass::Plus, PostProcessTokenClass::Number) => {
                    processed_tokens.pop();
                    processed_tokens.push(token.clone());
                    penultimate_token_class = PostProcessTokenClass::Other;
                    ultimate_token_class = PostProcessTokenClass::Number;
                }
                (_, _, _) => {
                    processed_tokens.push(token.clone());
                    penultimate_token_class = ultimate_token_class;
                    ultimate_token_class = current_token_class;
                },
            }
        }

        processed_tokens
    }

    fn post_process_strings(&self, tokens: &Vec<Token>) -> Vec<Token>{

        //  Scan the tokens for strings, and replace the escapes with the actual character

        let mut processed_tokens = Vec::new();

        for token in tokens {
            match &token.token_type {
                TokenType::Character(_) => {
                    processed_tokens.push(Token { 
                        token_type: TokenType::Character(convert_escape_sequence(token.string_value.as_str())), 
                        string_value: convert_escape_sequences(token.string_value.as_str()), 
                        starting_position: token.starting_position.clone() });
                },
                TokenType::String(s) => {
                    processed_tokens.push(Token { 
                        token_type: TokenType::String(convert_escape_sequences(s.as_str())), 
                        string_value: convert_escape_sequences(token.string_value.as_str()), 
                        starting_position: token.starting_position.clone() });
                },
                _ => processed_tokens.push(token.clone()),
            }
        }

        processed_tokens
    }

    fn punctuation(&self, tokens: &mut Vec<Token>) -> Result<bool, String> {
        self.context.next();
        let p = self.context.get_lexeme_source();
        tokens.push(Token { 
            token_type: TokenType::Punctuation(p.clone()), 
            string_value: p.clone(),
            starting_position: self.context.get_starting_position() });
        return Ok(true);
    }

    fn push_float_token(&self, tokens: &mut Vec<Token>) {
        if self.context.is_double() {
            tokens.push(Token { 
                token_type: TokenType::Double(self.context.get_float_value()), 
                string_value: self.context.get_lexeme_source(),
                starting_position: self.context.get_starting_position() });
        } else {
            tokens.push(Token { 
                token_type: TokenType::Real(self.context.get_float_value() as f32), 
                string_value: self.context.get_lexeme_source(),
                starting_position: self.context.get_starting_position() });
        }
    }

    fn string(&self, tokens: &mut Vec<Token>) -> Result<bool, String> {
        loop {
            self.context.literal_next();
            match self.context.get_current_character_class() {
                CharacterClass::DoubleQuote => {
                    self.context.literal_next();
                    match self.context.get_current_character_class() {
                        CharacterClass::DoubleQuote => {
                            continue;
                        },
                        _ => {
                            let s = self.context.get_lexeme_source();
                            let trimmed_s = &s[1..s.len() - 1];
                            tokens.push(Token { 
                                token_type: TokenType::String(trimmed_s.replace(r#""""#, r#"""#)), 
                                string_value: self.context.get_lexeme_source(),
                                starting_position: self.context.get_starting_position() });
                            return Ok(true);
                        },
                    }
                },
                CharacterClass::Eos => return self.error(format!("Expected double-quote").as_str()),
                _ => {

                },
            }
        }
    }

    fn start(&self, tokens: &mut Vec<Token>) -> Result<bool, String> {
        loop {
            self.context.set_lexeme_start();
            match self.context.get_current_character_class() {
                CharacterClass::Alpha => self.identifier(tokens)?,
                CharacterClass::Numeric => self.integer(tokens)?,
                CharacterClass::Dot => self.dot(tokens)?,
                CharacterClass::SingleQuote => self.character(tokens)?,
                CharacterClass::DoubleQuote => self.string(tokens)?,
                CharacterClass::Punctuation | CharacterClass::Other => self.punctuation(tokens)?,
                CharacterClass::Dollar | CharacterClass::Minus | CharacterClass::Plus | CharacterClass::BackSlash | 
                CharacterClass::ComposableOperator => self.operator(tokens)?,
                CharacterClass::Newline => self.newline(tokens)?,
                CharacterClass::WhiteSpace => {
                    self.context.next();
                    continue;
                },
                CharacterClass::Eos => {
                    tokens.push(Token { 
                        token_type: TokenType::EOS, 
                        string_value: String::new(),
                        starting_position: self.context.get_starting_position() });
                    return Ok(true);
                },
                _ => panic!("Unexpected CharacterClass"),
            };
        }
    }

    pub fn tokenize(input_string: &'a str, workspace: &'b WorkSpace) -> Result<Vec<Token>, String> {

        let context = Context::new(input_string, workspace);
        let lexer = Lexer { context: &context };
        let mut tokens = Vec::new();

        lexer.start(&mut tokens)?;
        if workspace.features.borrow().is_set(&crate::workspace::optional_features::Feature::StringEscapes) {
            tokens = lexer.post_process_strings(&tokens);
        }
        tokens = lexer.post_process_numeric_signs(&tokens);
        Ok(tokens)
    }
}

fn sanitize_for_display(s: &str) -> String {
    s.replace("\r", "\\r").replace("\n", "\\n").replace("\t", "\\t")
}