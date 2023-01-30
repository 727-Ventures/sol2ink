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

const INK_VERSION: &str = "~3.4.0";
const SOL_2_INK_VERSION: &str = "2.0.0-beta";
// const OPENBRUSH_VERSION: &str = "2.3.0";

pub fn generate_cargo_toml(package_name: &str, mod_name: Option<String>) -> String {
    let mut out = String::new();

    out.push_str("[package]\n");
    out.push_str("name = \"");
    out.push_str(package_name);
    out.push_str("\"\n");
    out.push_str("version = \"");
    out.push_str(SOL_2_INK_VERSION);
    out.push_str("\"\n");
    out.push_str("edition = \"2021\"\n");
    out.push_str("authors = [\"Sol2Ink\"]\n");
    out.push('\n');
    out.push_str("[dependencies]\n");
    out.push_str(generate_ink_dependency("ink_primitives", false, false).as_str());
    out.push_str(generate_ink_dependency("ink_metadata", true, true).as_str());
    out.push_str(generate_ink_dependency("ink_env", false, false).as_str());
    out.push_str(generate_ink_dependency("ink_storage", false, false).as_str());
    out.push_str(generate_ink_dependency("ink_lang", false, false).as_str());
    out.push_str(generate_ink_dependency("ink_prelude", false, false).as_str());
    out.push_str(generate_ink_dependency("ink_engine", false, true).as_str());
    out.push_str("scale = { package = \"parity-scale-codec\", version = \"3\", default-features = false, features = [\"derive\"] }\n");
    out.push_str("scale-info = { version = \"2\", default-features = false, features = [\"derive\"], optional = true }\n");
    // TODO: uncomment when openbrush on crates.io
    // out.push_str("openbrush = { version = \"");
    // out.push_str(OPENBRUSH_VERSION);
    // out.push_str("\", default-features = false }\n");
    out.push_str(r#"openbrush = { git = "https://github.com/Supercolony-net/openbrush-contracts", tag = "v2.3.0", default-features = false, features = [] }"#);
    out.push('\n');

    if let Some(mod_name) = mod_name.clone() {
        out.push_str(mod_name.as_str());
        out.push_str(" = { path = \"../../src\", default-features = false }\n");
    }

    out.push('\n');
    out.push_str("[lib]\n");
    out.push_str("name = \"");
    out.push_str(package_name);
    out.push_str("\"\n");
    out.push_str("path = \"lib.rs\"\n");
    if mod_name.is_some() {
        out.push_str("crate-type = [\"cdylib\"]\n");
    }
    out.push('\n');
    out.push_str("[features]\n");
    out.push_str("default = [\"std\"]\n");
    out.push_str("std = [\n");
    out.push_str("\"ink_primitives/std\",\n");
    out.push_str("\"ink_metadata\",\n");
    out.push_str("\"ink_metadata/std\",\n");
    out.push_str("\"ink_env/std\",\n");
    out.push_str("\"ink_storage/std\",\n");
    out.push_str("\"ink_lang/std\",\n");
    out.push_str("\"scale/std\",\n");
    out.push_str("\"scale-info\",\n");
    out.push_str("\"scale-info/std\",\n");
    out.push_str("\"openbrush/std\",\n");
    if let Some(mod_name) = mod_name {
        out.push('"');
        out.push_str(mod_name.as_str());
        out.push_str("/std\"\n");
    }
    out.push_str("]\n");
    out.push('\n');

    out
}

fn generate_ink_dependency(crate_name: &str, derive: bool, optional: bool) -> String {
    let mut out = String::new();

    out.push_str(crate_name);
    out.push_str(" = { version = \"");
    out.push_str(INK_VERSION);
    out.push_str("\", default-features = false");

    if derive {
        out.push_str(", features = [\"derive\"]");
    }
    if optional {
        out.push_str(", optional = true");
    }

    out.push_str(" }\n");

    out
}
