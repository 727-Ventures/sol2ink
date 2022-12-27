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

use crate::structures::*;
use convert_case::{
    Case::{
        Pascal,
        Snake,
        UpperSnake,
    },
    Casing,
};
use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::*;
use std::{
    collections::{
        HashSet,
        VecDeque,
    },
    str::FromStr,
};

// constant vector of rust keywords which are not keywords in solidity
const RUST_KEYWORDS: [&str; 27] = [
    "const", "crate", "extern", "fn", "impl", "in", "loop", "mod", "move", "mut", "pub", "ref",
    "self", "Self", "trait", "unsafe", "use", "where", "become", "box", "final", "priv", "unsized",
    "async", "await", "dyn", "union",
];

/// Assembles ink! contract from the parsed contract struct and return it as a vec of Strings
pub fn assemble_contract(contract: &Contract) -> TokenStream {
    let mod_name = format_ident!("{}", contract.name.to_case(Snake));
    let contract_name = format_ident!("{}Contract", contract.name);
    let trait_name = format_ident!("{}", contract.name);
    let signature = signature();
    let imports = assemble_imports(&contract.imports);
    let events = assemble_events(&contract.events);
    let storage = assemble_storage(&contract.name);
    let constructor = assemble_constructor(&contract.constructor, &contract.fields);
    let constants = assemble_constants(&contract.fields);
    let comments = assemble_contract_doc(&contract.contract_doc);
    let emit_functions = assemble_contract_emit_functions(&contract.events);

    let contract = quote! {
        #![cfg_attr(not(feature = "std"), no_std)]
        #![feature(min_specialization)]
        _blank_!();
        #signature
        #comments
        #[openbrush::contract]
        pub mod #mod_name {
            #imports
            use scale::Encode;
            use scale::Decode;
            use ink_storage::traits::SpreadAllocate;
            use openbrush::traits::Storage;
            use #mod_name::*;
            use ink_lang::codegen::Env;
            use ink_lang::codegen::EmitEvent;
            _blank_!();
            #constants
            #events
            #storage
            _blank_!();
            impl #trait_name for #contract_name {}
            _blank_!();
            impl #mod_name::Internal for #contract_name {
                #emit_functions
            }
            _blank_!();
            impl #contract_name {
                #constructor
            }
        }
    };

    contract
}

/// Assembles the implementation of the contract's trait
pub fn assemble_impl(contract: &Contract) -> TokenStream {
    let trait_name = format_ident!("{}", contract.name);
    let signature = signature();
    let imports = assemble_imports(&contract.imports);
    let data = assemble_data_struct(&contract.fields);
    let getters = assemble_getters(&contract.fields);
    let functions = assemble_functions(
        &contract
            .functions
            .iter()
            .filter(|f| f.header.external)
            .cloned()
            .collect(),
        false,
    );
    let internal_trait = assemble_function_headers(
        &contract
            .functions
            .iter()
            .filter(|f| !f.header.external)
            .map(|f| f.clone().header)
            .collect(),
    );
    let internal_functions = assemble_functions(
        &contract
            .functions
            .iter()
            .filter(|f| !f.header.external)
            .cloned()
            .collect(),
        false,
    );
    let (emit_function_headers, impl_emit_functions) = assemble_emit_functions(&contract.events);
    let modifiers = assemble_modifiers(&contract.modifiers, &trait_name);

    let contract = quote! {
        #signature
        pub use crate::{
            impls,
            traits::*,
        };
        #imports
        use openbrush::traits::Storage;
        _blank_!();
        #data
        _blank_!();
        #modifiers
        _blank_!();
        impl <T: Storage<Data>> #trait_name for T {
            #functions
            #getters
        }
        _blank_!();
        pub trait Internal {
            #internal_trait
            #emit_function_headers
        }
        _blank_!();
        impl<T: Storage<Data>> Internal for T {
            #internal_functions
            #impl_emit_functions
        }
    };

    contract
}

/// Assembles ink! trait of the provided contract
pub fn assemble_trait(contract: &Contract) -> TokenStream {
    let trait_name = TokenStream::from_str(&contract.name).unwrap();
    let ref_name = TokenStream::from_str(&format!("{}Ref", contract.name)).unwrap();
    let signature = signature();
    let imports = assemble_imports(&contract.imports);
    let enums = assemble_enums(&contract.enums);
    let structs = assemble_structs(&contract.structs);
    let getters_trait = assemble_getters_trait(&contract.fields);
    let function_headers = assemble_function_headers(
        &contract
            .functions
            .clone()
            .iter()
            .filter(|f| f.header.external)
            .map(|f| f.header.clone())
            .collect(),
    );

    quote! {
        #signature
        #imports
        use scale::{
            Decode,
            Encode,
        };
        _blank_!();
        #[derive(Debug, Encode, Decode, PartialEq, Eq)]
        #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
        pub enum Error {
            Custom(String),
        }
        _blank_!();
        #enums
        _blank_!();
        #structs
        _blank_!();
        #[openbrush::wrapper]
        pub type #ref_name = dyn #trait_name;
        _blank_!();
        #[openbrush::trait_definition]
        pub trait #trait_name {
            #function_headers
            #getters_trait
        }
    }
}

