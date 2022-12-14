# Rust
> This project will serve as my ongoing notes, code snippets, and review of the Rust programming language as I learn it whilst working through the [The Rust Programming Language](https://doc.rust-lang.org/book/) book. 

Rust is a multi-paradigm programming language emphasising performance, type safety, and concurrency. It was designed by Graydon Hoare and released its first stable version in 2014.
- Strongly, statically typed.
- Ahead-of-time compiled language.
- Type inference.

## Table of Contents
- [Programs](#programs)
- [Introduction](#introduction)
  - [Installation](#installation)
  - [Hello World](#hello-world)
  - [Compilation and Running](#compilation-and-running)
- [Cargo](#cargo)

## Programs
This repository includes the following small programs and mini projects:
- `hello-world` - basic hello world example.
- `guessing-game` - guess a random number between 1 and 100.
- `shadowing` - working example of variable shadowing.
- `indexing` - indexing arrays with user input index.
- `variables` - examples of defining variables.

## Introduction

### Installation
```sh
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

### Updating
```sh
rustup update
```

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
Cargo is the official package manager for the Rust language. Packages of code are referred to as `crates`. Cargo expects that all code is placed in the `src` directory. Below are a number of useful commands to create, build, and run projects.

- `cargo --version` - verify installation and check installed version.
- `cargo new learning-rust` - create a new project with a new Git repository.
- `cargo new` - create a new project within an existing Git repository.
- `cargo build` - build a Cargo project.
- `cargo run` - build and run in one command.
- `cargo build --release` - release build.
- `cargo check` - check code compiles without builing it.

## Terminology
- **Expression** - an instruction that evaluates to a value.
  - `12+ 18`
  - Calling a function
  - Calling a macro
- **Statement** - an instruction that performs an action and doesn't return a value.
  - `let x = 16;`
  - End with a semicolon ";"
- **Macros**
  - `println!(...)`

## Variables
By default variables are immutable meaning once a value is bound to a name it cannot be changed. Constants are always immutable and the type of the value must always be annotated. By convention constants are named in uppercase letters with underscores.
```rust
let x = 3;
let mut y = 0;
y = 100;
const SECONDS_IN_DAY: u32 = 60 * 60 * 24;
```

### Shadowing
Declaring a new variable with the same name as a previously declared variable will "overshadow" the previous, taking any uses of the variable name until it is shadowed itself or the scope ends. Shadowing allows a variable to keep the same name, but change the value or even the type.
```rust
let x = 1;
let x = x + 1;

{
    let x = x * 2;
    println!("Value of x is {x}"); // 4
}

println!("Value of x is {x}"); // 2
```
```rust
let password = "&t3e*OBt_c2";
let password = password.len()";
```

## Random Number Generator
The `Rng` trait defines methods that random number generators implement and must be in scope to use those methods. The `rand::thread_rng` function provides a random number generator local to the current thread of execution and seeded by the OS. The `gen_range` method takes a range expression as an argument and generates a random number in the range.

```rust
use rand::Rng

fn main() {
    let secret = rand::thread_rng().gen_range(1..=10);
    println!("Random number: {secret}");
}
```

## Data Types
Every value in Rust has a data type which can be divided into two groups: scalar and compound. Rust must know the type of all variables at compile time else it will throw a compilation error.

### Scalar Types
- Integer
  - `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.
  - `u8`, `u16`, `u32`, `u64`, `u128`, `usize`.
  - Signed integers are stored using two's complement.
  - The last variants `isize` and `usize` depend on the host systems architecture.
  - In debug mode integer overflow results in the program panicing, in release mode integer overflow causes numbers to wrap according to two's complement.
- Floating-Point number
  - `f64` is the default value with double-precision, but Rust also offers a single-precision `f32`.
- Boolean
  - `true`, `false`.
- Char
  - 'char'
  - Specified char literals with single quotes.
  - Stored using 4 bytes representing a Unicode Scalar Value.

### Compound Types
Compound types group multiple values into a single type. The two primitive compound types that Rust provides are: tuples, and arrays.

#### Tuples
**Tuples** can have a number of values each with different types. In the snippet below Rust creates a tuple and binds it to the variable `tuple`. Patterns can be used to destructure the tuple into 3 parts. Alternatively, a tuple element can be accessed using `.` and the index.
```rust
let tuple: (i32, f64, char) = (128, 3.14, 'A');
let (x, y, z) = tuple;
println!("The value of x is {x}");
println!("The value of y is {tuple.1}");
```

#### Arrays
**Arrays** are a collection of multiple values of fixed length, all with the same data type. Data for arrays will be allocated on the stack.
```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first: i32 = arr[0];
```
Declare an array with the same value for each element.
```rust
let arr: [i32, 4] = [64; 4];
```
If an invalid array index is accessed (such as from user input) then Rust will throw a runtime error and exit the program without executing further code. Arrays must be indexed with the type `usize`.

## Enums
- `Result` - an enum with the variants `Ok` and `Err`
- `Ordering` - `Less`, `Greater`, or `Equal`


## String
- `.len()` - returns the number of characters in a string.
- `.trim()` - removes whitespace from the beginning and end of the string.


## Pattern Matching
```rust
match result {
    Ok(num) => num,
    Err(_) => 0
};
```

## Functions
- `main` is the entry point of Rust programs.
- Functions are named using snake case (e.g. `add_one`).
- Functions can be declared in any order as long as they are in scope when called.

```rust
fn foo(x: i32) {
    println!("Value of x is {x}");
}
```

Expressions evaluate to a value, such as numerical operations, calling functions, or calling macros.
```rust
let x = {
    let y = 10;
    let z = 2;
    y / z
};
println!("x is {x}"); // 5
```

Functions will return the last expression implicitly, but values can be returned early using `return`. The following function doubles the input, note there is no semicolon as it is an expression.
```rust
fn double(x: i32) -> i32 {
    x * 2
}
```

## Comments
```rust
/*
 * Block Comment
 * Useful for temporarily disabling blocks of code.
 */
let x = 5; // Single line comment
```

## Formatted Print
- `println!` - text printed to io::stdout with newline appended.
- `print!` - text printed to io:stout without newline.
- `eprintln!` - text printed to io::stderr with newline appended.
- `eprint!` - text printed to io::stderr without newline.
- `format!` - output formatted text to String.

[Docs for std::fmt](https://doc.rust-lang.org/std/fmt/)
```rust
let name = "Alice";
let age = 32;
let theme = "red";

// Positional arguments.
println!("This is {0}, {0} is {1} years old.", name, age);

// Named arguments.
println!("{person} likes {colour} and {food}", food="pizza", person=name, colour=theme);

// Other
let x = 12345;
println!("Base 10: {}", x);
println!("Binary:  {:b}", x);
println!("Octal:   {:o}", x);
println!("Hex:     {:x}", x); // Lowercase hex
println!("Hex:     {:X}", x); // Uppercase hex

println!("{:5}cm", 20);
println!("{x:>10}"); // Right-align in 10-wide column
println!("{x:0>8}"); // Pad number with zeroes
println!("{x:0>width$}", width=6); // Named arguments by appending '$'
println!("{:^10}", x); // Centre align with width 10
println!("{:+}", x); // Display numerical sign (+/-)
println!("{:.2}", 3.141592); // Display number to 2 d.p.
println!("{:.dp$}", dp=8); // Display number to N d.p.
println!("{:>+20.2}", pi); // 2 d.p. right-aligned, 20 width, with sign.
```

Only types that implement `fmt::Display` can be formatted with `{}`.

### Other
```rust
cargo install cargo-watch
cargo watch -x run
```
