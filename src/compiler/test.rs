#[cfg(test)]

mod compiler_tests {
  use insta::assert_debug_snapshot;
  use crate::{parser::Parser, compiler::Compiler, compiler::symbol_table};

  #[test]
  fn test_simple_function_compiling() {
    let input = r#"
    #[export_as("c")]
    function add(a, b) {
      #0(#"", "Hello, World!");
    }"#;

    let syntax_tree = Parser.parse_program_from_str(input).unwrap();

    let mut compiler = Compiler::new(syntax_tree);
    let structure_res = compiler.compile();

    assert_debug_snapshot!(structure_res);
  }

  #[test]
  fn test_symbol_table_shadowing_mangles() {
    use symbol_table::{SymbolTable, SymbolType};

    let mut table = SymbolTable::new();
    let root_scope = table.root_scope();
    let outer = table
      .add_symbol(root_scope, "x", SymbolType::Variable)
      .expect("Outer symbol should be added");

    let child_scope = table.enter_scope(root_scope);
    let inner = table
      .add_symbol(child_scope, "x", SymbolType::Variable)
      .expect("Inner symbol should be added");

    assert_eq!(outer.original_name, "x");
    assert_eq!(outer.unique_name, "x");
    assert_ne!(inner.unique_name, "x");

    let resolved = table
      .resolve(child_scope, "x")
      .expect("Inner shadowed symbol should resolve");
    assert_eq!(resolved.unique_name, inner.unique_name);
  }

  #[test]
  fn test_symbol_table_duplicate_in_scope_errors() {
    use symbol_table::{SymbolTable, SymbolType};

    let mut table = SymbolTable::new();
    let root_scope = table.root_scope();
    table
      .add_symbol(root_scope, "x", SymbolType::Variable)
      .expect("First symbol should be added");

    let duplicate = table.add_symbol(root_scope, "x", SymbolType::Variable);
    assert!(duplicate.is_err());
  }
}