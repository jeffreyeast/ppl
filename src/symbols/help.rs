//  This module holds the format logic for the symbols module

use crate::{execution::value::{Value, sequence::SequenceInstance}, workspace::{GeneralSymbol, WorkSpace}};

use super::{metadata::{FunctionArgumentList, FunctionDescription, FunctionImplementation, FunctionClass, FormalArgument, SelectorDescription, VariableDescription}, name::Name};




pub trait Help {
    fn help_text(&self, workspace: &WorkSpace) -> Option<String>;
    fn help_text_len(&self, workspace: &WorkSpace) -> usize;
    fn pretty_print(&self) -> String;
    fn show_help(&self, name: &str, workspace: &WorkSpace) -> Result<Value,String>;
}




impl Help for GeneralSymbol {
    fn show_help(&self, name: &str, workspace: &WorkSpace) -> Result<Value,String> {
        match self {
            GeneralSymbol::Datatype(d) => d.show_help (name, workspace),
            GeneralSymbol::Function(f) => f.show_help (name, workspace),
            GeneralSymbol::Selector(s) => s.show_help (name, workspace),
            GeneralSymbol::Variable(v) => v.show_help (name, workspace),
            GeneralSymbol::Unresolved(u) => Err(format!("No help available for {}", u))
        }
    }

    fn help_text(&self, workspace: &WorkSpace) -> Option<String> {
        match self {
            GeneralSymbol::Datatype(d) => d.help_text (workspace),
            GeneralSymbol::Function(f) => f.help_text (workspace),
            GeneralSymbol::Selector(s) => s.help_text (workspace),
            GeneralSymbol::Variable(v) => v.help_text (workspace),
            GeneralSymbol::Unresolved(_) => None
        }
    }

    fn help_text_len(&self, workspace: &WorkSpace) -> usize {
        match self {
            GeneralSymbol::Datatype(d) => d.help_text_len (workspace),
            GeneralSymbol::Function(f) => f.help_text_len (workspace),
            GeneralSymbol::Selector(s) => s.help_text_len (workspace),
            GeneralSymbol::Variable(v) => v.help_text_len (workspace),
            GeneralSymbol::Unresolved(_) => 0,
        }
    }

    fn pretty_print(&self) -> String {
        match self {
            GeneralSymbol::Datatype(d) => d.pretty_print(),
            GeneralSymbol::Function(f) => f.pretty_print(),
            GeneralSymbol::Selector(s) => s.pretty_print(),
            GeneralSymbol::Variable(v) => v.pretty_print(),
            GeneralSymbol::Unresolved(u) => u.as_string(),
        }
    }
}



impl FormalArgument {
    fn format(&self) -> String {
        let open_brace = r#"{"#;
        let close_brace = r#"}"#;
        format!("{}{}: {}{}", open_brace, self.name.as_string(), self.datatype.as_string().to_ascii_uppercase(), close_brace)
    }
}




impl Help for FunctionDescription {
    fn help_text(&self, _workspace: &WorkSpace) -> Option<String> {
        Some(self.help_text.clone())
    }

    fn help_text_len(&self, workspace: &WorkSpace) -> usize {
        match self.help_text(workspace) {
            Some(help_text) => help_text.len(),
            None => 0,
        }
    }

    fn pretty_print(&self) -> String {
        format!("{:?}", self)
    }

    fn show_help(&self, name: &str, _workspace: &WorkSpace) -> Result<Value,String> {
        Ok(self.format_help_text(name).unwrap())
    }
}

impl FunctionDescription {
    fn format_arguments(&self) -> String {
        let mut result: String;
        match self.arguments {
            FunctionArgumentList::Fixed(ref args) => {
                result = String::new();
                let mut separator = "";
                for arg in args {
                    result += format!("{}{}", separator, arg.format()).as_str();
                    separator = " , ";
                }
            },
            FunctionArgumentList::Varying(_) => result = format!("..."),
        }
        return result;
    }

