#![feature(once_cell)]
#![feature(string_remove_matches)]
#![feature(exclusive_range_pattern)]

mod assembler;
mod file_utils;
mod formatter;
mod parser;
mod structures;
mod toml_builder;

use std::collections::{
    HashMap,
    HashSet,
};

use crate::parser::ParserError;

pub fn run(path: &String) -> Result<(), parser::ParserError> {
    // read the file
    let content = file_utils::read_file(path)?;
    let mut chars = content.chars();
    let mut imports = HashSet::new();
    let mut storage = HashMap::new();
    let mut functions = HashMap::new();
    let mut events = HashMap::new();
    let mut modifiers = HashMap::new();
    let mut structs = HashMap::new();

    let mut parser = parser::Parser::new(
        &mut chars,
        &mut imports,
        &mut storage,
        &mut functions,
        &mut events,
        &mut modifiers,
        &mut structs,
    );
    let output = parser.parse_file()?;
    match output {
        (None, None) | (Some(_), Some(_)) => Err(ParserError::FileCorrupted),
        (Some(contract), None) => {
            let ink_contract = assembler::assemble_contract(contract);
            let file_name = path.replace(".sol", "");
            file_utils::write_file(ink_contract, Some(file_name))?;
            println!("File saved!");
            Ok(())
        }
        (None, Some(interface)) => {
            let ink_trait = assembler::assemble_interface(interface);
            let file_name = path.replace(".sol", "");
            file_utils::write_file(ink_trait, Some(file_name))?;
            println!("File saved!");
            Ok(())
        }
    }
}