/// Assembles lib file
pub fn assemble_lib() -> TokenStream {
    quote! {
        #![cfg_attr(not(feature = "std"), no_std)]
        #![feature(min_specialization)]
        _blank_!();
        pub mod impls;
        pub mod traits;

        pub use impls::*;
        pub use impls::Data as Data;
    }
}

/// Assembles ink! interface(trait) from the parsed interface struct and return it as a vec of Strings
pub fn assemble_interface(interface: Interface) -> TokenStream {
    let interface_name = TokenStream::from_str(&interface.name).unwrap();
    let interface_name_ref = TokenStream::from_str(&format!("{}Ref", interface.name)).unwrap();
    let signature = signature();
    let imports = assemble_imports(&interface.imports);
    let events = assemble_events(&interface.events);
    let enums = assemble_enums(&interface.enums);
    let structs = assemble_structs(&interface.structs);
    let function_headers = assemble_function_headers(&interface.function_headers);

    let interface = quote! {
        #signature
        #imports
        _blank_!();
        #events
        #enums
        #structs
        #[openbrush::wrapper]
        pub type #interface_name_ref = dyn #interface_name;
        _blank_!();
        #[openbrush::trait_definition]
        pub trait #interface_name {
            #function_headers
        }
    };

    interface
}

/// Assembles a solidity library as a plain Rust file from the parsed library struct and return it as a TokenStream
pub fn assemble_library(library: Library) -> TokenStream {
    let signature = signature();
    let imports = assemble_imports(&library.imports);
    let events = assemble_events(&library.events);
    let enums = assemble_enums(&library.enums);
    let structs = assemble_structs(&library.structs);
    let constants = assemble_constants(&library.fields);
    let functions = assemble_functions(&library.functions, true);
    let comments = assemble_contract_doc(&library.libraray_doc);

    let library = quote! {
        #![cfg_attr(not(feature = "std"), no_std)]
        #![feature(min_specialization)]
        _blank_!();
        #signature
        #comments
        #imports
        _blank_!();
        pub enum Error {
            Custom(String),
        }
        _blank_!();

        #constants
        #events
        #enums
        #structs
        #functions
    };

    library
}

fn assemble_contract_doc(comments: &Vec<String>) -> TokenStream {
    let mut output = TokenStream::new();

    // assemble comments
    for comment in comments.iter() {
        output.extend(quote! {
            #[doc = #comment]
        });
    }

    output
}

/// Sorts the imports inside the HashSet and return it as a Vec of Strings
fn assemble_imports(imports: &HashSet<String>) -> TokenStream {
    let mut output = TokenStream::new();
    let output_vec = Vec::from_iter(imports);

    for import in output_vec {
        output.extend(TokenStream::from_str(&import).unwrap());
    }

    output
}

/// Assembles ink! enums from the vec of parsed Enum structs and return them as a vec of Strings
fn assemble_enums(enums: &Vec<Enum>) -> TokenStream {
    let mut output = TokenStream::new();

    for enumeration in enums.iter() {
        let enum_name = TokenStream::from_str(
            &check_expression_for_keywords(&enumeration.name).to_case(Pascal),
        )
        .unwrap();
        let mut enum_comments = TokenStream::new();
        let mut values = TokenStream::new();

        // assemble comments
        for comment in enumeration.comments.iter() {
            enum_comments.extend(quote! {
                #[doc = #comment]
            });
        }

        // assemble enum values
        for value in enumeration.values.iter() {
            let value_name = TokenStream::from_str(&value.name.to_case(Pascal)).unwrap();
            let mut value_comment = TokenStream::new();
            for comment in value.comments.iter() {
                value_comment.extend(quote! {
                    #[doc = #comment]
                })
            }

            values.extend(quote! {
                #value_comment
                #value_name,
            });
        }

        output.extend(quote! {
            #enum_comments
            pub enum #enum_name {
                #values
            }
            _blank_!();
        });
    }

    output
}

/// Assembles ink! events from the vec of parsed Event structs and return them as a vec of Strings
fn assemble_events(events: &Vec<Event>) -> TokenStream {
    let mut output = TokenStream::new();

    for event in events.iter() {
        let event_name = TokenStream::from_str(&event.name).unwrap();
        let mut event_comments = TokenStream::new();
        let mut event_fields = TokenStream::new();

        // assemble comments
        for comment in event.comments.iter() {
            event_comments.extend(quote! {
                #[doc = #comment]
            });
        }

        // assemble event fields
        for event_field in event.fields.iter() {
            let mut event_field_comments = TokenStream::new();
            for comment in event_field.comments.iter() {
                event_field_comments.extend(quote! {
                    #[doc = #comment]
                })
            }
            event_fields.extend(quote! {
                    #event_field_comments
            });

            if event_field.indexed {
                event_fields.extend(quote! {
                    #[ink(topic)]
                });
            }

            let event_field_name = format_ident!(
                "{}",
                check_expression_for_keywords(&event_field.name).to_case(Snake)
            );
            let event_field_type = TokenStream::from_str(&event_field.field_type).unwrap();

            event_fields.extend(quote! {
                #event_field_name: #event_field_type,
            });
        }

        output.extend(quote! {
            #event_comments
            #[ink(event)]
            pub struct #event_name
            {
                #event_fields
            }
            _blank_!();
        });
    }

    output
}

