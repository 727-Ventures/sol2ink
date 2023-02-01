![Sol2!nk](https://user-images.githubusercontent.com/43150707/215464954-13e4c8d8-96b4-49da-996c-3e79b8344b3a.png)

## Summary
**Sol2Ink is a tool for easy migration from Solidity to Ink! and Rust**

As we are the builders in the Dotsama ecosystem and experts in ink! smart contracts, we help companies with their path to the Dotsama ecosystem.
One of our many tasks is to help projects and teams migrate their smart contracts from popular Solidity to Polkadot's ink!. During this process,
we found out that the transition process may be unnecessarily long, and if we had a tool that would transpile a Solidity file to Rust and ink!, 
we would save much time. And that is how the idea of Sol2Ink was born.

### Capabilities

Sol2Ink in its current state is able to parse compilable Solidity interfaces into ink! traits and compilable Solidity contracts into ink! contracts, while leveraging the power of [OpenBrush](https://github.com/727-Ventures/openbrush-contracts). Currently, Sol2Ink supports only single file contract transpiling, not supporting inheritance. The output of Sol2Ink is a folder with the ink! smart contract and a Cargo.toml.

Some errors may occur in this version of Sol2Ink and will be fixed in upcoming versions.
With some statements, a parsing error can occur and cause the member to be parsed incorrectly. This needs to be corrected by the user.
The program may panic while parsing uncompilable code. Future versions should bring more user-friendly errors.
Some expressions may be parsed incorrectly, while still creating compilable code (one known example is `type(uint).max` is parsed as `u128.max` instead of `u128::MAX`.
And of course, as with all programs, there are probably some hidden unknown bugs as well :)

### Future development

- [X] Sol2Ink CLI
- [X] User friendly errors when transpiling uncompilable contract
- [X] Parsing libraries
- [X] Implement currently incorrectly parsed statements and expressions
- [X] Ability to parse a whole Solidity project into ink! project
- [X] Parse inheritance
- [ ] Sol2Ink Web Application with interface
- [ ] Make the parsed contracts 

### How to use it?

To run the application you will need to have installed Rust and run the nightly toolchain. ​
You can run the application with `cargo +nightly run contract.sol`, assuming you have a solidity file called contract.sol in the working directory.
The result will be stored in `contract/lib.rs` and the Cargo.toml file in `contract/Cargo.toml`.

You can transpile the example contracts from examples folder by running `cargo +nightly test creating`.

If you are using Sol2Ink from release pages, you will need to run `./sol2ink contract.sol`, substituting contract.sol with your Solidity contract's name.

### Examples

Examples are stored in the example folder, where we have the input Solidity file and the output Rust and Ink! file.

By running `cargo test`, we will transpile all of the examples stored in this folder. We have several example contracts from OpenZeppelin and two example contracts from Solang, and some Solidity contracts created by us. These original contracts were not modified, and the outputs of Sol2Ink are not modified either.

### Our Community

If you have any questions regarding Sol2Ink, you can join the [Brushfam Element channel](https://matrix.to/#/!utTuYglskDvqRRMQta:matrix.org?via=matrix.org&via=t2bot.io&via=web3.foundation) to find your answers and meet other ink! smart contracts developers.
