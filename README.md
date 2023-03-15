<div id="top"></div>
<p align="center">
<img src=https://img.shields.io/github/stars/sweatpotato13/substrate-parachain-boilerplate?style=for-the-badge&logo=appveyor&color=blue />
<img src=https://img.shields.io/github/forks/sweatpotato13/substrate-parachain-boilerplate?style=for-the-badge&logo=appveyor&color=blue />
<img src=https://img.shields.io/github/issues/sweatpotato13/substrate-parachain-boilerplate?style=for-the-badge&logo=appveyor&color=informational />
<img src=https://img.shields.io/github/issues-pr/sweatpotato13/substrate-parachain-boilerplate?style=for-the-badge&logo=appveyor&color=informational />
</p>
<br />
<!-- PROJECT LOGO -->
<p align="center">
  <a href="https://substrate.io/" target="blank"><img src="https://cdn-images-1.medium.com/max/960/1*OQP5QAtLtrVCtNCKwB6GkQ.png" width="320" alt="Nest Logo" /></a>
</p>

<br />
<div align="center">
  <a href="https://github.com/sweatpotato13/substrate-parachain-boilerplate">
    <!-- <img src="images/logo.png" alt="Logo" width="80" height="80"> -->
  </a>

<h3 align="center">Substrate Parachain boilerplate</h3>

  <p align="center">
    Substrate Parachain boilerplate
    <br />
    <a href="https://github.com/sweatpotato13/substrate-parachain-boilerplate"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/sweatpotato13/substrate-parachain-boilerplate">View Demo</a>
    ·
    <a href="https://github.com/sweatpotato13/substrate-parachain-boilerplate/issues">Report Bug</a>
    ·
    <a href="https://github.com/sweatpotato13/substrate-parachain-boilerplate/issues">Request Feature</a>
  </p>
</div>

<!-- TOC -->

<h3 align="center">Table of Contents</h3>
  <p align="center">
    <a href="#1-introduction">1. Introduction</a><br>
    <a href="#2-overview">2. Overview</a><br>
    <a href="#3-building">3. Building</a><br>
    <a href="#4-run">4. Run</a><br>
    <a href="#5-development">5. Development</a><br>
  </p>

<!-- /TOC -->

## 1. Introduction

Welcome to the Substrate Boilerplate! This is a starting point for building custom blockchains using the Substrate blockchain development framework from Parity Technologies.

Substrate is a modular and scalable framework that allows you to easily create custom blockchains with unique features and functionalities. The Substrate Boilerplate provides a template for building a Substrate-based blockchain with a basic set of features and components.

<p align="right">(<a href="#top">back to top</a>)</p>

## 2. Overview

The Substrate Boilerplate includes the following features:

-   Consensus: Proof of Authority (PoA, Aura) consensus algorithm
-   Governance: Council-based governance model
-   Token: Native token with initial supply
-   Runtime Modules: Example runtime modules for accounts, balances, and identity

With these features, you can quickly create a basic blockchain with a built-in token economy and governance system.

<p align="right">(<a href="#top">back to top</a>)</p>

## 3. Building

Install Rust:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

and install rust nightly:

```
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

and install other dependencies like profobuf, llvm:

```sh
# MacOS
brew install llvm protobuf
# Ubuntu
sudo apt-get install -y protobuf-compiler llvm
```

build infra-did node:

```sh
cargo build --release
```

<p align="right">(<a href="#top">back to top</a>)</p>

## 4. Run

You can start local development chain

```sh
./target/release/parachain-template-node --dev --alice --tmp
```

or start local chain

```sh
./target/release/parachain-template-node --chain=local --alice --tmp
```

<p align="right">(<a href="#top">back to top</a>)</p>

## 5. Development

You can start local development parachain with relay chain using [zombienet](https://github.com/paritytech/zombienet)

```sh
zombienet spawn --provider native zombienet/local-dev.toml
```

And apply rust formatter

```sh
cargo +nightly fmt
```
