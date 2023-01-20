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

use std::collections::{
    HashMap,
    VecDeque,
};

use crate::structures::*;
use convert_case::{
    Case::Snake,
    Casing,
};
use rbtree::RBTree;
use solang_parser::{
    parse,
    pt::{
        Comment,
        ContractDefinition,
        ContractPart,
        ContractTy,
        EnumDefinition,
        EventDefinition,
        EventParameter,
        Expression as SolangExpression,
        FunctionAttribute,
        FunctionDefinition,
        FunctionTy,
        Identifier,
        IdentifierPath,
        Mutability,
        SourceUnitPart,
        Statement as SolangStatement,
        StructDefinition,
        Type as SolangType,
        VariableAttribute,
        VariableDeclaration,
        VariableDefinition,
        Visibility,
    },
};

pub enum ParserOutput {
    Contract(Contract),
    Interface(Interface),
    Library(Library),
    None,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParserError {
    FileError(String),
    FileCorrupted,

    ContractNameNotFound,
    StructNameNotFound,
    EventNameNotFound,
    VariableNameNotFound,
    EnumValueNotDefined,

    IncorrectTypeOfVariable,
}

impl From<std::io::Error> for ParserError {
    fn from(error: std::io::Error) -> Self {
        ParserError::FileError(error.to_string())
    }
}

#[derive(Debug)]
pub enum Node {
    ContractDefinition(ContractDefinition),
    Comments(String),
    EventDefinition(EventDefinition),
    EventParameter(EventParameter),
    EnumDefinition(EnumDefinition),
    FunctionDefinition(FunctionDefinition),
    Identifier(Identifier),
    SolangStatement(SolangStatement),
    StructDefinition(StructDefinition),
    VariableDeclaration(VariableDeclaration),
    VariableDefinition(VariableDefinition),
}

pub struct Parser<'a> {
    members_map: &'a mut HashMap<String, MemberType>,
    modifiers_map: &'a mut HashMap<String, FunctionDefinition>,
    pub rb_tree: &'a mut RBTree<usize, Node>,
}

impl<'a> Parser<'a> {
    pub fn new(
        members_map: &'a mut HashMap<String, MemberType>,
        rb_tree: &'a mut RBTree<usize, Node>,
        modifiers_map: &'a mut HashMap<String, FunctionDefinition>,
    ) -> Self {
        Parser {
            members_map,
            rb_tree,
            modifiers_map,
        }
    }

    pub fn parse_file(&mut self, content: &str) -> Result<Vec<ParserOutput>, ParserError> {
        let token_tree = parse(content, 0).map_err(|_| ParserError::FileCorrupted)?;

        let mut output = Vec::new();
        let source_unit = token_tree.0;
        let comments = token_tree.1;

        comments.iter().for_each(|comment| {
            match comment {
                Comment::Line(loc, comment)
                | Comment::Block(loc, comment)
                | Comment::DocLine(loc, comment)
                | Comment::DocBlock(loc, comment) => {
                    self.rb_tree
                        .insert(loc.start(), Node::Comments(comment.clone()));
                }
            }
        });

        for source_unit_part in source_unit.0.iter() {
            match &source_unit_part {
                SourceUnitPart::ContractDefinition(contract) => {
                    output.push(self.handle_contract_definition(contract)?);
                }
                SourceUnitPart::ImportDirective(_) | SourceUnitPart::PragmaDirective(..) => {}
                _ => println!("Found a source unit outside of contract"),
            }
        }

        Ok(output)
    }

    fn handle_contract_definition(
        &mut self,
        contract_definition: &ContractDefinition,
    ) -> Result<ParserOutput, ParserError> {
        match contract_definition.ty {
            ContractTy::Abstract(loc) | ContractTy::Contract(loc) => {
                self.rb_tree.insert(
                    loc.start(),
                    Node::ContractDefinition(contract_definition.clone()),
                );
                Ok(ParserOutput::Contract(
                    self.parse_contract(contract_definition)?,
                ))
            }
            ContractTy::Library(loc) => {
                self.rb_tree.insert(
                    loc.start(),
                    Node::ContractDefinition(contract_definition.clone()),
                );
                Ok(ParserOutput::Library(
                    self.parse_library(contract_definition)?,
                ))
            }
            ContractTy::Interface(loc) => {
                self.rb_tree.insert(
                    loc.start(),
                    Node::ContractDefinition(contract_definition.clone()),
                );
                Ok(ParserOutput::Interface(
                    self.parse_interface(contract_definition)?,
                ))
            }
        }
    }

