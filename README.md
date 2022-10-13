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

Verify installation
```
cargo --version
```

Create a new project. Also initialises a new Git repository.
```
cargo new learning-rust
```

Create a new project within an existing Git repository.
```
cargo new
```

Build a Cargo project
```
cargo build
```

Running a Cargo project
```
./target/debug/learning-rust
```

Build and Run in one command
```
cargo run
```

Release build
```
cargo build --release
```

Check code compiles without builing it
```
cargo check
```



