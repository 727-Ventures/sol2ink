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

use crate::toml_builder;
use convert_case::{
    Case::Snake,
    Casing,
};
use proc_macro2::TokenStream;
use rust_format::{
    Config,
    Formatter,
    PostProcess,
    RustFmt,
};
use std::{
    fs::{
        create_dir_all,
        File,
    },
    io::{
        prelude::*,
        BufReader,
    },
};

/// Reads the file to be transpiled and returns it as string
///
/// `path` the path to the file
pub fn read_file(path: &String) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

/// writes the output to file
///
/// `lines` the transpiled file in the form of vec of strings
/// each item in the vec represents a separate line in the output file
pub fn write_file(lines: TokenStream, file_name: Option<String>) -> std::io::Result<()> {
    let path = file_name.unwrap_or_else(|| String::from("output"));

    let mut file = File::create(path)?;
    let config = Config::new_str().post_proc(PostProcess::ReplaceMarkersAndDocBlocks);
    file.write_all(
        RustFmt::from_config(config)
            .format_tokens(lines)
            .unwrap()
            .as_bytes(),
    )?;

    Ok(())
}

pub fn write_files(
    contract: TokenStream,
    implementation: TokenStream,
    trait_definition: TokenStream,
    lib_definition: TokenStream,
    file_name: Option<String>,
    contract_name_raw: &String,
) -> std::io::Result<()> {
    let contract_name = contract_name_raw.to_case(Snake);
    let config = Config::new_str().post_proc(PostProcess::ReplaceMarkersAndDocBlocks);
    let rust_fmt = RustFmt::from_config(config);
    // contract
    let contract_path = format!(
        "{}/contract",
        file_name.clone().unwrap_or_else(|| String::from("output"))
    );
    create_dir_all(&contract_path)?;

    let mut contract_file = File::create(format!("{contract_path}/lib.rs"))?;
    contract_file.write_all(rust_fmt.format_tokens(contract).unwrap().as_bytes())?;

    let mut cargo_toml = File::create(format!("{contract_path}/Cargo.toml"))?;
    cargo_toml.write_all(
        toml_builder::generate_cargo_toml("contract", Some(contract_name.clone())).as_bytes(),
    )?;

    // impl
    let base_path = format!(
        "{}/{contract_name}",
        file_name.unwrap_or_else(|| String::from("output"))
    );
    create_dir_all(&base_path)?;

    let mut impl_file = File::create(format!("{base_path}/impls.rs"))?;
    impl_file.write_all(rust_fmt.format_tokens(implementation).unwrap().as_bytes())?;

    // trait
    let mut trait_file = File::create(format!("{base_path}/traits.rs"))?;
    trait_file.write_all(rust_fmt.format_tokens(trait_definition).unwrap().as_bytes())?;

    let mut lib_file = File::create(format!("{base_path}/lib.rs"))?;
    lib_file.write_all(rust_fmt.format_tokens(lib_definition).unwrap().as_bytes())?;

    let mut main_cargo_toml = File::create(format!("{base_path}/Cargo.toml"))?;
    main_cargo_toml
        .write_all(toml_builder::generate_cargo_toml(&contract_name, None).as_bytes())?;

    Ok(())
}
