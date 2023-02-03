"use strict";(self.webpackChunksol_2_ink=self.webpackChunksol_2_ink||[]).push([[53],{1109:i=>{i.exports=JSON.parse('{"pluginId":"default","version":"current","label":"Next","banner":null,"badge":false,"className":"docs-version-current","isLast":true,"docsSidebars":{"tutorialSidebar":[{"type":"link","label":"Getting started","href":"/","docId":"intro"},{"type":"link","label":"Capabilities","href":"/capabilities","docId":"capabilities"},{"type":"category","label":"ERC-20 Tutorial","collapsible":true,"collapsed":false,"items":[{"type":"link","label":"Preparation","href":"/tutorial/preparation","docId":"tutorial/preparation"},{"type":"link","label":"Building the ink! smart contract","href":"/tutorial/building","docId":"tutorial/building"}]},{"type":"category","label":"How it works","collapsible":true,"collapsed":false,"items":[{"type":"link","label":"Parsing","href":"/how_it_works/parsing","docId":"how_it_works/parsing"},{"type":"link","label":"Parsing an interface","href":"/how_it_works/parsing_interface","docId":"how_it_works/parsing_interface"},{"type":"link","label":"Parsing a library","href":"/how_it_works/parsing_library","docId":"how_it_works/parsing_library"},{"type":"link","label":"Parsing a contract","href":"/how_it_works/parsing_contract","docId":"how_it_works/parsing_contract"},{"type":"link","label":"Parsing functions","href":"/how_it_works/parsing_functions","docId":"how_it_works/parsing_functions"},{"type":"link","label":"Parsing expressions","href":"/how_it_works/parsing_expressions","docId":"how_it_works/parsing_expressions"},{"type":"link","label":"Assembling a contract","href":"/how_it_works/assembler","docId":"how_it_works/assembler"}]},{"type":"link","label":"Known issues","href":"/issues","docId":"issues"}]},"docs":{"capabilities":{"id":"capabilities","title":"Capabilities","description":"Sol2Ink can parse Solidity files into ink! project while leveraging the power of OpenBrush. You can either parse a single file by providing the path to the file, or a whole folder by providing the path to teh folder. In the latter case, Sol2Ink will parse all Solidity files in the selected folder file tree and add them to one big ink! project. The output of Sol2Ink is a folder called generated with the following file structure:","sidebar":"tutorialSidebar"},"how_it_works/assembler":{"id":"how_it_works/assembler","title":"Assembling a contract","description":"Sol2Ink has everything it needs; now, it needs to mix it. Here we will clarify what may not be obvious.","sidebar":"tutorialSidebar"},"how_it_works/parsing":{"id":"how_it_works/parsing","title":"Parsing","description":"This section will look at how Sol2Ink works under the hood.","sidebar":"tutorialSidebar"},"how_it_works/parsing_contract":{"id":"how_it_works/parsing_contract","title":"Parsing a contract","description":"When parsing a contract, Sol2Ink will create an ink! trait definition, implementation of this trait, and a contract file from the parsed contract. This contract may include the following:","sidebar":"tutorialSidebar"},"how_it_works/parsing_expressions":{"id":"how_it_works/parsing_expressions","title":"Parsing expressions","description":"Another step of parsing a statement is parsing each expression. Here the program will decide how to parse each expression inside a statement.","sidebar":"tutorialSidebar"},"how_it_works/parsing_functions":{"id":"how_it_works/parsing_functions","title":"Parsing functions","description":"All parsed functions may include Statement enum variant from Solang. We need to convert this to Sol2Ink Statement, so it is more suitable for the ink! contract generation. We do this to ease some steps in the code generation, as well as to actually easily build the output code from these inputs. We will go over some remarkable points regarding the functions parsing.","sidebar":"tutorialSidebar"},"how_it_works/parsing_interface":{"id":"how_it_works/parsing_interface","title":"Parsing an interface","description":"When parsing an interface, Sol2Ink will create an ink! trait definition from the parsed interface. This trait definition may include the following:","sidebar":"tutorialSidebar"},"how_it_works/parsing_library":{"id":"how_it_works/parsing_library","title":"Parsing a library","description":"When parsing a libraray, Sol2Ink will create a plain Rust file, making all functions public so they can be used in the parsed contract. This file definition may include the following:","sidebar":"tutorialSidebar"},"intro":{"id":"intro","title":"Sol2Ink Documentation","description":"Welcome to Sol2Ink documentation. In this documentation, we will describe the capabilities of Sol2Ink, how the process works under the hood,","sidebar":"tutorialSidebar"},"issues":{"id":"issues","title":"Known issues","description":"Here is a list of known issues which you may face using Sol2Ink:","sidebar":"tutorialSidebar"},"tutorial/building":{"id":"tutorial/building","title":"Building the ink! smart contract","description":"To build the ink! smart contract we will need cargo-contract. Note that the generated contracts use ink! version 3.4.0, so we will need to have cargo-contract version 1.5.1 installed. So if we satisfy this condition, we will navigate to the generated folder generated/contracts/erc_20 and call cargo contract build. The contract will start building; we will wait for a while and...","sidebar":"tutorialSidebar"},"tutorial/preparation":{"id":"tutorial/preparation","title":"Preparation","description":"In this tutorial, we will transpile the ERC-20 contract from OpenZeppelin.","sidebar":"tutorialSidebar"}}}')}}]);