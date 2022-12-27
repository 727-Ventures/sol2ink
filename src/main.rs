// MIT License

// Copyright (c) 2022 727.ventures

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![feature(once_cell)]
#![feature(string_remove_matches)]
#![feature(exclusive_range_pattern)]

pub mod assembler;
pub mod file_utils;
pub mod formatter;
pub mod parser;
pub mod structures;
pub mod toml_builder;

use crate::parser::ParserError;
use parser::ParserOutput;
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    env,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please pass name of the file as argument");
        return
    }

    std::process::exit(match run(&args[1]) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn run(path: &String) -> Result<(), parser::ParserError> {
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
        ParserOutput::Contract(contract) => {
            let ink_contract = assembler::assemble_contract(&contract);
            let implementation = assembler::assemble_impl(&contract);
            let trait_definition = assembler::assemble_trait(&contract);
            let lib_definition = assembler::assemble_lib();
            let file_name = path.replace(".sol", "");
            file_utils::write_files(
                ink_contract,
                implementation,
                trait_definition,
                lib_definition,
                Some(file_name),
                &contract.name,
            )?;
            println!("File saved!");
            Ok(())
        }
        ParserOutput::Interface(interface) => {
            let ink_trait = assembler::assemble_interface(interface);
            let file_name = path.replace(".sol", ".rs");
            file_utils::write_file(ink_trait, Some(file_name))?;
            println!("File saved!");
            Ok(())
        }
        ParserOutput::Library(library) => {
            let ink_trait = assembler::assemble_library(library);
            let file_name = path.replace(".sol", ".rs");
            file_utils::write_file(ink_trait, Some(file_name))?;
            println!("File saved!");
            Ok(())
        }
        _ => Err(ParserError::FileCorrupted),
    }
}