/// Assembles ink! storage struct from the vec of parsed ContractField structs and return it as a vec of Strings
fn assemble_data_struct(fields: &[ContractField]) -> TokenStream {
    let mut output = TokenStream::new();
    let mut storage_fields = TokenStream::new();

    // assemble storage fields
    for field in fields.iter().filter(|field| !field.constant) {
        let field_name = format_ident!("{}", field.name.to_case(Snake));
        let field_type = TokenStream::from_str(&field.field_type).unwrap();

        for comment in field.comments.iter() {
            storage_fields.extend(quote! {
                #[doc = #comment]
            });
        }
        storage_fields.extend(quote! {
            pub #field_name: #field_type,
        });
    }

    output.extend(quote! {
        pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);
        _blank_!();
        #[derive(Default, Debug)]
        #[openbrush::upgradeable_storage(STORAGE_KEY)]
        pub struct Data {
            #storage_fields
            pub _reserved: Option<()>,
        }
    });

    output
}

/// Assembles ink! storage struct from the vec of parsed ContractField structs and return it as a vec of Strings
fn assemble_getters(fields: &[ContractField]) -> TokenStream {
    let mut output = TokenStream::new();

    // assemble storage fields
    for field in fields
        .iter()
        .filter(|field| !field.constant && field.public)
    {
        let field_name = format_ident!("{}", field.name.to_case(Snake));
        let field_type = TokenStream::from_str(&field.field_type).unwrap();

        output.extend(quote! {
            fn #field_name(&self) -> #field_type {
                self.data().#field_name
            }
            _blank_!();
        });
    }

    output
}

/// Assembles ink! storage struct from the vec of parsed ContractField structs and return it as a vec of Strings
fn assemble_getters_trait(fields: &[ContractField]) -> TokenStream {
    let mut output = TokenStream::new();

    // assemble storage fields
    for field in fields.iter().filter(|field| !field.constant) {
        let field_name = format_ident!("{}", field.name.to_case(Snake));
        let field_type = TokenStream::from_str(&field.field_type).unwrap();

        output.extend(quote! {
            #[ink(message)]
            fn #field_name(&self) -> #field_type;
            _blank_!();
        });
    }

    output
}

/// Assembles ink! storage struct from the vec of parsed ContractField structs and return it as a vec of Strings
fn assemble_storage(contract_name: &String) -> TokenStream {
    let mut output = TokenStream::new();
    let contract_name = format_ident!("{}Contract", contract_name);

    output.extend(quote! {
        #[ink(storage)]
        #[derive(Default, SpreadAllocate, Storage)]
        pub struct #contract_name {
            #[storage_field]
            data: impls::Data,
        }
    });

    output
}

/// Assembles constant fields of the contract
fn assemble_constants(fields: &Vec<ContractField>) -> TokenStream {
    let mut output = TokenStream::new();

    // assemble storage fields
    for field in fields.iter().filter(|field| field.constant) {
        let field_name = format_ident!(
            "{}",
            check_expression_for_keywords(&field.name).to_case(UpperSnake)
        );
        let field_type = TokenStream::from_str(&field.field_type).unwrap();
        let initial_value = field.initial_value.clone().unwrap();

        for comment in field.comments.iter() {
            output.extend(quote! {
                #[doc = #comment]
            });
        }
        output.extend(quote! {
            pub const #field_name: #field_type = #initial_value;
        });
    }

    output.extend(quote! {
        _blank_!();
    });

    output
}

/// Assembles ink! structs from the vec of parsed Struct structs and return them as a vec of Strings
fn assemble_structs(structs: &Vec<Struct>) -> TokenStream {
    let mut output = TokenStream::new();

    for structure in structs.iter() {
        let struct_name =
            TokenStream::from_str(&check_expression_for_keywords(&structure.name)).unwrap();
        let mut struct_comments = TokenStream::new();
        let mut struct_fields = TokenStream::new();

        // assemble comments
        for comment in structure.comments.iter() {
            struct_comments.extend(quote! {
                #[doc = #comment]
            });
        }

        // assemble struct fields
        for struct_field in structure.fields.iter() {
            let mut struct_field_comments = TokenStream::new();
            for comment in struct_field.comments.iter() {
                struct_field_comments.extend(quote! {
                    #[doc = #comment]
                })
            }
            let struct_field_name = format_ident!(
                "{}",
                &check_expression_for_keywords(&struct_field.name).to_case(Snake)
            );

            let struct_field_type = TokenStream::from_str(&struct_field.field_type).unwrap();

            struct_fields.extend(quote! {
                #struct_field_comments
                #struct_field_name: #struct_field_type,
            });
        }

        output.extend(quote! {
            #struct_comments
            #[derive(Default, Encode, Decode)]
            #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
            pub struct #struct_name {
                #struct_fields
            }
        });

        output.extend(quote! {
            _blank_!();
        });
    }

    output
}

