# Rust
> This project will serve as my ongoing notes, code snippets, and review of the Rust programming language as I learn it. I am currently working through [The Rust Programming Language](https://doc.rust-lang.org/book/) book. 

Rust is a multi-paradigm programming language emphasising performance, type safety, and concurrency. It was designed by Graydon Hoare and with its first stable version released in 2014.
- Ahead-of-time compiled language.

## Table of Contents
- [Introduction](#introduction)
  - [Installation](#installation)
  - [Hello World](#hello-world)
  - [Compilation and Running](#compilation-and-running)
- [Cargo](#cargo)

## Introduction

### Installation
Install Rust
```sh
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

Update Rust
```sh
rustup update
```

Uninstall
```sh
rustup self uninstall
````

### Hello World
In a file called `hello_world.rs` add the following:
```rust
fn main() {
  println!("Hello world");
}
```

### Compilation and Running
```sh
rustc hello_world.rs
./hello_world
```

## Cargo
Cargo is the official package manager for the Rust language. Packages of code are referred to as `crates`. Cargo expects the all code is placed in the `src` directory.

- `cargo --version` - verify installation and check installed version.
- `cargo new learning-rust` - create a new project with a new Git repository.
- `cargo new` - create a new project within an existing Git repository.
- `cargo build` - build a Cargo project.
- `cargo run` - build and run in one command.
- `cargo build --release` - release build.
- `cargo check` - check code compiles without builing it.
