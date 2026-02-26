# 🐱 LynxScript

LynxScript is a programming language designed for CatWeb, a Roblox game where users can build website-esque creations.
The LynxScript compiler compiles source code files into a JSON format that can be imported into CatWeb.  
NOTE: This project is in early development and is still work in progress. Expect bugs and breaking changes.

## Usage
### Syntax
```js
// Familiar syntax to web-devs
console.log("Hello, world!");

// In-language standard library definition
#[export_as("console.log")]
function log(arg) {
  // Raw CatWeb block ID calls
  #0(#"", arg);
}
```
### Command line interface
```bash
# Compile a LynxScript source file to JSON and output it to output.json
lync --compile ./src/main.lxs --output ./out/output.json

# Or just output the JSON onto the console
lync -c ./src/main.lxs
# (-c is shorthand for --compile, and -o for --output)
```

## Features/ Roadmap
- [x] Function declarations
- [x] Event handlers
- [x] Raw CatWeb block ID calls
- [x] In-language standard library implementation
- [ ] Link statement (Importing site JSON files and reference UI objects)
- [ ] Arbitary expression compilation (binary, boolean)
- [ ] Return statements
- [ ] If statements
- [ ] Loops
- [ ] Optimizations
  - [ ] Function inlining
  - [ ] Constant folding
  - [ ] Dead code elimination

## Installation
TODO

## Development
### Prerequisites
- [Rust](https://rust-lang.org/) (Edition 2024 or newer)
- Cargo (comes with Rust)
- [Deno](https://deno.com/) (Option, only used for data generation scripts)

### Building and Running
To build the project, run the following command in the project directory:
```bash
cargo build
```
To run the CLI in development mode, use the following commands in the project directory:
```bash
# Compiles the script and output to a file
cargo run -- --compile "<path_to_your_script>.lxs" --output "<path_to_destination>.json" 

# Compiles the script and output to stdout
cargo run -- -c "<path_to_your_script>.lxs" 
# -c is shorthand for --compile, and -o for --output
```

## License
This project is licensed under the [MIT License](LICENSE)

## Acknowledgments
- Built with the blazingly fast [Rust](https://www.rust-lang.org/)
- PEG-grammar parser powered by [pest](https://pest.rs/)
- Similar project: [catlua](https://github.com/quitism/catlua) also shaped the ecosystem of CatWeb text-based programming languages ✨

---

<div align="center">

**⭐ Star this repo if you find it helpful!**

Made with ❤️ by [pickaxe828](https://github.com/pickaxe828) and [the contributors](https://github.com/pickaxe828/LynxScript/graphs/contributors)

</div>