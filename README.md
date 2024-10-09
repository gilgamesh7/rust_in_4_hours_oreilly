# rust_in_4_hours_oreilly
Andy Olsen's Rust in 4 hours course

# Links
- [Code & Slides link page](https://olsensoft.com/rust/)
- [Code & slides on Github](https://github.com/andyolsen/rust)
- [Rust Online Playground](https://play.rust-lang.org/?)
- [The Rust community’s crate registry](https://crates.io/)

# Installation
- Command : curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
- Check installation : rustc --version
    - Example output : rustc 1.81.0 (eeb90cda1 2024-09-04)

# Compile , code, run 
## Cargo
- Create new : cargo new name_of_app
- Fast Build Low Optimisation : cargo build
- Build and Optimise : cargo build --release
- Build & Run - debug : cargo run
- Build for release & run : cargo run --release
- Sanity Check : cargo check
## Simple
- Use .rs file extension
- To fix formatting : rustfmt file_name.rs
- To build : rustc file_name.rs