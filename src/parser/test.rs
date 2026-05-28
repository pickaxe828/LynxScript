#[cfg(test)]

mod parser_tests {
  use insta::assert_debug_snapshot;
  use crate::parser::Parser;

  #[test]
  fn test_simple_function_parsing() {
    let input = r#"
    #[export_as("c")]
    function add(a, b) {
      #0(#"", "Hello, World!");
    }"#;

    let syntax_tree = Parser.parse_program_from_str(input).unwrap();

    assert_debug_snapshot!(syntax_tree);
  }
}