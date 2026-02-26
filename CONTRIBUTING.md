# Contributing to LynxScript
We welcome contributions to the LynxScript project! If you're interested in improving the language, parser, interpreter, or CLI, please consider the following:

- Fork the repository and create your feature branch from `master`.
- Ensure your code adheres to Rust's idiomatic best practices and passes `cargo fmt` and `cargo clippy`.
- Write tests for any new features or bug fixes, and ensure all existing tests pass.
- Submit a pull request with a clear description of your changes.

### Development Setup for Contributors
The development setup is straightforward:
```bash
git clone https://github.com/pickaxe828/LynxScript.git
cd LynxScript
cargo build         # Build the project
cargo run -- <args> # Run with arguments (cargo run -- path/to/script.lynx)
```
