# LynxScript Compiler Agent Instructions (AGENTS.md)

This file provides context, commands, styling rules, and guidelines for AI agents working on the LynxScript compiler.

## 1. Project Overview

LynxScript (LXS) is a lightweight scripting language designed to compile directly into CatWeb-compliant JSON block schemas (Roblox visual scripting). 
This repository contains the compiler implementation:
- **`lync` CLI Binary**: The frontend command-line interface.
- **`lynxscript` Library**: The backend compiler logic exposed for other integrations (such as WebAssembly wrapper bindings).

## 2. Build & Execution Commands

- **Build the Compiler**:
  ```bash
  cargo build
  ```
- **Compile a Source File**:
  ```bash
  cargo run --bin lync -- -c <path_to_source.lxs> -o <path_to_output.json>
  ```
- **Print Compiled output to stdout**:
  ```bash
  cargo run --bin lync -- -c <path_to_source.lxs>
  ```
- **Run Tests**:
  ```bash
  cargo test
  ```

## 3. Code Style Guidelines & Conventions

### Rust Coding Style
- Follow idiomatic Rust patterns (run `cargo fmt` before proposing changes).
- Maintain strict modular boundary lines: the parser, compiler/scope-tracker, and generator reside inside the `lynxscript` library target, keeping `main.rs` as a thin CLI wrapper.

### LynxScript Syntax Style
- **Identifiers**: Match `[a-zA-Z_][a-zA-Z0-9_]*`
- **Keywords**: `link`, `let`, `function`, `true`, `false`
- **Block IDs**: Prefixed with `#` followed by numbers (e.g. `#0`).
- **Attributes**: Prefixed with `#[` and brackets, e.g. `#[export_as("alias_name")]`.
- **String Types**: Single-line double quotes (`"..."`) and raw strings (`#"..."`).

### Commit Prefixes
When committing code, prefix your commit message with one of the following Conventional Commit prefixes:
- `feat:` for new features (e.g. new language syntax or compiler options)
- `fix:` for compiler bugs or lexer/parser fixes
- `refactor:` for internal refactorings (such as modularization or AST updates)
- `test:` for adding or updating test cases
- `docs:` for documentation updates (e.g. `AGENTS.md`)
- `chore:` for build system/dependency updates

### Testing & Verification
- AI agents are encouraged to write and run unit/integration tests (`cargo test`) to ensure compiler functionalities and code behaviors remain consistent before and after refactoring.
