//  This module parses the language and builds an execution tree
//
//   Expression grammar:
//
//      <expr> := <atom> | <func> | ( <expr> ) | <atom> <op> <expr>
//      <func> := <func_identifier> | <func_identifier> () | <func_identifier> ( <expr> [, <expr> ])
//      <atom> := <literal> | <var_identifier>

mod scanner;
mod statementbuilder;
pub mod tree;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec;

use crate::{execution::{runtime::executable::Executable,
                        value::{sequence::SequenceInstance, Value}},
            lexical::{Token, TokenType, TokenPosition, Lexer, LineNumber},
            symbols::{metadata::{FunctionDescription, FormalArgument, MetaDataTypeName, FunctionArgumentList, FunctionImplementation, FunctionBody, ArgumentMechanism},
                      name::Name},
            workspace::{debug::DebugOption, {WorkSpace, GeneralSymbol}}};

use self::statementbuilder::StatementBuilder;
use self::tree::{Node, OperationNode, ReferenceNode, DefinitionNode, DefinitionType, SequenceDefinition, StructureDefinition, StructureMemberDescription, IndexNode};
use self::scanner::TokenScanner;



pub struct Parser<'a> {
    workspace: &'a WorkSpace,
    source: &'a str,
    line_number_bias: RefCell<u32>,
}

impl<'a> Parser<'a> {

    fn construct_label_database(&self, executable: &Executable) -> Result<HashMap<String,LineNumber>,String> {
        let mut labels = HashMap::new();
        for statement_index in 1..=executable.get_statement_count() {
            let statement = executable.get_statement(statement_index).unwrap();
            if let Node::StatementLabel(ref label_node) = executable.get_node(statement.as_first_node_index()).as_ref() {
                let normalized_name = label_node.name.to_ascii_lowercase();
                if labels.contains_key(&normalized_name) {
                    return Err(format!("Statement label {} is duplicated", label_node.name));
                }
                labels.insert(normalized_name, statement.as_line_number());
            }
        }

        Ok(labels)
    }

    fn error<T>(&self, starting_position: &TokenPosition, message: &str) -> Result<T, String> {
        let source_lines = self.source.lines();
        let mut target_line = self.source;
        let mut remaining_lines = starting_position.line_number;
        for source_line in source_lines {
            target_line = source_line;
            remaining_lines -= 1;
            if remaining_lines == 0 {
                break;
            }
        }
        let column = if starting_position.column > 0 {starting_position.column - 1} else { 0 };
        Err(format!("\n{}\n{}^\n{}", target_line, " ".repeat(column as usize), message))
    }

    fn is_diadic_operator(&self, op: &str) -> bool {
        return self.is_n_ary_operator(op, 2)
    }

    fn is_monadic_operator(&self, op: &str) -> bool {
        return self.is_n_ary_operator(op, 1)
    }

    fn is_nullary_operator(&self, op: &str) -> bool {
        return self.is_n_ary_operator(op, 0)
    }

    fn is_n_ary_operator(&self, op: &str, n: usize) -> bool {
        let functions = self.workspace.try_get_functions(op);
        for f in &functions {
            match f {
                GeneralSymbol::Function(f) => {
                    match &f.arguments {
                        FunctionArgumentList::Fixed(args) => {
                            if args.len() == n {
                                return true;
                            }
                        },
                        FunctionArgumentList::Varying(_) => {},
                    }
                },
                _ => panic!("internal error"),
            }
        }
        false
    }

