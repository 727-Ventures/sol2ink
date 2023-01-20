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

use clap::{
    command,
    Parser,
};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum CliInput {
    SolidityFile(String),
    Directory(String),
}

/// Sol2Ink - tool to convert Solidity smart contracts to Ink! smart contracts
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to files or directories
    #[arg(value_parser = sol_file_parser)]
    pub(crate) files: Option<Vec<CliInput>>,
}

fn sol_file_parser(s: &str) -> Result<CliInput, String> {
    let result = s.to_string();

    if !Path::new(&result).exists() {
        return Err(format!("{result} does not exist"))
    }

    if result.ends_with(".sol") {
        Ok(CliInput::SolidityFile(result))
    } else if Path::new(&result).is_dir() {
        Ok(CliInput::Directory(result))
    } else {
        Err(format!("{result} is not a solidity file or directory"))
    }
}

pub fn cli() -> Args {
    Args::parse()
}
