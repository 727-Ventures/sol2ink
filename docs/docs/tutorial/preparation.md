---
sidebar_position: 1
title: Preparation
---

In this tutorial, we will transpile the [ERC-20 contract](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/ERC20.sol) from OpenZeppelin.

#### Preparation

We will navigate to the folder where we saved `sol2ink`. To ease the access to input and output files, we will put the ERC-20 contract in the same directory. Before we run the program, we will make some adjustments to the ERC-20 files. We will add the events from the [IERC-20 interface](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/IERC20.sol). We do this because later in the contract, we want to emit these events, and if the parser does not know about them when emitting, it will panic with `Event XXX not defined`. Another thing we will change is to rewrite all functions `_msgSender()` to `msg.sender`. We do this because we want to demonstrate how to call `msg.sender` in ink!.

#### Running Sol2Ink

Running Sol2Ink is easy. Once we navigate to the directory where it resides, we will call `./sol2ink ERC20.sol`. Notice that we passed the name of the file as an argument. The output file will be stored in the newly created folder `ERC20`, containing files `Cargo.toml` and `lib.rs`. `Cargo.toml` will contain all the dependencies to build our contract, and the `lib.rs` file will contain the parsed ink! smart contract. Now we will try to build it!
