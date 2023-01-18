// MIT License

// Copyright (c) 2022 Supercolony

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
#![feature(if_let_guard)]

extern crate core;

pub mod assembler;
pub mod cli;
pub mod file_utils;
pub mod parser2;
pub mod structures;
pub mod toml_builder;

use parser2::Parser;
use rbtree::RBTree;

use crate::{
    cli::{
        cli,
        CliInput,
    },
    parser2::{
        ParserError,
        ParserOutput,
    },
};
use std::{
    collections::HashMap,
    env,
    path::Path,
};

fn main() {
    let args = cli();
    env::set_var("RUST_BACKTRACE", "1");

    let files = args.files.unwrap_or_else(|| {
        eprintln!("No files provided");
        std::process::exit(1);
    });

    if files.is_empty() {
        eprintln!("No files provided");
        std::process::exit(1);
    }

    for file in files {
        match file {
            CliInput::SolidityFile(file) => {
                match run(&file) {
                    Ok(_) => {
                        println!("Successfully parsed {file}");
                    }
                    Err(err) => {
                        eprintln!("error: {err:?}");
                        std::process::exit(1);
                    }
                }
            }
            CliInput::Directory(dir) => {
                let files = Path::new(&dir).read_dir().unwrap();

                for file in files {
                    let file = file.unwrap();
                    let file = file.path();
                    let file = file.to_str().unwrap();

                    if file.ends_with(".sol") {
                        match run(&file.to_string()) {
                            Ok(_) => {
                                println!("Successfully parsed {file}");
                            }
                            Err(err) => {
                                eprintln!("error: {err:?}");
                                std::process::exit(1);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn run(path: &String) -> Result<(), ParserError> {
    let content = file_utils::read_file(path)?;
    let mut fields_map = HashMap::new();
    let mut rb_tree = RBTree::new();

    let mut parser = Parser::new(&mut fields_map, &mut rb_tree);

    let output = parser.parse_file(&content)?;

    for output in output {
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
            }
            ParserOutput::Interface(interface) => {
                let ink_trait = assembler::assemble_interface(interface);
                let file_name = path.replace(".sol", ".rs");
                file_utils::write_file(ink_trait, Some(file_name))?;
                println!("File saved!");
            }
            ParserOutput::Library(library) => {
                let ink_trait = assembler::assemble_library(library);
                let file_name = path.replace(".sol", ".rs");
                file_utils::write_file(ink_trait, Some(file_name))?;
                println!("File saved!");
            }
            _ => {}
        }
    }
    Ok(())
}
