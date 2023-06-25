# Packster

Multi-plateform deterministic and unobstrusive System Package Manager

## Goal

This project aim to provide a System-level Package Manager that deal with dependencies in a healthy, deterministic way.

The idea is to use concepts from Language Package Managers ( like cargo ) to handle properly the diamond dependency problem : basically allow multiple versions to be deployed.

## Setup

With rustup: https://www.rust-lang.org/tools/install

We stick to the `stable` distribution for now ( `nightly` conflicts with `ntapi` crate )

## How To ...

### Execute tests

```sh
cargo test
```

### Execute linter

```sh
cargo clippy
```

### Build a binary

```sh
cargo build --release
```

You'd find your binary in `target\release` ( `packster-cli` or `packster-cli.exe` depending on your building platform )

### Get CLI help

General help:
```sh
cargo run -- --help
```

Command specific help ( by exemple `pack` command ):
```sh
cargo run -- pack --help
```

### Create a package

Create a `packster.toml` file in a directory ( _let's say myproject_ ) containing files or directories you want to pack.

It shall contain something like :

```toml
identifier = "my-package"
version = "0.0.1"
```

Then create the package file with :

```sh
cargo run -- pack --project-workspace myproject --package-output-directory .
```

You'd then see in your current working directory the package package file as `my-package_0.0.1_b7112762ff233f95979dd390197187a66ac164a808628228ef41b43042dc582d.302e312e30.packster`

### Initialize a deployment location

Create an empty directory ( _let's say mylocation_ )

Then initialize a location inside with :
```sh
cargo run -- init-location --location-directory mylocation
```

You'd then see a lockfile named `packster.lock` inside `mylocation`



Deployment
Source
Dependency