# XCSP3-Rust-Parser

[//]: # ([![Crate]&#40;https://img.shields.io/crates/v/quick-xml.svg&#41;]&#40;https://crates.io/crates/quick-xml&#41;)

[![xcsp3](https://img.shields.io/badge/xcsp3-red)](http://xcsp.org)
[![xcsp3rust](https://img.shields.io/badge/xcsp3_rust-8A2BE2)](https://github.com/luhanzhen/xcsp3-rust)
[![docs.rs](https://docs.rs/xcsp3-rust/badge.svg)](https://docs.rs/xcsp3-rust)
[![MSRV](https://img.shields.io/badge/rustc-1.70.0-90c541.svg)](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)
[![License](https://img.shields.io/badge/License-_MIT-blue)](https://github.com/luhanzhen/xcsp3-rust/blob/main/LICENSE)

## Description

`xcsp3-rust-parser` is a Rust library for reading constraint satisfaction and constraint optimization instances written in the [XCSP3](http://xcsp.org) format.
The library supports the XCSP3-core composed of the main components:

- CSP instances: constraint satisfaction problems
- COP instances: constraint optimization problems
- Integer variables and arrays
- Common global constraints (AllDiff, Regular, MDD, Count, Sum, cumulative...)
- Generic constraints (intention, table)
- Objectives

This library does not solve the problem by itself. Instead, it parses an XCSP3 file and reports the parsed elements through a callback interface.
This makes it useful for building XCSP3 frontends for Rust-based constraint solvers.

This project is heavily inspired by the [xcsp3-rust](https://github.com/luhanzhen/xcsp3-rust) project.
We are grateful to the original authors for their work!

## How it works

The parser follows an event/callback architecture.

You provide a type that implements `XcspCallback`.
While reading the XCSP3 file, the runner calls methods on your callback whenever it encounters variables, constraints,
objectives, or structural parts of the instance.

At a high level:

1. Create a callback object.
2. Call `XcspRunner::run(path, &mut callback)`.
3. The runner parses the XML file.
4. The runner calls callback methods such as:
    - `begin_instance`
    - `on_variable_interval`
    - `on_constraint_all_different_v1`
    - `on_constraint_intention`
    - `on_minimize_var`
    - `end_instance`

This lets solver developers translate XCSP3 instances into their own internal solver representation.

## Installation

Add the library to your `Cargo.toml`:

```toml 
[dependencies] xcsp3-parser-rust = "0.1.0"
```

Or, when using it from a local checkout:

```toml 
[dependencies] xcsp3-parser-rust = { path = "../xcsp3-rust" }
```

## Example

The following example parses an XCSP3 file and prints the variables and constraints using the built-in example callback.

```rust 
use xcsp3_rust::example_callback::PrintingSolver;
use xcsp3_rust::xcsp_runner::XcspRunner;

fn main() {
    let xml_file = "instances/my-example.xml";
    let mut solver = PrintingSolver::new();

    println!("Parse {}", xml_file);

    match XcspRunner::run(xml_file, &mut solver) {
        Ok(_) => println!("Successful parsing!"),
        Err(e) => eprintln!("Error during parsing: {}", e),
    }
}
```

Run it with:

```bash 
cargo run -- instances/my-example.xml
``` 

The example callback prints each parsed item, for example:

```text 
Parse instances/my-example.xml 
Start to load an instance of type Csp 

=== Variables === 
Interval Var x[0]: 1..9 
Interval Var x[1]: 1..9 
Number of variables: 2

=== Constraints === 
[AllDiff] ["x[0]", "x[1]"] 
Number of constraints: 1
Done... 
Successful parsing!
```

## Implementing your own callback

To connect the parser to your own solver, implement `XcspCallback`.
The file `src/example_callback.rs` contains an example implementation. It prints the parsed elements.

Unsupported or unimplemented callback methods may panic by default.
Override the corresponding callback method if your solver supports the construct.

## Library structure

The public modules are organized around the main XCSP3 concepts:

- `variables`: variable and domain representations
- `constraints`: constraint representations
- `objectives`: objective representations
- `data_structs`: shared data structures such as expression trees and operands
- `xcsp_xml`: XML model parsing
- `xcsp_callback`: callback trait used by solver frontends
- `xcsp_runner`: high-level runner that connects parsing and callbacks
- `example_callback`: example implementation that prints parsed elements

## Command-line usage

The crate also contains a simple executable that parses an XCSP3 file and uses the printing callback.

```bash 
cargo run -- path/to/instance.xml

```

## License

This project is distributed under the MIT License. See [`LICENSE`](LICENSE)

