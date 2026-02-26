#[test]
fn test_simple_function_parsing() {
  use crate::parser::{self};

  let input = r#"
  #[export_as("c")]
  function add(a, b) {
    #0(#"", "Hello, World!");
  }"#;

  let syntax_tree = super::Parser.parse_program_from_str(input).unwrap();

  dbg!(syntax_tree.clone());

  let expected_syntax_tree = parser::Program {
    link_statements: vec![],
    main_block: vec![
      parser::Item::Attribute(
        parser::Attribute::ExportAs("add".to_string())
      ),
      parser::Item::FunctionDeclaration(
        parser::FunctionDeclaration {         
          name: "add".to_string(),
          parameters: vec![
            parser::Expression::Identifier("a".to_string()),
            parser::Expression::Identifier("b".to_string()),
          ],
          body: vec![
            parser::Statement::Expression {
              expr: parser::Expression::Call {
                function: Box::new(parser::Expression::CWScriptBlockID("0".to_string())),
                arguments: vec![
                  parser::Expression::Literal(parser::Literal::RawString("".to_string())),
                  parser::Expression::Literal(parser::Literal::String("Hello, World!".to_string())),
                ]
              }
            }
          ] 
        }
      )
    ]
  };

  assert_eq!(expected_syntax_tree, syntax_tree);
}