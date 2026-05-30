---
name: lynxscript
description: >
  Enforces specifications for writing, parsing, compiling, and debugging LynxScript (LXS) source files.
  Use this skill whenever the user asks to write LynxScript code, compile a `.lxs` file to CatWeb JSON,
  or when you need to understand LynxScript syntax, AST structure, and compiler name-mangling logic.
---

# LynxScript Language and Compiler Guide

## When to Activate

Activate this skill when:
- The user asks you to write, modify, or review LynxScript (`.lxs`) source files.
- The user asks to compile or parse LynxScript code.
- You are debugging LynxScript compiler stages (lexer, parser, AST generation, name mangling).
- The user asks to map language concepts to the CatWeb block-based schema.

Do NOT activate this skill when:
- The user is working on unrelated languages (like Python, TypeScript, or general Rust) unless they are explicitly developing integration bindings or extensions for LynxScript.

## Language Specifications & Rules

### Lexer Rules
- **Identifiers**: Match `[a-zA-Z_][a-zA-Z0-9_]*`
- **Keywords**: `link`, `let`, `function`, `true`, `false`
- **Integers**: Match `\d+`
- **Floats**: Match `\d+\.\d+`
- **String Literals**: `"..."` (standard double-quoted strings).
- **Raw String Literals**: `#"..."` (Outer quotes stripped by the parser, returning the raw payload inside).
- **Block IDs**: `#` followed by digits (e.g. `#0` maps to CatWeb block ID 0).
- **Attributes**: `#[` followed by an attribute name and optional arguments inside brackets, e.g. `#[export_as("c")]` or `#[inline]`.

### Operator Precedence (Lowest to Highest)
1. Comma (`,`)
2. Dot (`.`)
3. Addition (`+`), Subtraction (`-`)
4. Multiplication (`*`), Division (`/`)
5. Power (`**`)
6. Unary prefix (`-`, `!`)
7. Postfix call `(arguments)`

### AST Mapping
- **Program**: Houses `link_statements` and `main_block` items list.
- **Item**: Either an `AttributeItem` or a `FunctionDeclarationItem`.
- **Statement**:
  - `ExpressionStatement`: An expression ending in a semicolon.
  - `AssignmentStatement`: Variable declaration or variable assignment (`lhs = rhs;`).
- **Expression**:
  - `LiteralExpr`: Integers, floats, booleans, string literals, and raw string literals.
  - `IdentifierExpr`: Scope-resolved variable or function names.
  - `CallExpr`: Normal function calls `func(args)` or raw block calls `#0(args)`.
  - `BinOperationExpr` / `UnaryOperationExpr`: Arithmetic, logical operations, and comma-separated lists.

### Compiler & Name Mangling Rules
- **Function Registration**: The compiler runs a prepass to register all function declarations in the global symbol table. If the function has `#[export_as("name")]`, it is registered under the specified alias/name.
- **Scope Shadowing / Name Mangling**: If a variable is declared in a nested scope, shadowing a parent variable, the compiler mangles the nested name into `{original_name}__s{scope_id}`.
- **Operator Mapping**: Standard operators compile to standard mangled function names:
  - `+` -> `add`
  - `-` -> `sub`
  - `*` -> `mul`
  - `/` -> `div`
  - `**` -> `pow`

## Process

1. **Write LynxScript Code**:
   - Save source files with a `.lxs` extension.
   - Use correct attribute annotations to export functions to the host environment.
2. **Compile to CatWeb JSON**:
   - Run the compiler CLI binary `lync`:
     ```bash
     lync -c source.lxs -o output.json
     ```
   - Verify that output matches the expected CatWeb block schema formats.
3. **Debug compilation issues**:
   - Inspect the compiled JSON structure for name-mangling correctness.
   - Trace shadowing scopes to ensure variables are suffixed with `__s{scope_id}`.