/// Assembles ink! cosntructor from the parsed Function struct and return it as a vec of Strings
fn assemble_constructor(constructor: &Function, fields: &[ContractField]) -> TokenStream {
    let mut output = TokenStream::new();
    let mut params = TokenStream::new();
    let mut comments = TokenStream::new();
    let constructor_functions = &constructor.body;

    // assemble comments
    for comment in constructor.header.comments.iter() {
        comments.extend(quote! {
            #[doc = #comment]
        });
    }

    // assemble params
    for param in constructor.header.params.iter() {
        let param_name = format_ident!("{}", param.name.to_case(Snake));
        let param_type = TokenStream::from_str(&param.param_type).unwrap();

        params.extend(quote! {
            #param_name: #param_type,
        });
    }

    let mut body = TokenStream::new();

    // assemble body
    body.extend(quote! {
        #(#constructor_functions)*
    });

    for field in fields
        .iter()
        .filter(|field| field.initial_value.is_some() && !field.constant)
    {
        let field_name = format_ident!("{}", field.name.to_case(Snake));
        let intial_value = field.initial_value.clone();

        body.extend(quote! {
            self.#field_name = #intial_value;
        });
    }

    output.extend(quote! {
        #comments
        #[ink(constructor)]
        pub fn new(#params) -> Self{
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                #body
            })
        }
        _blank_!();
    });

    output
}

