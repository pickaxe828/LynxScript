# LynxScript

<div align="center">

![Rust](https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Cargo](https://img.shields.io/badge/cargo-FF8D00?style=for-the-badge&logo=rust&logoColor=white)

[![GitHub stars](https://img.shields.io/github/stars/pickaxe828/LynxScript?style=for-the-badge)](https://github.com/gaxolotl/LynxScript/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/pickaxe828/LynxScript?style=for-the-badge)](https://github.com/gaxolotl/LynxScript/network)
[![GitHub issues](https://img.shields.io/github/issues/pickaxe828/LynxScript?style=for-the-badge)](https://github.com/gaxolotl/LynxScript/issues)
[![GitHub license](https://img.shields.io/github/license/pickaxe828/LynxScript?style=for-the-badge)](LICENSE)

**The blazingly fast CatWeb programming language with simple syntax.**

</div>

## Overview

LynxScript is an experimental CatWeb scripting language implemented in Rust. It provides a way for more advanced CatWeb site developers to write code without the blocks and only text.

## Features

- **Custom Scripting Language:** Defines its own syntax and semantics for expressive scripting.
- **High Performance:** Built with Rust for efficiency and speed, suitable for various scripting tasks.
- **Friendly Error Diagnostics:** Utilizes `ariadne` to produce clear, colored, and helpful error messages for syntax and runtime issues, enhancing the developer experience.
- **Robust Parsing:** Employs the `chumsky` parser combinator library for flexible and powerful language parsing capabilities.
- **Command-Line Interface:** Offers an easy-to-use CLI for executing LynxScript files directly from your terminal with no extra setup needed.

## Quick Start

### Prerequisites
- **Rust Toolchain:** You need to have the [Rust programming language](https://www.rust-lang.org/tools/install) installed on your system. This includes `rustc` and `cargo`.

### Installation

1. **Clone the repository**
   Begin by cloning the LynxScript repository to your local machine:
   ```bash
   git clone https://github.com/gaxolotl/LynxScript.git
   cd LynxScript
   ```

2. **Build the project**
   Compile the project. This command will create a debug executable.
   ```bash
   cargo build
   ```
   The compiled executable will be located at `./target/debug/lynxscript`.

3. **Install the CLI tool (optional, for global access)**
   To make `lynxscript` globally available on your system's PATH, you can install it using Cargo:
   ```bash
   cargo install --path .
   ```
   After this, you can run `lynxscript` from any directory in your terminal.

## Usage

Once LynxScript is built or installed, you can use the `lynxscript` command to execute your `.lynx` script files. Use the `lynxscript --help` command for all available commands.

### Basic Execution

To run a LynxScript file:

```bash
# If installed globally via `cargo install --path .`:
lynxscript <path-to-your-script>.lynx

# If running from the repository directory (after `cargo build`):
./target/debug/lynxscript <path-to-your-script>.lynx
```

### Examples

*(TODO: provide more examples as the codebase grows)*

```bash
# Example 1: Running a basic script
# Assuming you have a file named `hello.lynx` with simple LynxScript code:
# print("Hello, LynxScript!");
lynxscript hello.lynx
# Expected output (demonstrative): Hello, LynxScript!

# Example 2: Inspecting a script (conceptual)
# If LynxScript includes a subcommand for syntax checking or linting:
# lynxscript check my_complex_script.lynx
# Expected output: (Diagnostic messages or success indication)
```

## Development

### Building the Project

To compile the project for development:
```bash
cargo build
```

For an optimized, production-ready build:
```bash
cargo build --release
```
The release executable can be found at `./target/release/lynxscript`.

### Running Tests

If the project includes unit or integration tests (common in Rust projects), you can execute them with Cargo:
```bash
cargo test
```

## License

This project is licensed under the [MIT License](LICENSE) - see the [LICENSE](LICENSE) file for full details.

## Acknowledgments

- Original author and creator of LynxScript: [pickaxe828](https://github.com/pickaxe828)
- Built with the blazing fast [Rust](https://www.rust-lang.org/)
- Command-line interface parsing powered by [clap](https://crates.io/crates/clap) library
- Language parsing logic made possible by [chumsky](https://crates.io/crates/chumsky) parser combinator library
- Enhanced error reporting thanks to the diagnostics provided by [ariadne](https://crates.io/crates/ariadne)

---

<div align="center">

**⭐ Star this repo if you find it helpful!**

Made with ❤️ by [pickaxe828](https://github.com/pickaxe828) and [the contributors](https://github.com/pickaxe828/LynxScript/graphs/contributors)

</div>