    pub fn format_help_text(&self, function_name: &str) -> Result<Value,String> {
        let result:String;

        match &self.implementation_class {
            FunctionImplementation::System(ref s) => {
                match s {
                    FunctionClass::Nullary(_) => {
                        result = format!("{} -> {}\n\t- {}", function_name.to_ascii_uppercase(), self.format_return_value(), &self.help_text);
                    } ,
                    FunctionClass::NullValuedNullary(_) => {
                        result = format!("{}\n\t- {}", function_name.to_ascii_uppercase(), &self.help_text);
                    } ,
                    FunctionClass::Monadic(_) => {
                        if Name::is_name_a_legal_identifier(function_name) {
                            result = format!("{} {} ( {} )   - {}", self.format_return_value(), function_name.to_ascii_uppercase(), self.format_arguments(), &self.help_text);
                        } else {
                            result = format!("{} {} -> {}\n\t- {}", function_name, self.format_arguments(), self.format_return_value(), &self.help_text);
                        }
                    },
                    FunctionClass::NullValuedMonadic(_) => {
                        if Name::is_name_a_legal_identifier(function_name) {
                            result = format!("{} {} ( {} )   - {}", self.format_return_value(), function_name.to_ascii_uppercase(), self.format_arguments(), &self.help_text);
                        } else {
                            result = format!("{} {}\n\t- {}", function_name, self.format_arguments(), &self.help_text);
                        }
                    },
                    FunctionClass::Diadic(_) => {
                        if Name::is_name_a_legal_identifier(function_name) {
                            result = format!("{} {} ( {} )\n\t- {}", self.format_return_value(), function_name.to_ascii_uppercase(), self.format_arguments(), &self.help_text);
                        } else {
                            if let FunctionArgumentList::Fixed(ref args) = self.arguments {
                                result = format!("{} {} {} -> {}\n\t- {}", args[0].format(), function_name, args[1].format(), self.format_return_value(), &self.help_text);
                            } else {
                                return Err(format!("Inconsistent internal metadata"));
                            }
                        }
                    },
                    FunctionClass::NullValuedDiadic(_) => {
                        if Name::is_name_a_legal_identifier(function_name) {
                            result = format!("{} {} ( {} )\n\t- {}", self.format_return_value(), function_name.to_ascii_uppercase(), self.format_arguments(), &self.help_text);
                        } else {
                            if let FunctionArgumentList::Fixed(ref args) = self.arguments {
                                result = format!("{} {} {}\n\t- {}", args[0].format(), function_name, args[1].format(), &self.help_text);
                            } else {
                                return Err(format!("Inconsistent internal metadata"));
                            }
                        }
                    },
                    FunctionClass::NullValuedTriadic(_) => {
                        result = format!("{} {} ( {} )\n\t- {}", self.format_return_value(), function_name.to_ascii_uppercase(), self.format_arguments(), &self.help_text);
                    },
                    FunctionClass::Triadic(_) => {
                        result = format!("{} {} ( {} ) -> {}\n\t- {}", self.format_return_value(), function_name.to_ascii_uppercase(), self.format_arguments(), self.format_return_value(), &self.help_text);
                    },
                    FunctionClass::NullValuedVarying(_) => {
                        result = format!("{} {} ( ... )\n\t- {}", self.format_return_value(), function_name.to_ascii_uppercase(), &self.help_text);
                    },
                    FunctionClass::Varying(_) => {
                        result = format!("{} {} ( ... ) -> {}\n\t- {}", self.format_return_value(), function_name.to_ascii_uppercase(), self.format_return_value(), &self.help_text);
                    },
                }
            },
            FunctionImplementation::User(_) => todo!(),
        }

        Ok(SequenceInstance::construct_string_sequence(&result))
    }

    fn format_return_value(&self) -> String {
        match &self.return_value {
            Some(r) => r.as_string().to_ascii_uppercase(),
            None => String::from("")
        }
    }
}





impl Help for SelectorDescription {
    fn help_text(&self, _workspace: &WorkSpace) -> Option<String> {
        Some(String::from("selects a field in a structure"))
    }

    fn help_text_len(&self, workspace: &WorkSpace) -> usize {
        match self.help_text(workspace) {
            Some(help_text) => help_text.len(),
            None => 0,
        }
    }

    fn pretty_print(&self) -> String {
        format!("{:?}", self)
    }

    fn show_help(&self, name: &str, _workspace: &WorkSpace) -> Result<Value,String> {
        Ok(SequenceInstance::construct_string_sequence(&format!("Selects the {} field in a structure", name.to_ascii_uppercase())))
    }
}


impl Help for VariableDescription {
    fn pretty_print(&self) -> String {
        format!("{:?}", self)
    }

    fn help_text(&self, workspace: &WorkSpace) -> Option<String> {
        self.cell.borrow().as_ref_to_value().help_text(workspace)
    }

    fn help_text_len(&self, workspace: &WorkSpace) -> usize {
        match self.help_text(workspace) {
            Some(help_text) => help_text.len(),
            None => 0,
        }
    }

    fn show_help(&self, _name: &str, workspace: &WorkSpace) -> Result<Value,String> {
        match self.help_text(workspace) {
            Some(t) => Ok(SequenceInstance::construct_string_sequence(&t)),
            None => Err(format!("help not implemented for this type")),
        }
    }
}
