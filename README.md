# Yet Another RCON Client (written in Rust btw)

[![codecov](https://codecov.io/gh/raian621/yarcon/graph/badge.svg?token=GQH1KQBTAI)](https://codecov.io/gh/raian621/yarcon) [![ci](https://github.com/raian621/yarcon/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/raian621/yarcon/actions/workflows/checks.yml) [![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/raian621/yarcon)](https://rust-reportcard.xuri.me/report/github.com/raian621/yarcon)

Currently, this RCON client is pretty basic, but should be able to interface with most servers that use the RCON protocol.

## Usage

```sh
yarcon -H HOSTNAME_HERE -P PORT_HERE -p PASSWORD_HERE
```

## Installation

You can download a pre-compiled binary for most OSes in the [Releases](https://github.com/raian621/yarcon/releases) section of this repo.

## Compilation

To compile yarcon on your machine, ensure you have the [Rust toolchain installed](https://www.rust-lang.org/tools/install), then clone the yarcon repo on your machine, navigate to the yarcon project directory, and run:

```sh
cargo build --release
```

in your terminal. The compiled binary can be found in the `target/releases` folder of the project directory.

## Running

To run the project in development, simply run

```sh
cargo run -- <args>
```