    pub fn new(source: &'a str, workspace: &'a WorkSpace) -> Parser<'a> {
        Parser { source: source, workspace: workspace, line_number_bias: RefCell::new(0), }
    }

    pub fn parse(&self, source: &str) -> Result<Rc<Executable>,String> {
        let mut executable = Executable::new(source);
        let tokens = Lexer::tokenize(source, self.workspace)?;
        let mut token_iterator = TokenScanner::new(&tokens);

        if self.workspace.debug_options.borrow().is_set(&DebugOption::Lex) {
            dbg!(&tokens);
        }

        self.parse_statement_block_without_braces(&mut token_iterator, &mut executable)?;

        Ok(Rc::new(executable))
    }

    fn parse_alternate_definition(&self, identifier_name: &String, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {

        //  Gather the list of alternate names

        let mut alternate_names = Vec::new();

        loop {
            let alternate_name = token_iterator.consume_identifier()?;
            alternate_names.push(statement_builder.add_node(Node::IdentifierByValue(ReferenceNode::from_string(&alternate_name))));

            if token_iterator.consume_operator("!").is_err() {
                return Ok(statement_builder.add_node(Node::Definition(DefinitionNode::from_string(identifier_name, DefinitionType::Alternate(alternate_names)))));
            }
        }
    }

    fn parse_atomic_value(&self, token: &Token, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        match token.token_type {
            TokenType::Character(c) => {
                Ok(statement_builder.add_node(Node::Value(Value::Char(c))))
            },
            TokenType::Real(v) => {
                Ok(statement_builder.add_node(Node::Value(Value::Real(v))))
            },
            TokenType::Double(v) => {
                Ok(statement_builder.add_node(Node::Value(Value::Double(v))))
            },
            TokenType::Integer(v) => {
                Ok(statement_builder.add_node(Node::Value(Value::Int(v as i32))))
            },
            TokenType::String(ref s) => {
                Ok(statement_builder.add_node(Node::Value(SequenceInstance::construct_string_sequence(s))))
            },
            _ => return self.error(&token.starting_position, format!("Unexpected symbol: {}", token.string_value).as_str()),
        }
    }

    fn parse_datatype_definition(&self, identifier_name: &String, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        if token_iterator.consume_punctuation("[").is_ok() {
            self.parse_structure_or_sequence_definition(identifier_name, token_iterator, statement_builder)
        } else {
            self.parse_alternate_definition(identifier_name, token_iterator, statement_builder)
        }
    }

    fn parse_diadic_operation(&self, token: &Token, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        let left = self.parse_indexed_value(token, token_iterator, statement_builder)?;

        if let Ok(operator_name) = token_iterator.consume_any_operator() {
            if self.is_diadic_operator(&operator_name) {
                let right = self.parse_expression(token_iterator, statement_builder)?;
                statement_builder.add_node(Node::Operation(OperationNode::from_string(&operator_name, vec![ left, right] )));
                Ok(left)
            } else {
                self.error(&token_iterator.peek().unwrap().starting_position, format!("{} is not a binary operator", operator_name).as_str())
            }
        } else {
            Ok(left)
        }
    }

    fn parse_definition(&self, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        let identifier_name = token_iterator.consume_identifier()?.clone();
        
        if token_iterator.consume_operator("=").is_ok() {
            return self.parse_datatype_definition(&identifier_name, token_iterator, statement_builder);
        } else {
            return self.parse_function_definition(&identifier_name, token_iterator, statement_builder);
        }
    }

    fn parse_expression(&self, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        let token = token_iterator.next().expect("internal error").clone();

        self.parse_diadic_operation(&token, token_iterator, statement_builder)
    }

    fn parse_field_reference(&self, value_index_position: usize, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        token_iterator.consume_operator(".")?;
        let member_name = token_iterator.consume_identifier()?;
        let member_name_index = statement_builder.add_node(Node::IdentifierByValue(ReferenceNode::from_string(&member_name)));
        statement_builder.add_node(Node::Index(IndexNode { value_position: value_index_position, index_position: member_name_index}));
        Ok(member_name_index)
    }

    fn parse_function_call(&self, identifier_name: &String, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        let mut argument_list = Vec::new();
        let token = token_iterator.peek().expect("internal error").clone();

        if let TokenType::Punctuation(ref p) = token.token_type {
            if p == ")" {
                token_iterator.next();
                return Ok(statement_builder.add_node(Node::Operation(OperationNode::from_string(&identifier_name, argument_list))));
            }
        }

        loop {
            argument_list.push(self.parse_expression(token_iterator, statement_builder)?);

            let token = token_iterator.peek().expect("internal error").clone();
            match token.token_type {
                TokenType::Punctuation(ref p) => {
                    match p.as_str() {
                        ")" => {
                            token_iterator.next();
                            let first_argument_index = argument_list[0];
                            statement_builder.add_node(Node::Operation(OperationNode::from_string(&identifier_name, argument_list)));
                            return Ok(first_argument_index);
                        },
                        "," => {
                            token_iterator.next();
                        },
                        _ => {
                            return self.error(&token.starting_position, String::from("invalid function argument").as_str());
                        }
                    }
                },
                TokenType::EOL => {
                    return self.error(&token.starting_position, String::from("missing close paren on function call").as_str());
                },
                _ => {
                    return self.error(&token.starting_position, String::from("invalid function argument").as_str());
                }
            }
        }
    }

    fn parse_function_definition(&self, identifier_name: &String, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        let mut formal_args = Vec::new();
        let mut local_variables = Vec::new();
        let mut function_executable = Executable::new(self.source);

        if token_iterator.consume_punctuation("(").is_ok() {
            if token_iterator.consume_punctuation(")").is_err() {
                loop {
                    let is_reference_parameter = token_iterator.consume_operator("$").is_ok();
                    let parameter_name = token_iterator.consume_identifier()?;
                    let datatype_name: MetaDataTypeName;
                    if token_iterator.consume_operator(":").is_ok() {
                        datatype_name = MetaDataTypeName::from_string(&token_iterator.consume_identifier()?);
                    } else {
                        datatype_name = MetaDataTypeName::from_str("general");
                    }
                    formal_args.push(FormalArgument { 
                        name: Name::from_string(&parameter_name), 
                        mechanism: if is_reference_parameter {ArgumentMechanism::ByReference } else { ArgumentMechanism::ByValue }, 
                        datatype: datatype_name });
                    if token_iterator.consume_punctuation(",").is_err() {
                        if token_iterator.consume_punctuation(")").is_ok() {
                            break;
                        }
                        return self.error(&token_iterator.peek().unwrap().starting_position, format!("Expected , or )").as_str());
                    }
                }
            }
        }

        if token_iterator.consume_punctuation(";").is_ok() {
            loop {
                local_variables.push(Name::from_string(&token_iterator.consume_identifier()?));
                if token_iterator.consume_punctuation(",").is_err() {
                    break;
                }
            }
        }

        if token_iterator.consume_newline().is_err() {
            return self.error(&token_iterator.peek().unwrap().starting_position, "expected function body");
        }

        *self.line_number_bias.borrow_mut() = token_iterator.peek().unwrap().starting_position.line_number - 1;
        self.parse_statement_block_without_braces(token_iterator, &mut function_executable)?;
        *self.line_number_bias.borrow_mut() = 0;

        //  If they didn't end the function with $, we append one

        if function_executable.get_function_return_line_number().is_none() {
            let end_position = token_iterator.get_position();
            let appended_line_number = (function_executable.get_statement_count() + 1) as LineNumber;
            let mut statement_builder = StatementBuilder::begin_statement(appended_line_number, end_position.index, &mut function_executable);
            statement_builder.add_node(Node::FunctionReturn);
            statement_builder.finish_statement(end_position.index);
            function_executable.set_function_return_line_number(appended_line_number);
        }

        let labels = self.construct_label_database(&function_executable)?;
        Ok(statement_builder.add_node(Node::Definition(DefinitionNode::from_string(identifier_name, DefinitionType::Function(FunctionDescription {
            name: Name::from_string(identifier_name),
            arguments: FunctionArgumentList::Fixed(formal_args),
            local_variables: if local_variables.len() == 0 { None } else { Some(local_variables)},
            return_value: Some(MetaDataTypeName::from_str("general")),
            implementation_class: FunctionImplementation::User(Rc::new(FunctionBody { executable: Rc::new(function_executable), labels: labels})),
            help_text: String::from("")
        })))))
    }

    fn parse_identifier(&self, token: &Token, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        if let TokenType::Operator(ref op) = token.token_type {
            if op.as_str() == "$" { 

                //  This is an explicit by-reference identifier

                let identifier_name = token_iterator.consume_identifier()?;
                return Ok(statement_builder.add_node(Node::IdentifierByReference(ReferenceNode::from_string(&identifier_name))));
            }
        }

        let identifier_name = token.string_value.clone();
        if token_iterator.consume_punctuation("(").is_ok() {
            //  It's a function call
            return self.parse_function_call(&identifier_name, token_iterator, statement_builder);
        }

        //  And generate the identifier reference. It could be any of the following:
        //      - A variable value lookup
        //      - An assignment to a variable
        //      - A parameterless function call

        Ok(statement_builder.add_node(Node::IdentifierByValue(ReferenceNode::from_string(&token.string_value))))
    }

    fn parse_if(&self, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {

        //  The IF keyword has already been scanned

        let bool_expression_index = self.parse_expression(token_iterator, statement_builder)?;
        let condition_index = statement_builder.add_node(Node::Operation(OperationNode::from_str("-", vec![bool_expression_index] )));
        let condition_dispatch_destination_index = statement_builder.add_node(Node::Value(Value::Int(0)));
        statement_builder.add_node(Node::Operation(OperationNode::from_str("cbranch", vec![condition_index, condition_dispatch_destination_index])));
        statement_builder.add_node(Node::StatementEnd(0));

        token_iterator.consume_newline()?;
        self.parse_statement(token_iterator, statement_builder.as_executable())?;

        if token_iterator.consume_keyword("else").is_ok() {
            token_iterator.consume_newline()?;
            let true_branch_done_destination_index = statement_builder.add_node(Node::Value(Value::Int(0)));
            statement_builder.add_node(Node::Operation(OperationNode::from_str("branch", vec![true_branch_done_destination_index])));
            statement_builder.add_node(Node::StatementEnd(0));
            let false_branch = self.parse_statement(token_iterator, statement_builder.as_executable())?;
            let next_node_index = statement_builder.as_executable().get_next_node_index() as i32;
            if let Some(index) = false_branch {
                statement_builder.replace_node(condition_dispatch_destination_index, Node::Value(Value::Int(index as i32)));
            } else {
                statement_builder.replace_node(condition_dispatch_destination_index, Node::Value(Value::Int(next_node_index)));
            }
            statement_builder.replace_node(true_branch_done_destination_index, Node::Value(Value::Int(next_node_index)));
        } else {
            let next_node_index = statement_builder.as_executable().get_next_node_index() as i32;
            statement_builder.replace_node(condition_dispatch_destination_index, Node::Value(Value::Int(next_node_index)));
        }
        Ok(bool_expression_index)
    }

    fn parse_index(&self, value_index_position: usize, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        //  The leading [ has not been consumed yet

        let mut value_position = value_index_position;
        while token_iterator.consume_punctuation("[").is_ok() {
            loop {
                let index_position = self.parse_expression(token_iterator, statement_builder)?;
                value_position = statement_builder.add_node(Node::Index(IndexNode { value_position: value_position, index_position: index_position }));
                if token_iterator.consume_punctuation(",").is_err() {
                    break;
                }
            }
            token_iterator.consume_punctuation("]")?;
        }

        return Ok(value_index_position)
    }

    fn parse_indexed_value(&self, token: &Token, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        let mut value_index_position = self.parse_value(token, token_iterator, statement_builder)?;
        let first_value_index_position = value_index_position;
        loop {
            if token_iterator.peek_operator(".") {
                value_index_position = self.parse_field_reference(value_index_position, token_iterator, statement_builder)?;
            } else if token_iterator.peek_punctuation("[") {
                value_index_position = self.parse_index(value_index_position, token_iterator, statement_builder)?;
            } else {
                return Ok(first_value_index_position)
            }
        }
    }

    fn parse_monadic_operator(&self, token: &Token, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        if self.is_monadic_operator(token.string_value.as_str()) {
            let argument_index = self.parse_expression(token_iterator, statement_builder)?;
            return Ok(statement_builder.add_node(Node::Operation( OperationNode::from_string(&token.string_value, vec![ argument_index ]))))
        }
        self.parse_nullary_operator(&token, statement_builder)
    }

    fn parse_nullary_operator(&self, token: &Token, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        if let TokenType::Operator(operator_string_value) = &token.token_type {
            if self.is_nullary_operator(operator_string_value.as_str()) {
                Ok(statement_builder.add_node(Node::Operation( OperationNode::from_string(&operator_string_value, vec![ ]))))
            } else {
                self.error(&token.starting_position, format!("{} is not a nullary operator", operator_string_value).as_str())
            }
        } else {
            self.error(&token.starting_position, format!("Expected an operator").as_str())
        }
    }

    fn parse_parenthetical_expression<'b>(&self, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        let root_index = self.parse_expression(token_iterator, statement_builder)?;
        if token_iterator.consume_punctuation(")").is_err() {
            return self.error(&token_iterator.peek().unwrap().starting_position, format!("missing close paren").as_str());
        }
        Ok(root_index)
    }

    fn parse_sequence_definition(&self, identifier_name: &String, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        let lower_bound = token_iterator.consume_integer()?;
        token_iterator.consume_operator(":")?;

        let upper_bound: Option<i32>;
        let opt_upper_bound = token_iterator.consume_integer();
        match opt_upper_bound {
            Ok(i) => upper_bound = Some(i),
            Err(_) => upper_bound = None,
        }

        token_iterator.consume_punctuation("]")?;
        let datatype_name = token_iterator.consume_identifier()?;

        Ok(statement_builder.add_node(Node::Definition(DefinitionNode::from_string(identifier_name, DefinitionType::Sequence(SequenceDefinition::from_string(&datatype_name, lower_bound, upper_bound))))))
    }

    fn parse_statement(&self, token_iterator: &mut TokenScanner, executable: &mut Executable) -> Result<Option<usize>,String> {
        let statement_starting_position = &token_iterator.peek().unwrap().starting_position;
        let mut statement_builder = StatementBuilder::begin_statement(statement_starting_position.line_number - *self.line_number_bias.borrow(), 
            statement_starting_position.index, executable);
        match self.parse_statement_internal(statement_builder.as_starting_line_number(), token_iterator, &mut statement_builder)? {
            None => Ok(None),
            Some(statement_root_index) => {
                statement_builder.add_node(Node::StatementEnd(statement_root_index));
                let statement_ending_position = &token_iterator.peek().unwrap().starting_position;
        
                statement_builder.finish_statement(statement_ending_position.index);
        
                Ok(Some(statement_root_index))
            }
        }
    }

    fn parse_statement_internal(&self, normalized_line_number: LineNumber, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<Option<usize>,String> {
        if token_iterator.consume_newline().is_ok() || token_iterator.is_eos() {

            //  We've got an empty line

            Ok(None)

        } else if token_iterator.consume_operator("$").is_ok() {

            if token_iterator.consume_newline().is_ok() || token_iterator.is_eos() {

                //  We're at the end of the function definition ( final $)

                if !token_iterator.is_eos() {
                    return self.error(&token_iterator.get_position(), "expected end-of-string");
                }
                let return_node_index = statement_builder.add_node(Node::FunctionReturn);
                statement_builder.as_executable().set_function_return_line_number(normalized_line_number);
                Ok(Some(return_node_index))

            } else {

                //  We're at the start of a type or function definition

                let result = self.parse_definition(token_iterator, statement_builder)?;
                if token_iterator.consume_newline().is_err() && !token_iterator.is_eos() {
                    return self.error(&token_iterator.peek().unwrap().starting_position, "expected newline");
                }
                Ok(Some(result))
            }
        } else if token_iterator.consume_punctuation("{").is_ok() {

            //  We've got the start of a statement block

            self.parse_statement_block(token_iterator, statement_builder)

        } else if token_iterator.consume_keyword("if").is_ok(){ 

            //  IF statement

            Ok(Some(self.parse_if(token_iterator, statement_builder)?))

        } else if token_iterator.consume_keyword("while").is_ok(){ 

            //  WHILE statement

            Ok(Some(self.parse_while(token_iterator, statement_builder)?))

        } else {

            //  This is an expression

            let label_index_result = self.parse_statement_label(token_iterator, statement_builder);
            let expression_index = self.parse_expression(token_iterator, statement_builder)?;
            if token_iterator.consume_newline().is_err() && !token_iterator.is_eos() {
                return self.error(&token_iterator.peek().unwrap().starting_position, "expected newline");
            }
            if let Ok(label_index) =  label_index_result {
                Ok(Some(label_index))
            } else {
                Ok(Some(expression_index))
            }
        }
    }

    fn parse_statement_block(&self, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<Option<usize>,String> {

        //  We've already processed the opening brace

        token_iterator.consume_newline()?;
        let mut first_node_index = None;

        loop {
            match (first_node_index, self.parse_statement(token_iterator, statement_builder.as_executable())?) {
                (None, Some(index)) => first_node_index = Some(index),
                (_,_) => {},
            }
            if token_iterator.consume_punctuation("}").is_ok() {
                token_iterator.consume_newline()?;
                break;
            }
            if token_iterator.is_eos() {
                return self.error(&token_iterator.peek().unwrap().starting_position, format!("expected }}").as_str());
            }
        }

        Ok(first_node_index)
    }

    fn parse_statement_block_without_braces(&self, token_iterator: &mut TokenScanner, executable: &mut Executable) -> Result<Option<usize>,String> {
        let mut first_node_index = None;

        loop {
            match (first_node_index, self.parse_statement(token_iterator, executable)?) {
                (None, Some(index)) => first_node_index = Some(index),
                (_,_) => {},
            }
            if token_iterator.is_eos() {
                break;
            }
        }

        Ok(first_node_index)
    }

    fn parse_statement_label(&self, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        if self.workspace.features.borrow().is_set(&crate::workspace::optional_features::Feature::LineNames) {

            //  This requires looking ahead a couple of tokens, so we save the iterator in case we don't find a label

            token_iterator.push_iterator();

            let label = token_iterator.consume_identifier();
            if label.is_ok() {
                if token_iterator.consume_operator(":").is_ok() {
                    let label_index = statement_builder.add_node(Node::StatementLabel( tree::LabelNode { name: label.unwrap(), statement_index: 0 }));
                    token_iterator.discard_saved_iterator();
                    return Ok(label_index)
                }
            }

            //  No label

            token_iterator.pop_iterator();
        }

        Err(format!("Expected statement label"))
    }

    fn parse_structure_definition(&self, identifier_name: &String, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        let mut members = Vec::new();
        loop {
            members.push(self.parse_structure_member_definition(token_iterator)?);

            if token_iterator.consume_punctuation(",").is_ok() {
                continue;   
            }
            if token_iterator.consume_punctuation("]").is_ok() {
                return Ok(statement_builder.add_node(Node::Definition(DefinitionNode::from_string(identifier_name, 
                    DefinitionType::Structure (StructureDefinition { members: members })))));
            }
            return self.error(&token_iterator.peek().unwrap().starting_position, format!("syntax error in structure definition").as_str());
        }
    }

    fn parse_structure_member_definition(&self, token_iterator: &mut TokenScanner) -> Result<StructureMemberDescription,String> {
        let member_name = token_iterator.consume_identifier()?;
        token_iterator.consume_operator(":")?;
        let datatype_name = token_iterator.consume_identifier()?;
        Ok(StructureMemberDescription { name: Name::from_string(&member_name), datatype: Name::from_string(&datatype_name) })
    }

    fn parse_structure_or_sequence_definition(&self, identifier_name: &String, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        if token_iterator.peek_identifier() {
            self.parse_structure_definition(identifier_name, token_iterator, statement_builder)
        } else {
            self.parse_sequence_definition(identifier_name, token_iterator, statement_builder)
        }
    }

    fn parse_tuple_expression<'b>(&self, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        let mut members = Vec::new();

        if token_iterator.consume_punctuation("]").is_err() {
            loop {
                members.push(self.parse_expression(token_iterator, statement_builder)?);
                if token_iterator.consume_punctuation(",").is_err() {
                    if token_iterator.consume_punctuation("]").is_ok() {
                        break;
                    } else {
                        return self.error(&token_iterator.peek().unwrap().starting_position, format!("missing closing bracket").as_str());
                    }
                }
            }
        }

        if members.len() > 0 {
            let first_member_index = members[0];
            statement_builder.add_node(Node::Operation(OperationNode::from_str("tuple", members)));
            Ok(first_member_index)
        } else {
            Ok(statement_builder.add_node(Node::Operation(OperationNode::from_str("tuple", members))))
        }
    }

    fn parse_value(&self, token: &Token, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {
        match &token.token_type {
            TokenType::Identifier(_) => return self.parse_identifier(token, token_iterator, statement_builder),
            TokenType::Operator(ref op) => {
                match op.as_str() {
                    "$" => {
                        if token_iterator.peek_identifier() {
                            return self.parse_identifier(token, token_iterator, statement_builder);
                        }
                    },
                    _ => {},
                }
                return self.parse_monadic_operator(token, token_iterator, statement_builder);   
            },
            TokenType::Punctuation(ref p) => {
                match p.as_str() {
                    "(" => return self.parse_parenthetical_expression(token_iterator, statement_builder),
                    "[" => return self.parse_tuple_expression(token_iterator, statement_builder),
                    _ => return self.error(&token.starting_position, format!("Syntax error near {}", p).as_str()),
                }
            },
            _ => return self.parse_atomic_value(token, statement_builder),
        }
    }

    fn parse_while(&self, token_iterator: &mut TokenScanner, statement_builder: &mut StatementBuilder) -> Result<usize,String> {

        //  The WHILE keyword has already been scanned

        let bool_expression_index = self.parse_expression(token_iterator, statement_builder)?;
        let condition_index = statement_builder.add_node(Node::Operation(OperationNode::from_str("-", vec![bool_expression_index] )));
        let condition_dispatch_destination_index = statement_builder.add_node(Node::Value(Value::Int(0)));
        statement_builder.add_node(Node::Operation(OperationNode::from_str("cbranch", vec![condition_index, condition_dispatch_destination_index])));
        statement_builder.add_node(Node::StatementEnd(0));

        token_iterator.consume_newline()?;
        self.parse_statement(token_iterator, statement_builder.as_executable())?;
        statement_builder.add_node(Node::Value(Value::Int(bool_expression_index as i32)));
        statement_builder.add_node(Node::Operation(OperationNode::from_str("branch", vec![bool_expression_index])));
        statement_builder.add_node(Node::StatementEnd(0));

        let next_node_index = statement_builder.as_executable().get_next_node_index() as i32;
        statement_builder.replace_node(condition_dispatch_destination_index, Node::Value(Value::Int(next_node_index)));

        Ok(bool_expression_index)
    }
}