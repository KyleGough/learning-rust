# Rust
Rust is an ahead-of-time compiled language.


## Introduction

Rust is an ahead-of-time compiled language.


### Installation
Install Rust
```
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

Update Rust
```
rustup update
```

Uninstall
```
rustup self uninstall
````

### Hello World
In a file called `hello_world.rs` add the following:
```
fn main() {
  println!("Hello world");
}
```

### Compilation and Running
Compiling
```
rustc hello_world.rs
```

Running
```
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
