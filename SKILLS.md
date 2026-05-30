# LynxScript AI Agent Skill Integration (SKILLS.md)

This document provides instructions for AI agents on how to parse, write, modify, compile, and debug **LynxScript (LXS)** code and compiler pipelines.

---

## 1. Syntax Specifications

LynxScript is a simplified JavaScript-like language with unique annotations and block-mapping mechanics.

### Lexer Rules
- **Identifiers**: `[a-zA-Z_][a-zA-Z0-9_]*`
- **Keywords**: `link`, `let`, `function`, `true`, `false`
- **Integers / Floats**: `\d+`, `\d+\.\d+`
- **String Literals**: `"..."`
- **Raw String Literals**: `#"..."` (Outer quotes stripped by the parser, returning raw payload).
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

---

## 2. Abstract Syntax Tree (AST)

When parsing, rules are mapped as follows:
- **`Program`**: Houses `link_statements` and `main_block` items list.
- **`Item`**: Either an `AttributeItem` or `FunctionDeclarationItem`.
- **`Statement`**:
  - `ExpressionStatement`: A single expression ending in a semicolon.
  - `AssignmentStatement`: A variable declaration or variable assignment (`lhs = rhs;`).
- **`Expression`**:
  - `LiteralExpr`: Integers, floats, booleans, string literals, and raw string literals.
  - `IdentifierExpr`: Scope-resolved variable or function names.
  - `CallExpr`: Normal function calls `func(args)` or raw block calls `#0(args)`.
  - `BinOperationExpr` / `UnaryOperationExpr`: Arithmetic, logical operations, and comma lists.

---

## 3. Compiler & Name Mangling

The LynxScript compiler lowers AST structures into intermediate code blocks:
- **Function registration prepass**: All function declarations are registered in the global symbol table. If the function has `#[export_as("name")]`, the compiler registers it under the alias.
- **Name Mangling (Shadowing)**: If a variable name is declared in a nested scope shadowing a parent variable, the compiler mangles the nested name into `{original_name}__s{scope_id}`.
- **Operator mapping**: Standard operators compile to standard functions:
  - `+` -> `add`
  - `-` -> `sub`
  - `*` -> `mul`
  - `/` -> `div`
  - `**` -> `pow`

---

## 4. Verification and Packaging

To compile a `.lxs` file to CatWeb JSON:
```bash
lync -c source.lxs -o output.json
```
