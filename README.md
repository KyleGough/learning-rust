# Rust
> This project will serve as my ongoing notes, code snippets, and review of the Rust programming language as I learn it. I am currently working through [The Rust Programming Language](https://doc.rust-lang.org/book/) book. 

Rust is a multi-paradigm programming language emphasising performance, type safety, and concurrency. It was designed by Graydon Hoare and released its first stable version in 2014.
- Strongly, statically typed.
- Ahead-of-time compiled language.
- Type inference.

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

## Random Number Generator
The `Rng` trait defines methods that random number generators implement and must be in scope to use those methods. The `rand::thread_rng` function provides a random number generator local to the current thread of execution and seeded by the OS. The `gen_range` method takes a range expression as an argument and generates a random number in the range.

```rust
use rand::Rng

fn main() {
    let secret = rand::thread_rng().gen_range(1..=10);
    println!("Random number: {secret}");
}
```

## Types
Numerical
- `u32` - unsigned 32-bit number
- `i32' - 32-bit number
- 'i64' - 64-bit number

## Enums
- `Result` - an enum with the variants `Ok` and `Err`
- `Ordering` - `Less`, `Greater`, or `Equal`


## String

- `.trim()` - removes whitespace from the beginning and end of the string.