    fn parse_contract(
        &mut self,
        contract_definition: &ContractDefinition,
    ) -> Result<Contract, ParserError> {
        let name = self.parse_identifier(&contract_definition.name);

        let mut structs: Vec<Struct> = Default::default();
        let mut events: Vec<Event> = Default::default();
        let mut enums: Vec<Enum> = Default::default();
        let mut fields: Vec<ContractField> = Default::default();
        let mut functions: Vec<Function> = Default::default();
        let mut constructor: Function = Default::default();
        let mut modifiers: Vec<Function> = Default::default();

        // first we register all members of the contract
        for part in contract_definition.parts.iter() {
            match part {
                ContractPart::VariableDefinition(variable_definition) => {
                    let parsed_field = self.parse_storage_field(variable_definition)?;
                    let constant = parsed_field.constant;
                    self.members_map.insert(
                        parsed_field.name.clone(),
                        if constant {
                            MemberType::Constant
                        } else {
                            MemberType::Variable(Box::new(parsed_field.field_type.clone()))
                        },
                    );
                    fields.push(parsed_field);
                }
                ContractPart::FunctionDefinition(function_definition) => {
                    let fn_name = self.parse_identifier(&function_definition.name);
                    let external = function_definition.attributes.iter().any(|attribute| {
                        matches!(
                            attribute,
                            FunctionAttribute::Visibility(Visibility::External(_))
                                | FunctionAttribute::Visibility(Visibility::Public(_))
                        )
                    });
                    match function_definition.ty {
                        FunctionTy::Function => {
                            self.members_map.insert(
                                fn_name.clone(),
                                if external {
                                    MemberType::Function
                                } else {
                                    MemberType::FunctionPrivate
                                },
                            );
                        }
                        FunctionTy::Modifier => {
                            self.modifiers_map
                                .insert(fn_name.clone(), *function_definition.clone());
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        for part in contract_definition.parts.iter() {
            match part {
                ContractPart::Annotation(_) => println!("Anottation: {part:?}"),
                ContractPart::StructDefinition(struct_definition) => {
                    let parsed_struct = self.parse_struct(struct_definition)?;
                    structs.push(parsed_struct);
                }
                ContractPart::EventDefinition(event_definition) => {
                    let parsed_event = self.parse_event(event_definition)?;
                    events.push(parsed_event);
                }
                ContractPart::EnumDefinition(enum_definition) => {
                    let parsed_enum = self.parse_enum(enum_definition)?;
                    enums.push(parsed_enum);
                }
                ContractPart::ErrorDefinition(_) => {}
                ContractPart::FunctionDefinition(function_definition) => {
                    let parsed_function = self.parse_function(function_definition)?;
                    match function_definition.ty {
                        FunctionTy::Constructor => constructor = parsed_function,
                        FunctionTy::Modifier => {
                            modifiers.push(parsed_function);
                        }
                        _ => functions.push(parsed_function),
                    }
                }
                ContractPart::TypeDefinition(_) => {}
                ContractPart::Using(_) => {}
                ContractPart::StraySemicolon(_) => {}
                _ => {}
            }
        }

        Ok(Contract {
            name,
            structs,
            events,
            enums,
            fields,
            functions,
            constructor,
            modifiers,
            ..Default::default()
        })
    }

    fn parse_interface(
        &mut self,
        contract_definition: &ContractDefinition,
    ) -> Result<Interface, ParserError> {
        let name = self.parse_identifier(&contract_definition.name);

        let mut structs: Vec<Struct> = Default::default();
        let mut events: Vec<Event> = Default::default();
        let mut enums: Vec<Enum> = Default::default();
        let mut function_headers: Vec<FunctionHeader> = Default::default();

        for part in contract_definition.parts.iter() {
            match part {
                ContractPart::Annotation(_) => println!("Anottation: {part:?}"),
                ContractPart::StructDefinition(struct_definition) => {
                    let parsed_struct = self.parse_struct(struct_definition)?;
                    structs.push(parsed_struct);
                }
                ContractPart::EventDefinition(event_definition) => {
                    let parsed_event = self.parse_event(event_definition)?;
                    events.push(parsed_event);
                }
                ContractPart::EnumDefinition(enum_definition) => {
                    let parsed_enum = self.parse_enum(enum_definition)?;
                    enums.push(parsed_enum);
                }
                ContractPart::ErrorDefinition(_) => {}
                ContractPart::FunctionDefinition(function_definition) => {
                    if function_definition.ty == FunctionTy::Function {
                        let header = self.parse_function_header(function_definition);
                        function_headers.push(header);
                    }
                }
                ContractPart::TypeDefinition(_) => {}
                ContractPart::Using(_) => {}
                ContractPart::StraySemicolon(_) => {}
                _ => {}
            }
        }

        Ok(Interface {
            name,
            events,
            enums,
            structs,
            function_headers,
            ..Default::default()
        })
    }

    fn parse_library(
        &mut self,
        contract_definition: &ContractDefinition,
    ) -> Result<Library, ParserError> {
        let name = self.parse_identifier(&contract_definition.name);

        let mut fields: Vec<ContractField> = Default::default();
        let mut events: Vec<Event> = Default::default();
        let mut enums: Vec<Enum> = Default::default();
        let mut structs: Vec<Struct> = Default::default();
        let mut functions: Vec<Function> = Default::default();

        // first we register all members of the contract
        for part in contract_definition.parts.iter() {
            if let ContractPart::FunctionDefinition(function_definition) = part {
                let fn_name = self.parse_identifier(&function_definition.name);
                let external = function_definition.attributes.iter().any(|attribute| {
                    matches!(
                        attribute,
                        FunctionAttribute::Visibility(Visibility::External(_))
                            | FunctionAttribute::Visibility(Visibility::Public(_))
                    )
                });
                self.members_map.insert(
                    fn_name.clone(),
                    if external {
                        MemberType::Function
                    } else {
                        MemberType::FunctionPrivate
                    },
                );
            }
        }

        for part in contract_definition.parts.iter() {
            match part {
                ContractPart::Annotation(_) => println!("Anottation: {part:?}"),
                ContractPart::StructDefinition(struct_definition) => {
                    let parsed_struct = self.parse_struct(struct_definition)?;
                    structs.push(parsed_struct);
                }
                ContractPart::EventDefinition(event_definition) => {
                    let parsed_event = self.parse_event(event_definition)?;
                    events.push(parsed_event);
                }
                ContractPart::EnumDefinition(enum_definition) => {
                    let parsed_enum = self.parse_enum(enum_definition)?;
                    enums.push(parsed_enum);
                }
                ContractPart::ErrorDefinition(_) => {}
                ContractPart::FunctionDefinition(function_definition) => {
                    if function_definition.ty == FunctionTy::Function {
                        let parsed_function = self.parse_function(function_definition)?;
                        functions.push(parsed_function)
                    }
                }
                ContractPart::TypeDefinition(_) => {}
                ContractPart::Using(_) => {}
                ContractPart::StraySemicolon(_) => {}
                ContractPart::VariableDefinition(variable_definition) => {
                    let parsed_field = self.parse_storage_field(variable_definition)?;
                    self.members_map.insert(
                        parsed_field.name.clone(),
                        MemberType::Variable(Box::new(parsed_field.field_type.clone())),
                    );
                    fields.push(parsed_field);
                }
            }
        }

        Ok(Library {
            name,
            fields,
            events,
            enums,
            structs,
            functions,
            ..Default::default()
        })
    }

    fn parse_struct(
        &mut self,
        struct_definition: &StructDefinition,
    ) -> Result<Struct, ParserError> {
        self.rb_tree.insert(
            struct_definition.loc.start(),
            Node::StructDefinition(struct_definition.clone()),
        );
        let name = self.parse_identifier(&struct_definition.name);

        let fields: Vec<StructField> = struct_definition
            .fields
            .iter()
            .filter_map(|variable_declaration| {
                self.rb_tree.insert(
                    variable_declaration.loc.start(),
                    Node::VariableDeclaration(variable_declaration.clone()),
                );
                let name = self.parse_identifier(&variable_declaration.name);
                let ty = self.parse_type(&variable_declaration.ty).ok()?;
                self.members_map
                    .insert(name.clone(), MemberType::None(Box::new(ty.clone())));
                Some(StructField {
                    name,
                    field_type: ty,
                    comments: Default::default(),
                })
            })
            .collect();
        Ok(Struct {
            name,
            fields,
            comments: Default::default(),
        })
    }

    fn parse_event(&mut self, event_definition: &EventDefinition) -> Result<Event, ParserError> {
        self.rb_tree.insert(
            event_definition.loc.start(),
            Node::EventDefinition(event_definition.clone()),
        );
        let name = self.parse_identifier(&event_definition.name);

        let fields: Vec<EventField> = event_definition
            .fields
            .iter()
            .filter_map(|variable_declaration| {
                self.rb_tree.insert(
                    variable_declaration.loc.start(),
                    Node::EventParameter(variable_declaration.clone()),
                );
                Some(EventField {
                    name: self.parse_identifier(&variable_declaration.name),
                    field_type: self.parse_type(&variable_declaration.ty).ok()?,
                    indexed: variable_declaration.indexed,
                    comments: Default::default(),
                })
            })
            .collect();
        Ok(Event {
            name,
            fields,
            comments: Default::default(),
        })
    }

    fn parse_enum(&mut self, enum_definition: &EnumDefinition) -> Result<Enum, ParserError> {
        self.rb_tree.insert(
            enum_definition.loc.start(),
            Node::EnumDefinition(enum_definition.clone()),
        );
        let name = self.parse_identifier(&enum_definition.name);

        let values: Vec<EnumValue> = event_definition
            .values
            .iter()
            .map(|enum_value| {
                self.insert_enum_field_to_tree(enum_value.clone());
                EnumField {
                    name: self.parse_identifier(enum_value),
                    comments: Default::default(),
                }
            })
            .collect();
        Ok(Enum {
            name,
            values,
            comments: Default::default(),
        })
    }

    fn insert_enum_field_to_tree(&mut self, enum_value: Option<Identifier>) {
        if let Some(value) = enum_value {
            self.rb_tree
                .insert(value.loc.start(), Node::Identifier(value));
        }
    }

    fn parse_storage_field(
        &mut self,
        variable_definition: &VariableDefinition,
    ) -> Result<ContractField, ParserError> {
        self.rb_tree.insert(
            variable_definition.loc.start(),
            Node::VariableDefinition(variable_definition.clone()),
        );
        let field_type = self.parse_type(&variable_definition.ty)?;
        let name = self.parse_identifier(&variable_definition.name);
        let constant = variable_definition
            .attrs
            .iter()
            .any(|item| matches!(item, VariableAttribute::Constant(_)));
        let public = variable_definition.attrs.iter().any(|item| {
            matches!(
                item,
                VariableAttribute::Visibility(Visibility::External(_))
                    | VariableAttribute::Visibility(Visibility::Public(_))
            )
        });
        let initial_value = variable_definition.initializer.as_ref().map(|expression| {
            self.parse_expression(expression, VariableAccessLocation::Constructor)
        });
        let comments = Vec::default();
        Ok(ContractField {
            field_type,
            name,
            initial_value,
            constant,
            public,
            comments,
        })
    }

    fn parse_function(
        &mut self,
        function_definition: &FunctionDefinition,
    ) -> Result<Function, ParserError> {
        let header = self.parse_function_header(function_definition);
        let mut invalid_modifiers = HashMap::new();
        for modifier in header.invalid_modifiers.clone() {
            match modifier {
                Expression::InvalidModifier(name, _) => {
                    if let Some(function) = self.modifiers_map.get(&name) {
                        invalid_modifiers.insert(
                            (header.name.clone(), name),
                            Function {
                                header: self.parse_function_header(function),
                                body: if let Some(body) = &function.body {
                                    Some(self.parse_statement(
                                        body,
                                        self.parse_variable_access_location(function_definition),
                                    )?)
                                } else {
                                    None
                                },
                                ..Default::default()
                            },
                        );
                    }
                }
                _ => unreachable!("Only invalid modifiers are allowed here"),
            }
        }

        let body = if let Some(statement) = &function_definition.body {
            Some(self.parse_statement(
                statement,
                self.parse_variable_access_location(function_definition),
            )?)
        } else {
            None
        };

        Ok(Function {
            header,
            body,
            invalid_modifiers,
        })
    }

    fn parse_function_header(
        &mut self,
        function_definition: &FunctionDefinition,
    ) -> FunctionHeader {
        self.rb_tree.insert(
            function_definition.loc.start(),
            Node::FunctionDefinition(function_definition.clone()),
        );
        let name = self.parse_identifier(&function_definition.name);
        let params = function_definition
            .params
            .iter()
            .map(|item| item.1.clone().unwrap())
            .filter_map(|param| {
                let name = self.parse_identifier(&param.name);
                let param_type = self.parse_type(&param.ty).ok()?;
                Some(FunctionParam { name, param_type })
            })
            .collect();
        let all_modifiers: Vec<Expression> = function_definition
            .attributes
            .iter()
            .filter(|&attribute| matches!(attribute, FunctionAttribute::BaseOrModifier(..)))
            .map(|modifier| {
                if let FunctionAttribute::BaseOrModifier(_, base) = modifier {
                    let parsed_name = self.parse_identifier_path(&base.name);
                    let parsed_args = if let Some(args) = &base.args {
                        self.parse_expression_vec(args, VariableAccessLocation::Any)
                    } else {
                        Vec::default()
                    };
                    if parsed_args.iter().any(function_call_in_expression) {
                        Expression::InvalidModifier(parsed_name, parsed_args)
                    } else {
                        Expression::Modifier(parsed_name, parsed_args)
                    }
                } else {
                    unreachable!("The vec was filtered before");
                }
            })
            .collect();
        let modifiers = all_modifiers
            .iter()
            .filter(|expression| matches!(expression, Expression::Modifier(..)))
            .cloned()
            .collect();
        let invalid_modifiers = all_modifiers
            .iter()
            .filter(|expression| matches!(expression, Expression::InvalidModifier(..)))
            .cloned()
            .collect();
        let external = function_definition.attributes.iter().any(|attribute| {
            matches!(
                attribute,
                FunctionAttribute::Visibility(Visibility::External(_))
                    | FunctionAttribute::Visibility(Visibility::Public(_))
            )
        });
        let view = function_definition.attributes.iter().any(|attribute| {
            matches!(
                attribute,
                FunctionAttribute::Mutability(Mutability::Pure(_))
                    | FunctionAttribute::Mutability(Mutability::View(_))
            )
        });
        let payable = function_definition.attributes.iter().any(|attribute| {
            matches!(
                attribute,
                FunctionAttribute::Mutability(Mutability::Payable(_))
            )
        });
        let return_params = function_definition
            .returns
            .iter()
            .map(|item| item.1.clone().unwrap())
            .filter_map(|param| {
                let name = self.parse_identifier(&param.name);
                let param_type = self.parse_type(&param.ty).ok()?;
                Some(FunctionParam { name, param_type })
            })
            .collect();

        FunctionHeader {
            name,
            params,
            external,
            view,
            payable,
            return_params,
            modifiers,
            invalid_modifiers,
            ..Default::default()
        }
    }

    fn parse_variable_access_location(
        &self,
        function_definition: &FunctionDefinition,
    ) -> VariableAccessLocation {
        match function_definition.ty {
            FunctionTy::Constructor => VariableAccessLocation::Constructor,
            FunctionTy::Modifier => VariableAccessLocation::Modifier,
            _ => VariableAccessLocation::Any,
        }
    }

    fn parse_statement(
        &self,
        statement: &SolangStatement,
        location: VariableAccessLocation,
    ) -> Result<Statement, ParserError> {
        let (result, result_location) = match statement {
            SolangStatement::Block {
                loc,
                unchecked,
                statements,
            } => {
                let parsed_statements = statements
                    .iter()
                    .map(|statement| self.parse_statement(statement, location.clone()))
                    .map(|result| result.unwrap())
                    .collect::<Vec<_>>();
                if *unchecked {
                    (Statement::UncheckedBlock(parsed_statements), loc)
                } else {
                    (Statement::Block(parsed_statements), loc)
                }
            }
            SolangStatement::Assembly {
                loc: _,
                dialect: _,
                flags: _,
                block: _,
            } => {
                println!("{statement:?}");
                todo!()
            }
            SolangStatement::Args(_, _) => {
                println!("{statement:?}");
                todo!()
            }
            SolangStatement::If(loc, expression, if_true, if_false) => {
                let parsed_expression = self.parse_expression(expression, location.clone());
                let parsed_if_true = Box::new(self.parse_statement(if_true, location.clone())?);
                let parsed_if_false = if_false
                    .as_ref()
                    .map(|statement| self.parse_statement(statement, location.clone()))
                    .map(|result| Box::new(result.unwrap()));
                (
                    Statement::If(parsed_expression, parsed_if_true, parsed_if_false),
                    loc,
                )
            }
            SolangStatement::While(_, expression, statement) => {
                let parsed_expression = self.parse_expression(expression, location.clone());
                let parsed_statement = Box::new(self.parse_statement(statement, location)?);
                (Statement::While(parsed_expression, parsed_statement), loc)
            }
            SolangStatement::Expression(loc, expression) => {
                let parsed_expression = self.parse_expression(expression, location);
                (Statement::Expression(parsed_expression), loc)
            }
            SolangStatement::VariableDefinition(loc, declaration, initial_value_maybe) => {
                let parsed_declaration = self.parse_variable_declaration(declaration)?;
                let parsed_initial_value = initial_value_maybe
                    .as_ref()
                    .map(|expression| self.parse_expression(expression, location));
                (
                    Statement::VariableDefinition(parsed_declaration, parsed_initial_value),
                    loc,
                )
            }
            SolangStatement::For(loc, variable_definition, condition, on_pass, body) => {
                let parsed_variable_definition = variable_definition
                    .as_ref()
                    .map(|statement| self.parse_statement(statement, location.clone()))
                    .map(|result| Box::new(result.unwrap()));
                let parsed_condition = condition
                    .as_ref()
                    .map(|expression| self.parse_expression(expression, location.clone()));
                let parsed_on_pass = on_pass
                    .as_ref()
                    .map(|statement| self.parse_statement(statement, location.clone()))
                    .map(|result| Box::new(result.unwrap()));
                let parsed_body = body
                    .as_ref()
                    .map(|statement| self.parse_statement(statement, location))
                    .map(|result| Box::new(result.unwrap()));
                (
                    Statement::For(
                        parsed_variable_definition,
                        parsed_condition,
                        parsed_on_pass,
                        parsed_body,
                    ),
                    loc,
                )
            }
            SolangStatement::DoWhile(loc, body, condition) => {
                let parsed_condition = self.parse_expression(condition, location.clone());
                let parsed_body = Box::new(self.parse_statement(body, location)?);
                (Statement::DoWhile(parsed_body, parsed_condition), loc)
            }
            SolangStatement::Continue(loc) => (Statement::Continue, loc),
            SolangStatement::Break(loc) => (Statement::Break, loc),
            SolangStatement::Return(loc, expression) => {
                let parsed_expression = expression
                    .as_ref()
                    .map(|expression| self.parse_expression(expression, location));
                (Statement::Return(parsed_expression), loc)
            }
            SolangStatement::Revert(loc, identifier_path, args) => {
                let identifier_path = identifier_path
                    .as_ref()
                    .map(|identifier_path| self.parse_identifier_path(identifier_path))
                    .unwrap_or(String::from("_"));
                let parsed_args = self.parse_expression_vec(args, location);
                (Statement::Revert(identifier_path, parsed_args), loc)
            }
            SolangStatement::RevertNamedArgs(_, _, _) => todo!(),
            SolangStatement::Emit(loc, expression) => {
                let parsed_expression = self.parse_expression(expression, location);
                (Statement::Emit(parsed_expression), loc)
            }
            SolangStatement::Try(loc, expression, _, _) => {
                let parsed_expression = self.parse_expression(expression, location);
                (Statement::Try(parsed_expression), loc)
            }
            SolangStatement::Error(_) => {
                todo!()
            }
        };
        self.rb_tree
            .insert(result_location.start(), Node::SolangStatement(statement.clone()));
        Ok(result)
    }

    fn parse_variable_declaration(
        &mut self,
        variable_declaration: &VariableDeclaration,
    ) -> Result<Expression, ParserError> {
        self.rb_tree.insert(
            variable_declaration.loc.start(),
            Node::VariableDeclaration(variable_declaration.clone()),
        );
        let parsed_name = self
            .parse_identifier(&variable_declaration.name)
            .to_case(Snake);
        let parsed_type = Box::new(self.parse_type(&variable_declaration.ty)?);
        Ok(Expression::VariableDeclaration(parsed_type, parsed_name))
    }

    fn parse_expression(
        &self,
        expression: &SolangExpression,
        location: VariableAccessLocation,
    ) -> Expression {
        macro_rules! maybe_boxed_expression {
            ($to_declare:ident,$to_parse:expr) => {
                let $to_declare = $to_parse.as_ref().map(|expression| {
                    Box::new(self.parse_expression(&expression, location.clone()))
                });
            };
        }

        macro_rules! boxed_expression {
            ($to_declare:ident,$to_parse:expr) => {
                let $to_declare = Box::new(self.parse_expression($to_parse, location.clone()));
            };
        }

        match expression {
            SolangExpression::PostIncrement(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::PostIncrement(parsed_expression)
            }
            SolangExpression::PostDecrement(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::PostDecrement(parsed_expression)
            }
            SolangExpression::New(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::New(parsed_expression)
            }
            SolangExpression::ArraySubscript(_, array, index_maybe) => {
                match *array.clone() {
                    SolangExpression::ArraySubscript(..) => {
                        self.array_subscript_to_mapping_subscript(array, index_maybe, location)
                    }
                    SolangExpression::MemberAccess(_, expression, _) => {
                        let parsed_expresion = self.parse_expression(&expression, location.clone());
                        match parsed_expresion {
                            Expression::MappingSubscript(..) => {
                                self.array_subscript_to_mapping_subscript(
                                    array,
                                    index_maybe,
                                    location,
                                )
                            }
                            _ => {
                                boxed_expression!(parsed_array, array);
                                maybe_boxed_expression!(parsed_index_maybe, index_maybe);
                                Expression::ArraySubscript(parsed_array, parsed_index_maybe)
                            }
                        }
                    }
                    SolangExpression::Variable(identifier) => {
                        let parsed_identifier = self.parse_identifier(&Some(identifier));
                        match self.members_map.get(&parsed_identifier) {
                            Some(ty) => {
                                match ty {
                                    MemberType::Variable(variable_type)
                                    | MemberType::None(variable_type) => {
                                        match *variable_type.clone() {
                                            Type::Mapping(_, _) => {
                                                self.array_subscript_to_mapping_subscript(
                                                    array,
                                                    index_maybe,
                                                    location,
                                                )
                                            }
                                            _ => {
                                                boxed_expression!(parsed_array, array);
                                                maybe_boxed_expression!(
                                                    parsed_index_maybe,
                                                    index_maybe
                                                );
                                                Expression::ArraySubscript(
                                                    parsed_array,
                                                    parsed_index_maybe,
                                                )
                                            }
                                        }
                                    }
                                    _ => {
                                        boxed_expression!(parsed_array, array);
                                        maybe_boxed_expression!(parsed_index_maybe, index_maybe);
                                        Expression::ArraySubscript(parsed_array, parsed_index_maybe)
                                    }
                                }
                            }
                            None => {
                                boxed_expression!(parsed_array, array);
                                maybe_boxed_expression!(parsed_index_maybe, index_maybe);
                                Expression::ArraySubscript(parsed_array, parsed_index_maybe)
                            }
                        }
                    }
                    _ => {
                        boxed_expression!(parsed_array, array);
                        maybe_boxed_expression!(parsed_index_maybe, index_maybe);
                        Expression::ArraySubscript(parsed_array, parsed_index_maybe)
                    }
                }
            }
            SolangExpression::ArraySlice(_, _, _, _) => {
                todo!()
            }
            SolangExpression::Parenthesis(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::Parenthesis(parsed_expression)
            }
            SolangExpression::MemberAccess(_, expression, identifier) => {
                boxed_expression!(parsed_expression, expression);
                let parsed_identifier = self.parse_identifier(&Some(identifier.clone()));
                Expression::MemberAccess(parsed_expression, parsed_identifier)
            }
            SolangExpression::FunctionCall(_, function, args) => {
                boxed_expression!(parsed_function, function);
                let parsed_args = self.parse_expression_vec(args, location);
                Expression::FunctionCall(parsed_function, parsed_args)
            }
            SolangExpression::FunctionCallBlock(_, _, _) => todo!(),
            SolangExpression::NamedFunctionCall(_, expression, arguments) => {
                boxed_expression!(parsed_expression, expression);
                let parsed_arguments = arguments
                    .iter()
                    .map(|argument| {
                        let parsed_argument =
                            self.parse_expression(&argument.expr, location.clone());
                        let parsed_name = self.parse_identifier(&Some(argument.name.clone()));
                        (parsed_name, parsed_argument)
                    })
                    .collect();
                Expression::NamedFunctionCall(parsed_expression, parsed_arguments)
            }
            SolangExpression::Not(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::Not(parsed_expression)
            }
            SolangExpression::Complement(_, _) => todo!(),
            SolangExpression::Delete(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::Delete(parsed_expression)
            }
            SolangExpression::PreIncrement(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::PreIncrement(parsed_expression)
            }
            SolangExpression::PreDecrement(_, expression) => {
                boxed_expression!(parsed_expression, expression);
                Expression::PreDecrement(parsed_expression)
            }
            SolangExpression::UnaryPlus(_, _) => todo!(),
            SolangExpression::UnaryMinus(_, _) => todo!(),
            SolangExpression::Power(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Power(parsed_left, parsed_right)
            }
            SolangExpression::Multiply(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Multiply(parsed_left, parsed_right)
            }
            SolangExpression::Divide(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Divide(parsed_left, parsed_right)
            }
            SolangExpression::Modulo(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Modulo(parsed_left, parsed_right)
            }
            SolangExpression::Add(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Add(parsed_left, parsed_right)
            }
            SolangExpression::Subtract(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Subtract(parsed_left, parsed_right)
            }
            SolangExpression::ShiftLeft(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::ShiftLeft(parsed_left, parsed_right)
            }
            SolangExpression::ShiftRight(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::ShiftRight(parsed_left, parsed_right)
            }
            SolangExpression::BitwiseAnd(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::BitwiseAnd(parsed_left, parsed_right)
            }
            SolangExpression::BitwiseXor(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::BitwiseXor(parsed_left, parsed_right)
            }
            SolangExpression::BitwiseOr(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::BitwiseOr(parsed_left, parsed_right)
            }
            SolangExpression::Less(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Less(parsed_left, parsed_right)
            }
            SolangExpression::More(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::More(parsed_left, parsed_right)
            }
            SolangExpression::LessEqual(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::LessEqual(parsed_left, parsed_right)
            }
            SolangExpression::MoreEqual(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::MoreEqual(parsed_left, parsed_right)
            }
            SolangExpression::Equal(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Equal(parsed_left, parsed_right)
            }
            SolangExpression::NotEqual(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::NotEqual(parsed_left, parsed_right)
            }
            SolangExpression::And(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::And(parsed_left, parsed_right)
            }
            SolangExpression::Or(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Or(parsed_left, parsed_right)
            }
            SolangExpression::ConditionalOperator(_, condition, if_true, if_false) => {
                boxed_expression!(parsed_condition, condition);
                boxed_expression!(parsed_if_true, if_true);
                boxed_expression!(parsed_if_false, if_false);
                Expression::Ternary(parsed_condition, parsed_if_true, parsed_if_false)
            }
            SolangExpression::Assign(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::Assign(parsed_left, parsed_right)
            }
            SolangExpression::AssignOr(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignOr(parsed_left, parsed_right)
            }
            SolangExpression::AssignAnd(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignAnd(parsed_left, parsed_right)
            }
            SolangExpression::AssignXor(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignXor(parsed_left, parsed_right)
            }
            SolangExpression::AssignShiftLeft(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignShiftLeft(parsed_left, parsed_right)
            }
            SolangExpression::AssignShiftRight(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignShiftRight(parsed_left, parsed_right)
            }
            SolangExpression::AssignAdd(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignAdd(parsed_left, parsed_right)
            }
            SolangExpression::AssignSubtract(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignSubtract(parsed_left, parsed_right)
            }
            SolangExpression::AssignMultiply(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignMultiply(parsed_left, parsed_right)
            }
            SolangExpression::AssignDivide(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignDivide(parsed_left, parsed_right)
            }
            SolangExpression::AssignModulo(_, left, right) => {
                boxed_expression!(parsed_left, left);
                boxed_expression!(parsed_right, right);
                Expression::AssignModulo(parsed_left, parsed_right)
            }
            SolangExpression::BoolLiteral(_, value) => Expression::BoolLiteral(*value),
            SolangExpression::NumberLiteral(_, literal, b) => {
                if !b.is_empty() {
                    println!("Number literal: B was {b}")
                }
                Expression::NumberLiteral(literal.clone())
            }
            SolangExpression::RationalNumberLiteral(_, _, _, _) => todo!(),
            SolangExpression::HexNumberLiteral(_, hex_number) => {
                Expression::HexLiteral(hex_number.clone())
            }
            SolangExpression::StringLiteral(strings) => {
                let parsed_strings = strings
                    .iter()
                    .map(|string_literal| string_literal.string.clone())
                    .collect();
                Expression::StringLiteral(parsed_strings)
            }
            SolangExpression::Type(_, solidity_type) => {
                let parsed_type = Box::new(self.convert_solidity_type(solidity_type));
                Expression::Type(parsed_type)
            }
            SolangExpression::HexLiteral(hex_vec) => {
                let literal = hex_vec
                    .iter()
                    .map(|hex| hex.hex.clone())
                    .collect::<Vec<_>>()
                    .join("");
                Expression::HexLiteral(literal)
            }
            SolangExpression::AddressLiteral(_, _) => todo!(),
            SolangExpression::Variable(identifier) => {
                let parsed_identifier = self.parse_identifier(&Some(identifier.clone()));
                if parsed_identifier == "_" {
                    return Expression::ModifierBody
                }
                let none = MemberType::None(Box::new(Type::None));
                let member_type = self.members_map.get(&parsed_identifier).unwrap_or(&none);
                Expression::Variable(parsed_identifier, member_type.clone(), location)
            }
            SolangExpression::List(_, parameters) => {
                let list = parameters
                    .iter()
                    .map(|tuple| tuple.1.clone())
                    .filter(|option| option.is_some())
                    .map(|parameter| parameter.unwrap().ty)
                    .map(|expression| self.parse_expression(&expression, location.clone()))
                    .collect();
                Expression::List(list)
            }
            SolangExpression::ArrayLiteral(_, _) => todo!(),
            SolangExpression::Unit(_, _, _) => todo!(),
            SolangExpression::This(_) => todo!(),
        }
    }

    fn parse_expression_vec(
        &self,
        expressions: &[SolangExpression],
        location: VariableAccessLocation,
    ) -> Vec<Expression> {
        expressions
            .iter()
            .map(|expression| self.parse_expression(expression, location.clone()))
            .collect()
    }

    fn array_subscript_to_mapping_subscript(
        &self,
        array: &SolangExpression,
        index_maybe: &Option<Box<SolangExpression>>,
        location: VariableAccessLocation,
    ) -> Expression {
        let mut parsed_indices = VecDeque::default();
        parsed_indices.push_back(
            self.parse_expression(&index_maybe.as_ref().unwrap().clone(), location.clone()),
        );
        let mut array_now = array.clone();
        while let SolangExpression::ArraySubscript(_, array, index_maybe) = array_now {
            parsed_indices
                .push_back(self.parse_expression(&index_maybe.unwrap(), location.clone()));
            array_now = *array.clone();
        }
        let mut vec_indices = Vec::default();
        while !parsed_indices.is_empty() {
            vec_indices.push(parsed_indices.pop_back().unwrap());
        }

        let parsed_array = Box::new(self.parse_expression(&array_now, location));
        Expression::MappingSubscript(parsed_array, vec_indices)
    }

    fn parse_identifier_path(&self, identifier_path: &IdentifierPath) -> String {
        identifier_path
            .identifiers
            .iter()
            .map(|identifier| identifier.name.clone())
            .collect::<Vec<String>>()
            .join("::")
    }

    fn parse_type(&self, ty: &SolangExpression) -> Result<Type, ParserError> {
        match &ty {
            SolangExpression::Type(_, SolangType::Mapping(_, key_type, value_type)) => {
                let mut parsed_key_types = vec![self.parse_type(key_type)?];
                let mut value_type_now = value_type.as_ref();
                while let SolangExpression::Type(
                    _,
                    SolangType::Mapping(_, key_type_value, value_type_value),
                ) = value_type_now
                {
                    parsed_key_types.push(self.parse_type(key_type_value)?);
                    value_type_now = value_type_value;
                }
                let parsed_value_type = self.parse_type(value_type_now)?;
                Ok(Type::Mapping(parsed_key_types, Box::new(parsed_value_type)))
            }
            SolangExpression::Type(_, solidity_type) => {
                Ok(self.convert_solidity_type(solidity_type))
            }
            SolangExpression::Variable(identifier) => Ok(Type::Variable(identifier.name.clone())),
            SolangExpression::ArraySubscript(_, ty, expression_maybe) => {
                let parsed_type = Box::new(self.parse_type(ty)?);
                if expression_maybe.is_some() {
                    unreachable!("expression maybe in array is some {expression_maybe:?}");
                }
                Ok(Type::Array(parsed_type, None))
            }
            _ => Err(ParserError::IncorrectTypeOfVariable),
        }
    }

    fn convert_solidity_type(&self, solidity_type: &SolangType) -> Type {
        match solidity_type {
            SolangType::Address | SolangType::AddressPayable => Type::AccountId,
            SolangType::Bool => Type::Bool,
            SolangType::String => Type::String,
            SolangType::Int(original_bytes) => Type::Int(self.convert_int_bytes(original_bytes)),
            SolangType::Uint(original_bytes) => Type::Uint(self.convert_int_bytes(original_bytes)),
            SolangType::Bytes(length) => Type::Bytes(*length),
            SolangType::DynamicBytes => Type::DynamicBytes,
            _ => Type::None,
        }
    }

    fn convert_int_bytes(&self, original_bytes: &u16) -> u16 {
        match *original_bytes {
            i if i <= 8 => 8,
            i if i <= 16 => 16,
            i if i <= 32 => 32,
            i if i <= 64 => 64,
            _ => 128,
        }
    }

    fn parse_identifier(&self, variable_declaration: &Option<Identifier>) -> String {
        match variable_declaration {
            Some(identifier) => identifier.name.clone(),
            None => String::from("_"),
        }
    }
}

fn function_call_in_expression(expresion: &Expression) -> bool {
    match expresion {
        Expression::Add(expr1, expr2)
        | Expression::And(expr1, expr2)
        | Expression::Assign(expr1, expr2)
        | Expression::AssignAdd(expr1, expr2)
        | Expression::AssignDivide(expr1, expr2)
        | Expression::AssignModulo(expr1, expr2)
        | Expression::AssignMultiply(expr1, expr2)
        | Expression::AssignSubtract(expr1, expr2)
        | Expression::Divide(expr1, expr2)
        | Expression::Equal(expr1, expr2)
        | Expression::Less(expr1, expr2)
        | Expression::LessEqual(expr1, expr2)
        | Expression::Modulo(expr1, expr2)
        | Expression::More(expr1, expr2)
        | Expression::MoreEqual(expr1, expr2)
        | Expression::Multiply(expr1, expr2)
        | Expression::NotEqual(expr1, expr2)
        | Expression::Or(expr1, expr2)
        | Expression::Subtract(expr1, expr2)
        | Expression::ShiftLeft(expr1, expr2)
        | Expression::ShiftRight(expr1, expr2)
        | Expression::BitwiseAnd(expr1, expr2)
        | Expression::BitwiseXor(expr1, expr2)
        | Expression::BitwiseOr(expr1, expr2)
        | Expression::AssignOr(expr1, expr2)
        | Expression::AssignAnd(expr1, expr2)
        | Expression::AssignXor(expr1, expr2)
        | Expression::AssignShiftLeft(expr1, expr2)
        | Expression::AssignShiftRight(expr1, expr2)
        | Expression::Power(expr1, expr2) => {
            function_call_in_expression(expr1) || function_call_in_expression(expr2)
        }
        Expression::List(list) => {
            list.iter()
                .map(function_call_in_expression)
                .any(|output| output)
        }
        Expression::MappingSubscript(expr, list) => {
            list.iter()
                .map(function_call_in_expression)
                .any(|output| output)
                || function_call_in_expression(expr)
        }
        Expression::ArraySubscript(expr1, expr2) => {
            function_call_in_expression(expr1)
                || expr2
                    .as_ref()
                    .map_or(false, |expression| function_call_in_expression(expression))
        }
        Expression::MemberAccess(expr, _) => function_call_in_expression(expr),
        Expression::New(expr)
        | Expression::Not(expr)
        | Expression::Parenthesis(expr)
        | Expression::PostDecrement(expr)
        | Expression::PostIncrement(expr)
        | Expression::PreDecrement(expr)
        | Expression::PreIncrement(expr) => function_call_in_expression(expr),
        Expression::Ternary(expr1, expr2, expr3) => {
            function_call_in_expression(expr1)
                || function_call_in_expression(expr2)
                || function_call_in_expression(expr3)
        }
        Expression::NamedFunctionCall(..) | Expression::FunctionCall(..) => true,
        Expression::Modifier(_, list) => {
            list.iter()
                .map(function_call_in_expression)
                .any(|output| output)
        }
        _ => false,
    }
}