/// Assembles ink! functions from the vec of parsed Function structs and return them as a vec of Strings
fn assemble_functions(functions: &Vec<Function>, is_library: bool) -> TokenStream {
    let mut output = TokenStream::new();

    for function in functions.iter() {
        let mut function_name = TokenStream::new();
        let mut view = TokenStream::new();
        let mut params = TokenStream::new();
        let mut return_params = TokenStream::new();
        let mut body = TokenStream::new();
        let mut comments = TokenStream::new();
        let mut function_modifiers = TokenStream::new();
        let mut statements = function.body.clone();

        // assemble comments
        for comment in function.header.comments.iter() {
            comments.extend(quote! {
                #[doc = #comment]
            });
        }

        for function_modifier in function.header.modifiers.iter() {
            function_modifiers.extend(quote! {
                #[modifiers(#function_modifier)]
            });
        }

        // assemble function name
        function_name.extend(
            TokenStream::from_str(&format!(
                "{}fn {}{}",
                if !function.header.external {
                    String::from("default ")
                } else if is_library {
                    String::from("pub ")
                } else {
                    String::new()
                },
                if !function.header.external {
                    String::from("_")
                } else {
                    String::new()
                },
                if function.header.external {
                    check_expression_for_keywords(&function.header.name).to_case(Snake)
                } else {
                    function.header.name.to_case(Snake)
                }
            ))
            .unwrap(),
        );

        // assemble view
        view.extend(
            TokenStream::from_str(match function.header.view {
                true => "&self",
                false => "&mut self",
            })
            .unwrap(),
        );

        // assemble params
        for param in function.header.params.iter() {
            let param_name = format_ident!(
                "{}",
                check_expression_for_keywords(&param.name).to_case(Snake)
            );
            let param_type = TokenStream::from_str(&param.param_type).unwrap();

            params.extend(quote! {
                , #param_name: #param_type
            });
        }

        // assemble return params
        if !function.header.return_params.is_empty() {
            let mut params = TokenStream::new();

            for i in 0..function.header.return_params.len() {
                let param = &function.header.return_params[i];
                let param_type = TokenStream::from_str(&param.param_type).unwrap();

                if i > 0 {
                    params.extend(quote! {,});
                }

                params.extend(quote! {
                    #param_type
                });

                if param.name != "_" {
                    let param_name = TokenStream::from_str(
                        &check_expression_for_keywords(&param.name).to_case(Snake),
                    )
                    .unwrap();
                    body.extend(quote! {
                        let mut #param_name = Default::default();
                    })
                }
            }

            if function.header.return_params.len() > 1 {
                return_params.extend(quote! {
                    (#params)
                });
            } else {
                return_params.extend(quote! {
                    #params
                });
            }
        } else {
            return_params.extend(quote! {
                ()
            });
        }

        // @notice Rust fmt will panic if a return statement is followed by a statement
        // we will switch the order of the comments and the return statement
        if !function.header.return_params.is_empty() && function.header.return_params[0].name == "_"
        {
            let mut ordered = VecDeque::<Statement>::default();
            while let Some(Statement::Comment(_)) = statements.iter().last() {
                ordered.push_front(statements.pop().unwrap());
            }
            // the last statement now is the return statement
            if !ordered.is_empty() {
                let return_statement = statements.pop().unwrap();
                // we insert a comment that we reordered
                statements.push(Statement::Comment(
                    "Following statements were found after the return statement >>>".to_string(),
                ));
                // append ordered to statements
                statements.append(&mut ordered.into_iter().collect());
                statements.push(Statement::Comment(
                    "<<< Following statements were found after the return statement".to_string(),
                ));
                statements.push(return_statement);
            }
        }

        // body
        body.extend(quote! {
            #(#statements)*
        });

        if function.header.return_params.is_empty() {
            body.extend(quote! {
                Ok(())
            });
        } else if function.header.return_params[0].name != "_" {
            let out = TokenStream::from_str(
                &function
                    .header
                    .return_params
                    .iter()
                    .map(|param| check_expression_for_keywords(&param.name).to_case(Snake))
                    .collect::<Vec<String>>()
                    .join(","),
            )
            .unwrap();
            if !statements.iter().any(|s| matches!(s, Statement::Return(_))) {
                body.extend(
                    if function.header.return_params.len() > 1 {
                        quote! {
                            Ok((#out))
                        }
                    } else {
                        quote! {
                            Ok(#out)
                        }
                    },
                );
            }
        }

        output.extend(quote! {
            #comments
            #function_modifiers
            #function_name(#view #params) -> Result<#return_params, Error> {
                #body
            }
        });

        output.extend(quote! {
            _blank_!();
        });
    }

    output
}

fn assemble_emit_functions(events: &Vec<Event>) -> (TokenStream, TokenStream) {
    let mut default_output = TokenStream::new();
    let mut impl_output = TokenStream::new();

    for event in events.iter() {
        let event_name =
            TokenStream::from_str(&format!("_emit_{}", &event.name.to_case(Snake))).unwrap();
        let mut event_args = TokenStream::new();
        let mut unnamed_event_args = TokenStream::new();

        // assemble event fields
        for event_field in event.fields.iter() {
            let event_field_name = format_ident!(
                "{}",
                check_expression_for_keywords(&event_field.name).to_case(Snake)
            );
            let event_field_type = TokenStream::from_str(&event_field.field_type).unwrap();

            event_args.extend(quote! {
                #event_field_name: #event_field_type,
            });

            unnamed_event_args.extend(quote! {
                _: #event_field_type,
            });
        }

        default_output.extend(quote! {
            fn #event_name (&self, #event_args );
            _blank_!();
        });
        impl_output.extend(quote! {
            default fn #event_name (&self, #unnamed_event_args ) {}
            _blank_!();
        });
    }

    (default_output, impl_output)
}

fn assemble_contract_emit_functions(events: &Vec<Event>) -> TokenStream {
    let mut output = TokenStream::new();

    for event in events.iter() {
        let fn_name =
            TokenStream::from_str(&format!("_emit_{}", &event.name.to_case(Snake))).unwrap();
        let mut event_args = TokenStream::new();
        let mut event_params = TokenStream::new();
        let event_name = TokenStream::from_str(&event.name).unwrap();

        // assemble event fields
        for event_field in event.fields.iter() {
            let event_field_name = format_ident!(
                "{}",
                check_expression_for_keywords(&event_field.name).to_case(Snake)
            );
            let event_field_type = TokenStream::from_str(&event_field.field_type).unwrap();

            event_params.extend(quote! {
                #event_field_name: #event_field_type,
            });
            event_args.extend(quote! {
                #event_field_name,
            });
        }

        output.extend(quote! {
            fn #fn_name (&self, #event_params ) {
                self.env().emit_event(#event_name { #event_args });
            }
            _blank_!();
        });
    }

    output
}

/// Assembles ink! functions from the vec of parsed Function structs and return them as a vec of Strings
fn assemble_modifiers(modifiers: &Vec<Modifier>, contract_name: &Ident) -> TokenStream {
    let mut output = TokenStream::new();

    for modifier in modifiers.iter() {
        let modifier_name = format_ident!(
            "{}",
            check_expression_for_keywords(&modifier.header.name).to_case(Snake)
        );
        let mut body = TokenStream::new();
        let mut comments = TokenStream::new();
        let mut params = TokenStream::new();

        // assemble comments
        for comment in modifier.comments.iter() {
            comments.extend(quote! {
                #[doc = #comment]
            });
        }
        let statements = &modifier.statements;

        // assemble params
        for param in modifier.header.params.iter() {
            let param_name = format_ident!("{}", param.name.to_case(Snake));
            let param_type = TokenStream::from_str(&param.param_type).unwrap();

            params.extend(quote! {
                , #param_name: #param_type
            });
        }

        // body
        body.extend(quote! {
            #(#statements)*
        });

        output.extend(quote! {
            #comments
            #[modifier_definition]
            pub fn #modifier_name<T, F, R>(instance: &mut T, body: F #params) -> Result<R, Error>
            where
                T: #contract_name,
                F: FnOnce(&mut T) -> Result<R, Error>
            {
                #body
            }
        });

        output.extend(quote! {
            _blank_!();
        });
    }

    output
}

/// Assembles ink! trait function headers from the vec of parsed FunctionHeader structs and return them as a vec of Strings
fn assemble_function_headers(function_headers: &Vec<FunctionHeader>) -> TokenStream {
    let mut output = TokenStream::new();

    for header in function_headers.iter() {
        let mut function_comments = TokenStream::new();
        let mut message = TokenStream::new();
        let mut function_name = TokenStream::new();
        let mut view = TokenStream::new();
        let mut params = TokenStream::new();
        let mut return_params = TokenStream::new();

        // assemble comments
        for comment in header.comments.iter() {
            function_comments.extend(quote! {
                #[doc = #comment]
            });
        }

        // assemble message
        if header.external {
            if header.payable {
                message.extend(quote! {
                    #[ink(message, payable)]
                });
            } else {
                message.extend(quote! {
                    #[ink(message)]
                });
            }
        }

        // assemble function name
        function_name.extend(
            TokenStream::from_str(&format!(
                "fn {}{}",
                if header.external { "" } else { "_" },
                check_expression_for_keywords(&header.name).to_case(Snake)
            ))
            .unwrap(),
        );

        // assemble view
        view.extend(
            TokenStream::from_str(match header.view {
                true => "&self",
                false => "&mut self",
            })
            .unwrap(),
        );

        // assemble params
        for param in header.params.iter() {
            let param_name = format_ident!(
                "{}",
                check_expression_for_keywords(&param.name).to_case(Snake)
            );
            let param_type = TokenStream::from_str(&param.param_type).unwrap();

            params.extend(quote! {
                , #param_name: #param_type
            });
        }

        // assemble return params
        if !header.return_params.is_empty() {
            let mut params = TokenStream::new();
            for i in 0..header.return_params.len() {
                let param_type =
                    TokenStream::from_str(&header.return_params[i].param_type).unwrap();

                if i > 0 {
                    params.extend(quote! {,});
                }
                params.extend(quote! {
                    #param_type
                });
            }

            if header.return_params.len() > 1 {
                return_params.extend(quote! {
                    (#params)
                });
            } else {
                return_params.extend(quote! {
                    #params
                });
            }
        } else {
            return_params.extend(quote! {
                ()
            });
        }

        output.extend(quote! {
            #function_comments
            #message
            #function_name(#view #params) -> Result<#return_params, Error>;
        });

        output.extend(quote! {
            _blank_!();
        });
    }

    output
}

/// Adds a signature to the beginning of the file :)
fn signature() -> TokenStream {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let version = &format!("Generated with Sol2Ink v{}\n", VERSION);
    let link = "https://github.com/Supercolony-net/sol2ink\n";
    quote! {
        _comment_!(#version);
        _comment_!(#link);
        _blank_!();
    }
}

fn check_expression_for_keywords(expression_raw: &String) -> String {
    if RUST_KEYWORDS.contains(&expression_raw.as_str()) {
        format!("{}_is_rust_keyword", &expression_raw)
    } else {
        expression_raw.to_string()
    }
}

impl ToTokens for Operation {
    fn to_tokens(&self, stream: &mut TokenStream) {
        stream.extend(match self {
            Operation::Add => quote!(+),
            Operation::AddAssign => quote!(+=),
            Operation::AndAssign => quote!(&=),
            Operation::Assign => quote!(=),
            Operation::BitwiseAnd => quote!(&),
            Operation::BitwiseOr => quote!(|),
            Operation::Div => quote!(/),
            Operation::DivAssign => quote!(/=),
            Operation::Equal => quote!(==),
            Operation::GreaterThanEqual => quote!(>=),
            Operation::GreaterThan => quote!(>),
            Operation::LessThanEqual => quote!(<=),
            Operation::LessThan => quote!(<),
            Operation::LogicalAnd => quote!(&&),
            Operation::LogicalOr => quote!(||),
            Operation::Modulo => quote!(%),
            Operation::Mul => quote!(*),
            Operation::MulAssign => quote!(*=),
            Operation::Not => quote!(!),
            Operation::NotEqual => quote!(!=),
            Operation::OrAssign => quote!(|=),
            Operation::Pow => quote!(),
            Operation::ShiftLeft => quote!(<<),
            Operation::ShiftRight => quote!(>>),
            Operation::Subtract => quote!(-),
            Operation::SubtractAssign => quote!(-=),
            Operation::Xor => quote!(^),
            _ => quote!(),
        })
    }
}

impl ToTokens for Statement {
    fn to_tokens(&self, stream: &mut TokenStream) {
        match self {
            Statement::AssemblyEnd => {}
            Statement::Assign(left, right, operation) => {
                stream.extend(quote! {
                    #left #operation #right;
                })
            }
            Statement::Break => stream.extend(quote! {break}),
            Statement::Catch(statements) => {
                stream.extend(quote! {
                    else if false {
                        #(#statements)*
                        _comment_!("<<< Please handle try/catch blocks manually");
                    }
                })
            }
            Statement::CatchEnd => {}
            Statement::Comment(content) => {
                stream.extend(quote! {
                    _comment_!(#content);
                })
            }
            Statement::Declaration(var_name_raw, var_type_raw, initial_value_maybe) => {
                let var_name = format_ident!("{}", var_name_raw.to_case(Snake));
                let var_type = TokenStream::from_str(var_type_raw).unwrap();
                if let Some(initial_value) = &initial_value_maybe {
                    stream.extend(quote!(let mut #var_name : #var_type = #initial_value;));
                } else {
                    stream.extend(quote!(let mut #var_name : #var_type;));
                }
            }
            Statement::Delete(mapping, index) => {
                if index.clone().len() == 1 {
                    stream.extend(quote!(#mapping.remove(#(&#index)*);));
                } else {
                    stream.extend(quote!(#mapping.remove(&(#(#index,)*));));
                }
            }
            Statement::Loop(assign, condition, modification, statements) => {
                stream.extend(quote! {
                    #assign
                    loop {
                        #(#statements)*
                        #modification
                        if #condition {
                            break;
                        }
                    }
                })
            }
            Statement::Group(statements) => {
                stream.extend(quote! {
                        #(#statements)*
                })
            }
            Statement::Else(statements) => {
                stream.extend(quote! {
                    else {
                        #(#statements)*
                    }
                })
            }
            Statement::ElseIf(condition_raw, statements) => {
                let left = &condition_raw.left;
                let operation = condition_raw.operation;
                let condition = if let Some(right) = &condition_raw.right {
                    quote!(#left #operation #right)
                } else {
                    quote!(#operation #left)
                };
                stream.extend(quote! {
                    else if #condition {
                        #(#statements)*
                    }
                })
            }
            Statement::Emit(event_name_raw, args) => {
                let fn_name =
                    TokenStream::from_str(&format!("_emit_{}", &event_name_raw.to_case(Snake)))
                        .unwrap();
                stream.extend(quote! {
                    self. #fn_name ( #(#args),* );
                })
            }
            Statement::FunctionCall(expression) => {
                stream.extend(quote! {
                    #expression;
                })
            }
            Statement::If(condition_raw, statements) => {
                let left = &condition_raw.left;
                let operation = condition_raw.operation;
                let condition = if let Some(right) = &condition_raw.right {
                    quote!(#left #operation #right)
                } else {
                    quote!(#operation #left)
                };
                stream.extend(quote! {
                    if #condition {
                        #(#statements)*
                    }
                })
            }
            Statement::IfEnd => {}
            Statement::ModifierBody => {
                stream.extend(quote! {
                    let result = body(instance);
                })
            }
            Statement::ModifierResult(last_statement) => {
                if *last_statement {
                    stream.extend(quote! {
                        result
                    })
                } else {
                    stream.extend(quote! {
                        body(instance);
                    })
                }
            }
            Statement::Raw(_) => {}
            Statement::Require(condition_raw, expression_raw, constructor) => {
                let left = &condition_raw.left;
                let operation = condition_raw.operation;
                let expression = if let Expression::Literal(_) = expression_raw {
                    quote!(String::from(#expression_raw))
                } else {
                    quote!(#expression_raw)
                };

                let error = if *constructor {
                    quote! {
                        panic!(#expression)
                    }
                } else {
                    quote! {
                        return Err(Error::Custom(#expression))
                    }
                };

                let condition = if let Some(right) = &condition_raw.right {
                    quote!(#left #operation #right)
                } else {
                    quote!(#operation #left)
                };
                stream.extend(quote! {
                    if #condition {
                        #error
                    }
                })
            }
            Statement::Return(output) => {
                stream.extend(quote! {
                    return Ok(#output)
                })
            }
            Statement::Ternary(condition_raw, if_true, if_false) => {
                let left = &condition_raw.left;
                let operation = condition_raw.operation;
                stream.extend(
                    if let Some(right) = &condition_raw.right {
                        quote! {
                            if #left #operation #right {
                                #if_true
                            } else {
                                #if_false
                            }
                        }
                    } else {
                        quote! {
                            if #operation #left {
                                #if_true
                            } else {
                                #if_false
                            }
                        }
                    },
                );
            }
            Statement::Try(statements) => {
                stream.extend(quote! {
                    _comment_!("Please handle try/catch blocks manually >>>");
                    if true {
                        #(#statements)*
                    }
                })
            }
            Statement::TryEnd => {}
            Statement::While(assign, condition, modification, statements) => {
                stream.extend(quote! {
                    #assign
                    while #condition {
                        #(#statements)*
                        #modification
                    }
                })
            }
            Statement::WhileEnd => {}
        }
    }
}

impl ToTokens for Expression {
    fn to_tokens(&self, stream: &mut TokenStream) {
        stream.extend(match self {
            Expression::AccountId(selector_raw) => {
                let namespace =
                    TokenStream::from_str(&selector_raw.clone().unwrap_or_default()).unwrap();
                let selector = if selector_raw == &Some(String::from("Self")) {
                    quote!(::)
                } else {
                    quote!(.)
                };
                quote!(#namespace #selector env().account_id())
            }
            Expression::Arithmetic(left, right, operation) => {
                if operation == &Operation::Pow {
                    quote!(#left.pow(#right as u32))
                } else {
                    quote!(#left #operation #right)
                }
            }
            Expression::BlockTimestamp(selector_raw) => {
                let namespace =
                    TokenStream::from_str(&selector_raw.clone().unwrap_or_default()).unwrap();
                let selector = if selector_raw == &Some(String::from("Self")) {
                    quote!(::)
                } else {
                    quote!(.)
                };
                quote!(#namespace #selector env().block_timestamp())
            }
            Expression::Cast(unique_cast, cast_type_raw, expression) => {
                let cast_type = TokenStream::from_str(cast_type_raw).unwrap();
                if *unique_cast {
                    quote!(#cast_type(#expression))
                } else {
                    quote!((#expression as #cast_type))
                }
            }
            Expression::Condition(condition_raw) => {
                let left = &condition_raw.left;
                let operation = condition_raw.operation;
                if let Some(right_raw) = &condition_raw.right {
                    let right = right_raw;
                    quote!(#left #operation #right)
                } else {
                    quote!(#operation #left)
                }
            }
            Expression::Constant(expression_raw) => {
                let expression =
                    TokenStream::from_str(&expression_raw.to_case(UpperSnake)).unwrap();
                quote!(#expression)
            }
            Expression::Enclosed(expression) => {
                quote!((#expression))
            }
            Expression::EnvCaller(selector_raw) => {
                let namespace =
                    TokenStream::from_str(&selector_raw.clone().unwrap_or_default()).unwrap();
                let selector = if selector_raw == &Some(String::from("Self")) {
                    quote!(::)
                } else {
                    quote!(.)
                };
                quote!(#namespace #selector env().caller())
            }
            Expression::FunctionCall(function_name_raw, args_raw, selector_maybe, external) => {
                let mut function_call = TokenStream::new();
                if let Some(selector_raw) = selector_maybe {
                    let selector = TokenStream::from_str(selector_raw).unwrap();
                    function_call.extend(quote!(#selector.))
                }
                let function_name = format_ident!(
                    "{}{}",
                    if *external { "" } else { "_" },
                    function_name_raw.to_case(Snake)
                );
                let mut args = TokenStream::new();
                for (i, arg) in args_raw.iter().enumerate() {
                    if i > 0 {
                        args.extend(quote!(,#arg));
                    } else {
                        args.extend(quote!(#arg));
                    }
                }
                quote! {
                    #function_call #function_name(#args)?
                }
            }
            Expression::IsZero(expression) => {
                quote!(#expression.is_zero())
            }
            Expression::Literal(content_raw) => {
                let content = if content_raw.len() > 3 && content_raw.starts_with('\'') {
                    let mut content_tmp = content_raw.clone();
                    content_tmp.replace_range(0..1, "\"");
                    content_tmp.replace_range(content_tmp.len() - 1.., "\"");
                    content_tmp.to_owned()
                } else {
                    content_raw.to_owned()
                };
                TokenStream::from_str(&content).unwrap_or_else(|_| {
                    TokenStream::from_str(format!("\"S2I_ERR:{content}\"").as_str()).unwrap()
                })
            }
            Expression::Logical(left, operation, right) => {
                quote!(#left #operation #right)
            }
            Expression::Member(expression_raw, selector_raw) => {
                let expression_updated = check_expression_for_keywords(expression_raw);
                let expression_maybe = TokenStream::from_str(&expression_updated.to_case(Snake));
                if let Ok(expression) = expression_maybe {
                    if let Some(selector_raw) = selector_raw {
                        let selector = format_ident!("{}", selector_raw);
                        quote!(#selector.data().#expression)
                    } else {
                        quote!(#expression)
                    }
                } else {
                    TokenStream::from_str("S2I_INCORRECTLY_PARSED_MEMBER").unwrap()
                }
            }
            Expression::Mapping(expression, indices_raw, insert_maybe) => {
                let indices = if indices_raw.len() > 1 {
                    let mut inner = TokenStream::new();
                    for i in 0..indices_raw.len() {
                        let expression = indices_raw.get(i).unwrap();
                        if i > 0 {
                            inner.extend(quote!(,));
                        }
                        inner.extend(quote!(#expression));
                    }
                    quote!((#inner))
                } else {
                    let expression = indices_raw.get(0).unwrap();
                    quote!(#expression)
                };
                if let Some(insert) = insert_maybe {
                    quote!(#expression.insert(&#indices, &(#insert)))
                } else {
                    quote!(#expression.get(&#indices).unwrap_or_default())
                }
            }
            Expression::Modifier(modifier_raw) => {
                let modifier = TokenStream::from_str(modifier_raw).unwrap();
                quote!(#modifier)
            }
            Expression::NewArray(array_type_raw, array_size) => {
                let array_type = TokenStream::from_str(array_type_raw).unwrap();
                quote!(vec![#array_type::default(); #array_size])
            }
            Expression::StructArg(field_name_raw, value) => {
                let field_name = TokenStream::from_str(&field_name_raw.to_case(Snake)).unwrap();
                quote!(#field_name : #value)
            }
            Expression::StructInit(struct_name_raw, struct_args_raw) => {
                let mut struct_args = TokenStream::new();
                for i in 0..struct_args_raw.len() {
                    let expression = struct_args_raw.get(i).unwrap();
                    if i > 0 {
                        struct_args.extend(quote!(,));
                    }
                    struct_args.extend(quote!(#expression));
                }
                let struct_name = TokenStream::from_str(&struct_name_raw.to_case(Pascal)).unwrap();
                quote!(#struct_name {#struct_args})
            }
            Expression::Ternary(condition_raw, if_true, if_false) => {
                let left = &condition_raw.left;
                let operation = condition_raw.operation;
                if let Some(right) = &condition_raw.right {
                    quote! {
                        if #left #operation #right {
                            #if_true
                        } else {
                            #if_false
                        }
                    }
                } else {
                    quote! {
                        if #operation #left {
                            #if_true
                        } else {
                            #if_false
                        }
                    }
                }
            }
            Expression::TransferredValue(selector_raw) => {
                let namespace =
                    TokenStream::from_str(&selector_raw.clone().unwrap_or_default()).unwrap();

                let selector = if selector_raw == &Some(String::from("Self")) {
                    quote!(::)
                } else {
                    quote!(.)
                };

                quote!(#namespace #selector env().transferred_value())
            }
            Expression::WithSelector(left, right) => {
                quote!(#left.#right)
            }
            Expression::ZeroAddressInto => quote!(ZERO_ADDRESS.into()),
        })
    }
}
