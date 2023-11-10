//  This module contains the workspace file I/O routines

use std::{fs::File, rc::Rc, io::Write, io::{Error, BufRead, BufReader}, collections::HashSet, path::PathBuf};

use crate::{workspace::WorkSpace, 
    symbols::{SymbolTable, 
        help::Help, 
        metadata::{self, MetaDataType, MetaStructure, MetaSequence, VariableDescription, FunctionDescription, MetaAlternate}, datatype::RootDataType}, 
        execution::{value::{Value, sequence::SequenceInstance, structure::StructureInstance}, evaluate_internal}};

const  OPEN_BRACKET: &str = r#"{"#;
const  CLOSE_BRACKET: &str = r#"}"#;


fn normalize_filename(filename: &String) -> PathBuf {
    let mut path = PathBuf::from(filename);
    if path.extension().is_none() {
        path.set_extension("ppl");
    }
    path
}

pub fn read(filename: &String, workspace: &WorkSpace) -> Result<(),String> {
    let f = File::open(normalize_filename(filename)).map_err(|e| e.to_string())?;
    let reader = BufReader::new(f);
    let mut in_quoted_function_definition = false;
    let mut function_definition = String::new();

    for line in reader.lines() {
        match line {
            Ok(ref l) => match l.trim() {
                OPEN_BRACKET => {
                    if in_quoted_function_definition {
                        return Err(format!("Malformed file, encounted unexpected {}", OPEN_BRACKET));
                    } else {
                        in_quoted_function_definition = true;
                        function_definition.clear();
                    }
                },
                CLOSE_BRACKET => {
                    if in_quoted_function_definition {
                        in_quoted_function_definition = false;
                        evaluate_internal(function_definition.as_str(), workspace)?;
                    } else {
                        return Err(format!("Malformed file, encounted unexpected {}", CLOSE_BRACKET));
                    }
                },
                _ => {
                    if in_quoted_function_definition {
                        function_definition += line.unwrap().as_str();
                        function_definition += "\n";
                    } else {
                        evaluate_internal(line.unwrap().as_str(), workspace)?;
                    }
                },
            },
            
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(())
}

pub fn write(filename: &String, workspace: &WorkSpace) -> Result<(),String>{
    let mut f = File::create(normalize_filename(filename)).map_err(|e| e.to_string())?;
    write_datatypes(&mut f, workspace).map_err(|e| e.to_string())?;
    write_symbol_table(&mut f, &*workspace.variable_symbol_table.borrow(), write_variable).map_err(|e| format!("{}", e)).map_err(|e| e.to_string())?;
    write_symbol_table(&mut f, &*workspace.user_function_symbol_table.borrow(), write_function).map_err(|e| format!("{}", e)).map_err(|e| e.to_string())?;

    
    Ok(())
}

fn write_alternate(f: &mut File, alternate: &MetaAlternate) -> Result<(),Error> {
    writeln!(f, "{}", alternate)
}

fn write_datatype(f: &mut File, name: &String, processed_datatypes: &mut HashSet<String>, symbol_table: &SymbolTable<MetaDataType>) -> Result<(),Error> {
    if !processed_datatypes.contains(name) {
        let opt_datatype = symbol_table.try_get(name.as_str());
        if let Some(datatype) = opt_datatype {
            processed_datatypes.insert(name.clone());
            match &datatype.root_data_type() {
                RootDataType::Structure(structure) => write_structure_definition(f, structure),
                RootDataType::Sequence(seq) => write_sequence_definition(f, seq),
                RootDataType::Alternate(a) => write_alternate(f, a),
                _ => Ok(())
            }
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

fn write_datatypes(f: &mut File, workspace: &WorkSpace) -> Result<(),Error> {
    let mut datatype_names = workspace.datatype_symbol_table.borrow().get_all_names();
    datatype_names.sort();

    let mut processed_datatype_names = HashSet::new();
    for name in metadata::get_system_datatypes() {
        processed_datatype_names.insert(name);
    }

    for name in datatype_names {
        write_datatype(f, &name, &mut processed_datatype_names, &*workspace.datatype_symbol_table.borrow())?;
    }

    Ok(())
}

fn write_function(f: &mut File, _name: &String, func: &Rc<FunctionDescription>) -> Result<(),Error> {
    writeln!(f, "{}", OPEN_BRACKET)?;
    write!(f, "{}", func.as_source())?;
    writeln!(f, "{}", CLOSE_BRACKET)
}

fn write_sequence_body(f: &mut File, seq: &SequenceInstance) -> Result<(),Error> {
    write!(f, "{}(", seq.as_string())?;
    let mut separator = "";
    for cell in &*seq.as_values() {
        write!(f, "{}", separator)?;
        write_value(f, &*cell.borrow().as_ref_to_value())?;
        separator = ", ";
    }
    write!(f, ")")
}

fn write_sequence_definition(f: &mut File, seq: &MetaSequence) -> Result<(),Error> {
    write!(f, "{}", seq)
}

fn write_structure_body(f: &mut File, structure: &StructureInstance) -> Result<(),Error> {
    write!(f, "{}(", structure.as_string())?;
    let mut separator = "";
    for member in structure.as_values() {
        write!(f, "{}", separator)?;
        write_value(f, &member.as_value())?;
        separator = ", ";
    }
    write!(f, ")")
}

fn write_structure_definition(f: &mut File, structure: &MetaStructure) -> Result<(),Error> {
    write!(f, "{}", structure)
}

fn write_symbol_table<T: Help>(file: &mut File, symbol_table: &SymbolTable<T>, formatter: fn(&mut File, &String, &Rc<T>) -> Result<(),Error>) -> Result<(),Error> {
    let symbols = symbol_table.get_all();
    for (name, item) in &symbols {
        formatter(file, name, item)?;
    }
    Ok(())
}

fn write_variable(f: &mut File, name: &String, variable: &Rc<VariableDescription>) -> Result<(),Error> {
    match &*variable.cell.borrow().as_ref_to_value() {
        Value::Bool(v) => writeln!(f, "{}_bool({})", name, v),
        Value::Int(v) => writeln!(f, "{}_int({})", name, v),
        Value::Real(v) => writeln!(f, "{}_real({})", name, v),
        Value::Double(v) => writeln!(f, "{}_dbl({})", name, v),
        Value::Char(v) => writeln!(f, "{}_char('{})", name, v),
        Value::Structure(structure) => {
            write!(f, "{}_", name)?;
            write_structure_body(f, structure)?;
            writeln!(f, "")
        },
        Value::Sequence(seq) => {
            write!(f, "{}_", name)?;
            write_sequence_body(f, seq)?;
            writeln!(f, "")
        }
        _ => Ok(()),
    }
}

fn write_value(f: &mut File, v: &Value) -> Result<(),Error> {
    match v {
        Value::Bool(v) => write!(f, "bool({})", v),
        Value::Int(v) => write!(f, "int({})", v),
        Value::Real(v) => write!(f, "real({})", v),
        Value::Double(v) => write!(f, "dbl({})", v),
        Value::Char(v) => write!(f, "'{}", v),
        Value::Structure(structure) => write_structure_body(f, structure),
        Value::Sequence(seq) => write_sequence_body(f, seq),
        _ => Ok(()),
    }
}